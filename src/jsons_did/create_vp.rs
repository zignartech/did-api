
use serde::Serialize;
use serde_derive::Deserialize;

use super::{create_did_vm::CreateDidVm, holder_credential::HolderCredential};

#[derive(Serialize, Deserialize,Debug)]
pub struct CreateVp { 
    pub holder: CreateDidVm,
    pub holder_credential: HolderCredential,
}
   