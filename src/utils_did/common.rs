use crate::jsons_did::holder_credential::HolderCredential;
use crate::jsons_did::holder_data::HolderData;
use crate::jsons_did::holder_presentation::HolderPresentation;
use actix_web::web;
use identity::credential::{Presentation, PresentationBuilder};
// use identity::iota::{CredentialValidator, PresentationValidation};
use crate::utils_did::validator::CredentialValidator;
use crate::utils_did::validator::PresentationValidation;
use crate::utils_did::validator::CredentialValidation;
use identity::prelude::*;
use identity::{
    core::{FromJson, Url},
    credential::{Credential, Subject},
    did::DID,
    iota::{ClientMap, IotaDID},
};
use serde_json::json;
/// Helper that takes two DID Documents (identities) for issuer and subject, and
/// creates an unsigned credential with claims about subject by issuer.
pub fn issue_degree(issuer: &IotaDID, data_holder: &HolderData) -> Result<Credential> {
    // Create VC "subject" field containing subject ID and claims about it.

    let subject: Subject = Subject::from_json_value(json!(data_holder.to_owned())).unwrap();

    // Build credential using subject above and issuer.
    let credential: Credential = Credential::builder(Default::default())
        .id(Url::parse("https://example.edu/credentials/3732").unwrap())
        .issuer(Url::parse(issuer.as_str()).unwrap())
        .type_("UniversityDegreeCredential")
        .subject(subject)
        .build()
        .unwrap();

    Ok(credential)
}

pub fn holder_presentation(holder: &IotaDID, credential: Credential) -> Result<Presentation> {
    // Create VC "subject" field containing subject ID and claims about it.

    // Create an unsigned Presentation from the previously issued Verifiable Credential.
    let presentation: Presentation = PresentationBuilder::default()
        // .id(Url::parse("asdf:foo:a87w3guasbdfuasbdfs").unwrap())
        .holder(Url::parse(holder.as_str()).unwrap())
        .credential(credential)
        .build()
        .unwrap();

    Ok(presentation)
}

/// Convenience function for checking that a verifiable credential is valid and not revoked.
#[tokio::main]
pub async fn check_credential(
    client: &ClientMap,
    credential: web::Json<HolderCredential>,
) -> Result<CredentialValidation> {
    // Convert the Verifiable Credential to JSON to potentially "exchange" with a verifier
    let credential_json = serde_json::to_string(&credential.into_inner()).unwrap();

    // Create a `CredentialValidator` instance to fetch and validate all
    // associated DID Documents from the Tangle.
    let validator: CredentialValidator<ClientMap> = CredentialValidator::new(client);

    // Perform the validation operation.
    let validation: CredentialValidation = validator.check(&credential_json).await.unwrap();
    Ok(validation)
}

#[tokio::main]
pub async fn check_presentation(
    client: &ClientMap,
    presentation: web::Json<HolderPresentation>,
) -> Result<PresentationValidation> {
    // Convert the Verifiable Credential to JSON to potentially "exchange" with a verifier
    let presentation_json = serde_json::to_string(&presentation.into_inner()).unwrap();

    // let presentation: Presentation = Presentation::from_json(&data_holder_c).unwrap();

    // Create a `CredentialValidator` instance to fetch and validate all
    // associated DID Documents from the Tangle.
    let validator: CredentialValidator<ClientMap> = CredentialValidator::new(&client);

    // Perform the validation operation.
    let validation: PresentationValidation = validator
        .check_presentation(&presentation_json)
        .await
        .unwrap();
    // println!("validation = {:#?}", validation);
    Ok(validation)
}
