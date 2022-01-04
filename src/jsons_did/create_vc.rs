
use serde::Serialize;
use serde_derive::Deserialize;

use super::{holder_data::HolderData, create_did_vm::CreateDidVm};

#[derive(Serialize, Deserialize,Debug)]
pub struct CreateVc { 
    pub issuer: CreateDidVm,
    pub holder: HolderData,
}
   