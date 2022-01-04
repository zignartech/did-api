use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Serialize, Deserialize,Debug)]
pub struct CreateDidVm { 
    pub nick_name: String,
    pub password: String,
    pub did: Option<String>,
    pub vm_name: Option<String>,
}