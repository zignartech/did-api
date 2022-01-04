use std::path::PathBuf;

use crate::utils_did::rnd_vm_name::randomVM;
use identity::account::{IdentityCreate, IdentityState};
use identity::prelude::*;
use identity::{
    account::{Account, AccountStorage},
    credential::{Credential, Presentation},
    iota::IotaDID,
    prelude::IotaDocument,
};

#[derive(Debug)]
pub struct User {
    account: Account,
}

impl User {
    #[tokio::main]
    pub async fn new(
        nick_name: String,
        password: String,
    ) -> Result<User> {
        // Stronghold settings
        let stronghold_path: PathBuf =
            format!("./accounts_stronghold/{}.stronghold", nick_name).into();

        // let password_json = &password;
        let password: String = Into::into(password);

        let account: Account = Account::builder()
            .storage(AccountStorage::Stronghold(stronghold_path, Some(password)))
            .build()
            .await
            .unwrap();

        Ok(Self { account: account })
    }

    #[tokio::main]
    pub async fn create_identity(&self) -> Result<IdentityState> {
        Ok(self
            .account
            .create_identity(IdentityCreate::default())
            .await
            .unwrap())
    }

    #[tokio::main]
    pub async fn create_method(&mut self, iota_did: &IotaDID) -> Result<String> {
        let vm_name: String = randomVM();
        self.account
            .update_identity(&iota_did)
            .create_method()
            .fragment(&vm_name)
            .apply()
            .await
            .unwrap();
        Ok(vm_name)
    }

    #[tokio::main]
    pub async fn delete_method(&mut self, iota_did: &IotaDID, vm_name: String) -> () {
        self.account
            .update_identity(&iota_did)
            .delete_method()
            .fragment(&vm_name)
            .apply()
            .await
            .unwrap();
    }

    #[tokio::main]
    pub async fn sign_c(&self, key: &IotaDID, vm_name: &str, credential: &mut Credential) -> () {
        self.account.sign(&key, vm_name, credential).await.unwrap()
    }

    #[tokio::main]
    pub async fn sign_p(&self, key: &IotaDID, vm_name: &str, presentation: &mut Presentation) -> () {
        self.account
            .sign(&key, vm_name, presentation)
            .await
            .unwrap()
    }

    #[tokio::main]
    pub async fn resolve_identity(&self, iota_did: &IotaDID) -> Result<IotaDocument> {
        Ok(self.account.resolve_identity(&iota_did).await.unwrap())
    }
}
