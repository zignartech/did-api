use std::path::PathBuf;

use identity_iota::prelude::*;
use identity_iota::{
    account::{Account, AccountBuilder, AutoSave, IdentitySetup, MethodContent},
    account_storage::Stronghold,
    client::ClientBuilder,
    core::Timestamp,
    credential::{Credential, Presentation},
    crypto::ProofOptions,
    iota_core::IotaDID,
};

use crate::utils_did::rnd_vm_name::randomVM;

#[derive(Debug)]
pub struct IssuerBuilder {
    accountBuilder: AccountBuilder,
}

impl IssuerBuilder {
    pub async fn new(nick_name: String, password: String) -> Result<Self> {
        // Stronghold settings
        let stronghold_path: PathBuf =
            format!("./accounts_stronghold/{}.stronghold", nick_name).into();

        // let password_json = &password;
        let password: String = Into::into(password);

        // Primay Node Url
        let private_node = &std::env::var("PRIMARY_NODE_HORNET")
            .unwrap_or("https://api.lb-0.h.chrysalis-devnet.iota.cafe".to_string());

        // Primay Node Url
        let private_pow_node = &std::env::var("PRIMARY_POW_NODE_HORNET")
            .unwrap_or("https://api.lb-1.h.chrysalis-devnet.iota.cafe".to_string());

        let stronghold: Stronghold = Stronghold::new(&stronghold_path, password, Some(true))
            .await
            .unwrap();

        let accountBuilder: AccountBuilder = Account::builder()
            .autosave(AutoSave::Every)
            .storage(stronghold)
            .client_builder(
                ClientBuilder::new()
                    .primary_node(private_node, None, None)
                    .unwrap()
                    .primary_pow_node(private_pow_node, None, None)
                    .unwrap()
                    .local_pow(false),
                // .permanode(<permanode_url>, None, None)?
            );

        Ok(Self { accountBuilder })
    }

    pub async fn create_identity(&mut self) -> Result<String> {
        let account = self
            .accountBuilder
            .create_identity(IdentitySetup::default())
            .await
            .unwrap();
        Ok(account.did().to_string())
    }
}

#[derive(Debug)]
pub struct Issuer {
    account: Account,
}

impl Issuer {
    pub async fn load_account(
        nick_name: String,
        password: String,
        iota_did: IotaDID,
    ) -> Result<Self, identity_iota::account::Error> {
        let mut builder = IssuerBuilder::new(nick_name, password).await.unwrap();
        let account = builder
            .accountBuilder
            .load_identity(iota_did)
            .await
            .unwrap();
        Ok(Self { account })
    }

    pub async fn create_method(&mut self) -> Result<String> {
        let vm_name: String = randomVM();
        self.account
            .update_identity()
            .create_method()
            .content(MethodContent::GenerateEd25519)
            .fragment(&vm_name)
            .apply()
            .await
            .unwrap();
        Ok(vm_name)
    }

    pub async fn delete_method(&mut self, vm_name: String) -> () {
        self.account
            .update_identity()
            .delete_method()
            .fragment(&vm_name)
            .apply()
            .await
            .unwrap();
    }

    pub async fn sign_c(
        &self,
        fragment: &str,
        credential: &mut Credential,
        expires: &str,
        challenge: String,
    ) -> () {
        let expires: Timestamp = Timestamp::parse(expires).unwrap();
        self.account
            .sign(
                &fragment,
                credential,
                ProofOptions::new()
                    .challenge(challenge.to_string())
                    .expires(expires),
            )
            .await
            .unwrap()
    }

    pub async fn sign_p(
        &self,
        fragment: &str,
        presentation: &mut Presentation,
        expires: &str,
        challenge: String,
    ) -> () {
        let expires: Timestamp = Timestamp::parse(expires).unwrap();
        self.account
            .sign(
                &fragment,
                presentation,
                ProofOptions::new()
                    .challenge(challenge.to_string())
                    .expires(expires),
            )
            .await
            .unwrap()
    }
}
