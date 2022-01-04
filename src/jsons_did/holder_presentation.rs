use serde::Serialize;
use serde_derive::Deserialize;

use super::{holder_credential::HolderCredential};

#[derive(Serialize, Deserialize,Debug)]
pub struct HolderPresentation { 
    #[serde(rename = "@context")]
    pub context: String,
    pub verifiableCredential: HolderCredential,
    pub holder: String,
    pub proof: HolderProof,
    pub r#type: String
}

#[derive(Serialize, Deserialize,Debug)]
pub struct HolderProof { 
    signatureValue: String,
    r#type: String,
    verificationMethod: String
}