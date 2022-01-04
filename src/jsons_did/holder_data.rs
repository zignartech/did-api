use serde_derive::Deserialize;
use serde_derive::Serialize;


#[derive(Serialize, Deserialize,Debug)]
pub struct HolderData { 
    pub id: String,
    pub name: String,
    pub degreeName: String,
    pub degreeType: String,
    pub GPA: String,
}