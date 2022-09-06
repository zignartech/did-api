use actix_web::{post, web, HttpResponse};
use identity_iota::{
    client::{
        CredentialValidationOptions, CredentialValidator, FailFast, PresentationValidationOptions,
        ResolvedIotaDocument, Resolver, StatusCheck, SubjectHolderRelationship,
    },
    core::{Duration, FromJson, Timestamp, ToJson},
    credential::{Credential, Presentation},
    did::verifiable::VerifierOptions,
    iota_core::IotaDID,
};
use serde_json::{json, Value};
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    models::models::{
        CreateDidVm, CreateVc, CreateVp, HolderCredential, HolderPresentation, RemoveVm,
    },
    utils_did::{
        common,
        issuer::{Issuer, IssuerBuilder},
    },
};

#[post("/create_did")]
async fn createDidApi(data_json: web::Json<CreateDidVm>) -> HttpResponse {
    let data = data_json.into_inner();

    let mut accountBuilder =
        IssuerBuilder::new(data.nick_name.to_string(), data.password.to_string())
            .await
            .expect("Error Account");

    let iota_did = accountBuilder.create_identity().await.unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
          "Error": false,
          "Status": "Did Created",
          "did": iota_did,
        }))
}

#[post("/add_verif_method")]
async fn addVerifMethodApi(data_json: web::Json<CreateDidVm>) -> HttpResponse {
    let data = data_json.into_inner();
    let iota_did = IotaDID::from_str(&data.did.unwrap()).unwrap();
    let mut account = match Issuer::load_account(
        data.nick_name.to_string(),
        data.password.to_string(),
        iota_did,
    )
    .await
    {
        Ok(v) => v,
        Err(_e) => {
            return HttpResponse::NotFound().json(json!({
                "error": true,
                "message": "DID Not Found"
            }))
        }
    };
    let vm_randon = account.create_method().await.unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
          "Error": false,
          "Status": "Method Created",
          "vm_name": vm_randon
        }))
}

#[post("/create_vc")]
async fn createVcApi(data_json: web::Json<CreateVc>) -> HttpResponse {
    let data = data_json.into_inner();
    let data_issuer = data.issuer;
    let iota_did = IotaDID::from_str(&data_issuer.did.unwrap()).unwrap();

    let account = match Issuer::load_account(
        data_issuer.nick_name.to_string(),
        data_issuer.password.to_string(),
        iota_did.clone(),
    )
    .await
    {
        Ok(v) => v,
        Err(_e) => {
            return HttpResponse::NotFound().json(json!({
                "error": true,
                "message": "DID Not Found"
            }))
        }
    };

    let data_holder = data.holder;

    let mut credential: Credential = common::issuer_degree(&iota_did, &data_holder).unwrap();
    let challenge = Uuid::new_v4();
    account
        .sign_c(
            &data_issuer.vm_name.unwrap(),
            &mut credential,
            &data.expires,
            challenge.to_string(),
        )
        .await;

    let credential_str = credential.clone().to_json().unwrap();
    let credential_json: Value = serde_json::from_str(&credential_str).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
          "Error": false,
          "Status": "Credential Created",
          "Credential": credential_json,
          "challenge": challenge.to_string(),
        }))
}

#[post("/verify_validity_credential")]
pub async fn verifyValidityApiCred(data_json: web::Json<HolderCredential>) -> HttpResponse {
    let data = data_json.into_inner();
    let credential: Credential =
        Credential::from_json(&serde_json::to_string(&data.holderCredential).unwrap()).unwrap();

    let credential_verifier_options: VerifierOptions = VerifierOptions::new()
        .challenge(data.challenge)
        .allow_expired(false);

    let credential_validation_options = CredentialValidationOptions::default()
        .verifier_options(credential_verifier_options)
        .status_check(StatusCheck::Strict);

    let resolver: Resolver = Resolver::new().await.unwrap();
    let resolved_issuer_doc: ResolvedIotaDocument =
        match resolver.resolve_credential_issuer(&credential).await {
            Ok(doc) => doc,
            Err(_e) => {
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .json(json!({
                      "error": true,
                      "status": "Invalid document".to_string(),
                    }))
            }
        };

    match CredentialValidator::validate(
        &credential,
        &resolved_issuer_doc,
        &credential_validation_options,
        identity_iota::client::FailFast::FirstError,
    ) {
        Ok(_) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                  "error": false,
                  "status": "verified".to_string(),
                }))
        }
        Err(_e) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                  "error": true,
                  "status": "Invalid credential".to_string(),
                }))
        }
    }
}

#[post("/create_vp")]
pub async fn createVpApi(data_json: web::Json<CreateVp>) -> HttpResponse {
    let data = data_json.into_inner();
    let data_holder = data.holder;
    let iota_did = IotaDID::from_str(&data_holder.did.unwrap()).unwrap();

    let account = match Issuer::load_account(
        data_holder.nick_name.to_string(),
        data_holder.password.to_string(),
        iota_did.clone(),
    )
    .await
    {
        Ok(v) => v,
        Err(_e) => {
            return HttpResponse::NotFound().json(json!({
                "error": true,
                "message": "DID Not Found"
            }))
        }
    };

    let data_holder_c = serde_json::to_string(&data.holder_credential).unwrap();

    let credential: Credential = Credential::from_json(&data_holder_c).unwrap();

    let mut presentation: Presentation =
        common::holder_presentation(&iota_did, credential).unwrap();
    let challenge = uuid::Uuid::new_v4();
    account
        .sign_p(
            &data_holder.vm_name.unwrap(),
            &mut presentation,
            &data.expires,
            challenge.to_string(),
        )
        .await;

    let presentation_str = presentation.clone().to_json().unwrap();
    let presentation_json: Value = serde_json::from_str(&presentation_str).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
          "Error": false,
          "Status": "Presentation Created",
          "Presentation": presentation_json,
          "challenge": challenge.to_string(),
        }))
}

#[post("/verify_validity_presentation")]
pub async fn verifyValidityApiPres(data_json: web::Json<HolderPresentation>) -> HttpResponse {
    let data = data_json.into_inner();
    let presentation: Presentation =
        Presentation::from_json(&serde_json::to_string(&data.holderPresentation).unwrap()).unwrap();

    let presentation_verifier_options: VerifierOptions = VerifierOptions::new()
        .challenge(data.challenge)
        .allow_expired(false);

    // Do not allow credentials that expire within the next 10 hours.
    let credential_validation_options: CredentialValidationOptions =
        CredentialValidationOptions::default().earliest_expiry_date(
            Timestamp::now_utc()
                .checked_add(Duration::hours(10))
                .unwrap(),
        );

    let presentation_validation_options = PresentationValidationOptions::default()
        .presentation_verifier_options(presentation_verifier_options.clone())
        .shared_validation_options(credential_validation_options)
        .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject);

    // Validate the presentation and all the credentials included in it.
    let resolver: Resolver = Resolver::new().await.unwrap();
    match resolver
        .verify_presentation(
            &presentation,
            &presentation_validation_options,
            FailFast::FirstError,
            None,
            None,
        )
        .await
    {
        Ok(_) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                  "error": false,
                  "status": "verified",
                }))
        }
        Err(_e) => {
            return HttpResponse::Ok()
                .content_type("application/json")
                .json(json!({
                  "error": true,
                  "status": "Invalid presentation".to_string(),
                }))
        }
    }
}

#[post("/remove_vm")]
async fn removeVmApi(data_json: web::Json<RemoveVm>) -> HttpResponse {
    let data = data_json.into_inner();
    let data_issuer = data.issuer;
    let iota_did = IotaDID::from_str(&data_issuer.did.unwrap()).unwrap();

    let mut account = match Issuer::load_account(
        data_issuer.nick_name.to_string(),
        data_issuer.password.to_string(),
        iota_did,
    )
    .await
    {
        Ok(v) => v,
        Err(_e) => {
            return HttpResponse::NotFound().json(json!({
                "error": true,
                "message": "DID Not Found"
            }))
        }
    };

    account.delete_method(data_issuer.vm_name.unwrap()).await;

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
          "error": false,
          "Status": "Removed VM"
        }))
}
