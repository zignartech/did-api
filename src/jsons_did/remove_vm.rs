
use serde::Serialize;
use serde_derive::Deserialize;

use super::{create_did_vm::CreateDidVm};

#[derive(Serialize, Deserialize,Debug)]
pub struct RemoveVm { 
    pub issuer: CreateDidVm,
}
   