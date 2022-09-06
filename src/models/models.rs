use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize,Debug)]
pub struct CreateDidVm { 
    pub nick_name: String,
    pub password: String,
    pub did: Option<String>,
    pub vm_name: Option<String>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct CreateVc { 
    pub issuer: CreateDidVm,
    pub holder: HolderData,
    pub expires: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct CreateVp { 
    pub holder: CreateDidVm,
    pub holder_credential: Value,
    pub expires: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct HolderCredential { 
    pub holderCredential: Value,
    pub challenge: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct HolderData { 
    pub id: String,
    pub name: String,
    pub degreeName: String,
    pub degreeType: String,
    pub GPA: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct HolderPresentation { 
    pub holderPresentation: Value,
    pub challenge: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct RemoveVm { 
    pub issuer: CreateDidVm,
}