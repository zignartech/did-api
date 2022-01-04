use serde::Serialize;
use serde_derive::Deserialize;

use super::{holder_data::HolderData};

#[derive(Serialize, Deserialize,Debug)]
pub struct HolderCredential { 
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    pub credentialSubject: HolderData,
    pub issuanceDate: String,
    pub issuer: String,
    pub proof: HolderProof,
    pub r#type: Vec<String>
}

#[derive(Serialize, Deserialize,Debug)]
pub struct HolderProof { 
    signatureValue: String,
    r#type: String,
    verificationMethod: String
}