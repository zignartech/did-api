use std::str::FromStr;

use actix_web::{post, web, HttpResponse};
use identity::{
    core::{FromJson, ToJson},
    credential::{Credential, Presentation},
    iota::{ClientMap, IotaDID},
    prelude::IotaDocument,
};
use serde_json::{json, Value};
use crate::utils_did::validator::CredentialValidation;
use crate::utils_did::validator::PresentationValidation;
use crate::{
    jsons_did::{
        create_did_vm::CreateDidVm, create_vc::CreateVc, create_vp::CreateVp,
        holder_credential::HolderCredential, holder_presentation::HolderPresentation,
        remove_vm::RemoveVm,
    },
    utils_did::{common, user::User},
};

#[post("/create_did")]
async fn createDidApi(data_json: web::Json<CreateDidVm>) -> HttpResponse {
    let data = data_json.into_inner();

    let account = User::new(data.nick_name.to_string(), data.password.to_string())
        .expect("Error Account");

    let identity = account.create_identity().unwrap();
    let iota_did: &IotaDID = identity.try_did().unwrap();

    let explorer = iota_did
        .network()
        .unwrap()
        .explorer_url()
        .unwrap()
        .to_string();
    println!(
        "[Example] Explore the DID Document = {}/{}",
        iota_did
            .network()
            .unwrap()
            .explorer_url()
            .unwrap()
            .to_string(),
        iota_did.to_string()
    );

    let explorer = format!("{}/{}", explorer, iota_did.to_string());

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
          "Error": false,
          "Status": "Did Created",
          "did": iota_did,
          "Explorer": explorer,
        }))
}

#[post("/add_verif_method")]
async fn addVerifMethodApi(data_json: web::Json<CreateDidVm>) -> HttpResponse {
    let data = data_json.into_inner();
    let iota_did = IotaDID::from_str(&data.did.unwrap()).unwrap();

    let mut account = User::new(
        data.nick_name.to_string(),
        data.password.to_string(),
    )
    .expect("Error Account");

    let vm_randon = account.create_method(&iota_did).unwrap();

    let explorer = iota_did
        .network()
        .unwrap()
        .explorer_url()
        .unwrap()
        .to_string();
    println!(
        "[Example] Explore the DID Document = {}/{}",
        iota_did
            .network()
            .unwrap()
            .explorer_url()
            .unwrap()
            .to_string(),
        iota_did.to_string()
    );

    let explorer = format!("{}/{}", explorer, iota_did.to_string());

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
          "Error": false,
          "Status": "Method Created",
          "Explorer": explorer,
          "vm_name": vm_randon
        }))
}

#[post("/create_vc")]
async fn createVcApi(data_json: web::Json<CreateVc>) -> HttpResponse {
    let data = data_json.into_inner();
    let data_issuer = data.issuer;
    let iota_did = IotaDID::from_str(&data_issuer.did.unwrap()).unwrap();

    let account = User::new(
        data_issuer.nick_name.to_string(),
        data_issuer.password.to_string()
    )
    .expect("Error Account");

    let data_holder = data.holder;

    let mut credential: Credential =
        common::issue_degree(&iota_did, &data_holder).unwrap();

    account.sign_c(&iota_did, &data_issuer.vm_name.unwrap(), &mut credential);

    // println!("credential: {:#}", credential);

    let resolved: IotaDocument = account.resolve_identity(&iota_did).unwrap();

    let verified: bool = resolved.verify_data(&credential).is_ok();

    if verified == false {
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
              "Error": true,
              "Verified": verified,
            }));
    }

    let credential_str = credential.clone().to_json().unwrap();
    let credential_json: Value = serde_json::from_str(&credential_str).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
          "Error": false,
          "Status": "Credential Created",
          "Verified": verified,
          "Credential": credential_json
        }))
}

#[post("/verify_validity_credential")]
pub async fn verifyValidityApiCred(data_json: web::Json<HolderCredential>) -> HttpResponse {
    let client: ClientMap = ClientMap::new();

    let validation: CredentialValidation = common::check_credential(&client, data_json).unwrap();

    if validation.verified == false {
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
              "Error": true,
              "validation": validation.verified,
            }));
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
          "error": false,
          "validation": validation.verified,
        }))
}

#[post("/create_vp")]
pub async fn createVpApi(data_json: web::Json<CreateVp>) -> HttpResponse {
    let data = data_json.into_inner();
    let data_holder = data.holder;
    let iota_did = IotaDID::from_str(&data_holder.did.unwrap()).unwrap();

    let account = User::new(
        data_holder.nick_name.to_string(),
        data_holder.password.to_string(),
    )
    .expect("Error Account");

    let data_holder_c = serde_json::to_string(&data.holder_credential).unwrap();

    let credential: Credential = Credential::from_json(&data_holder_c).unwrap();

    // println!("Credential Validation > {:#?}", validation);

    let mut presentation: Presentation =
        common::holder_presentation(&iota_did, credential).unwrap();

    account.sign_p(&iota_did, &data_holder.vm_name.unwrap(), &mut presentation);

    // println!("presentation: {:#}", presentation);

    let resolved: IotaDocument = account.resolve_identity(&iota_did).unwrap();

    let verified: bool = resolved.verify_data(&presentation).is_ok();

    if verified == false {
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
              "Error": true,
              "Verified": verified,
            }));
    }

    let presentation_str = presentation.clone().to_json().unwrap();
    let presentation_json: Value = serde_json::from_str(&presentation_str).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
          "Error": false,
          "Status": "Presentation Created",
          "verified": verified,
          "Presentation": presentation_json
        }))
}

#[post("/verify_validity_presentation")]
pub async fn verifyValidityApiPres(data_json: web::Json<HolderPresentation>) -> HttpResponse {
    let client: ClientMap = ClientMap::new();

    let validation: PresentationValidation =
        common::check_presentation(&client, data_json).unwrap();

    if validation.verified == false {
        return HttpResponse::Ok()
            .content_type("application/json")
            .json(json!({
              "Error": true,
              "validation": validation.verified,
            }));
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
          "error": false,
          "validation": validation.verified,
        }))
}

#[post("/remove_vm")]
async fn removeVmApi(data_json: web::Json<RemoveVm>) -> HttpResponse {
    let data = data_json.into_inner();
    let data_issuer = data.issuer;
    let iota_did = IotaDID::from_str(&data_issuer.did.unwrap()).unwrap();

    let mut account = User::new(
        data_issuer.nick_name.to_string(),
        data_issuer.password.to_string(),
    )
    .expect("Error Account");

    account.delete_method(&iota_did, data_issuer.vm_name.unwrap());

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!({
          "error": false,
          "Status": "Removed VM"
        }))
}
