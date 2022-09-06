use identity_iota::credential::{CredentialBuilder, Presentation, PresentationBuilder};
use identity_iota::did::DID;
use identity_iota::iota_core::IotaDID;
use identity_iota::prelude::*;
use identity_iota::{
    core::{FromJson, Url},
    credential::{Credential, Subject},
};
use serde_json::json;

use crate::models::models::HolderData;
/// Helper that takes two DID Documents (identities) for issuer and subject, and
/// creates an unsigned credential with claims about subject by issuer.
pub fn issuer_degree(issuer: &IotaDID, data_holder: &HolderData) -> Result<Credential> {
    // Create VC "subject" field containing subject ID and claims about it.

    let subject: Subject = Subject::from_json_value(json!(data_holder.to_owned())).unwrap();

    // Build credential using subject above and issuer.
    let credential: Credential = CredentialBuilder::default()
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
        .holder(Url::parse(holder.as_str()).unwrap())
        .credential(credential)
        .build()
        .unwrap();

    Ok(presentation)
}