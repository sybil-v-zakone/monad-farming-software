use crate::Result;
use alloy::{primitives::Address, signers::local::PrivateKeySigner};
use common::state::{Dex, Lending};
use derive_builder::Builder;
use rand::seq::IndexedRandom;
use rquest::Impersonate;
use sea_orm::Set;
use std::str::FromStr;

use super::prelude::{AccountActiveModel, AccountModel};

#[derive(Debug, Default)]
pub struct AccountConditions {
    pub goal_reached: Option<bool>,
    pub id: Option<i32>,
}

#[derive(Builder)]
pub struct NewActiveModelOptions {
    pk: String,
    proxy: Option<String>,
    address: String,
    target_ambient_swaps_count: u32,
    target_apriori_deposit_count: u32,
    target_bean_swaps_count: u32,
    target_hashflow_swaps_count: u32,
    target_kinza_deposit_count: u32,
    target_shmonad_deposit_count: u32,
}

impl AccountActiveModel {
    pub fn new(opts: NewActiveModelOptions) -> Self {
        Self {
            address: Set(opts.address),
            private_key: Set(opts.pk),
            proxy: Set(opts.proxy),
            target_ambient_swaps_count: Set(opts.target_ambient_swaps_count as i32),
            target_apriori_deposit_count: Set(opts.target_apriori_deposit_count as i32),
            target_bean_swaps_count: Set(opts.target_bean_swaps_count as i32),
            target_hashflow_swaps_count: Set(opts.target_hashflow_swaps_count as i32),
            target_kinza_deposit_count: Set(opts.target_kinza_deposit_count as i32),
            target_shmonad_deposit_count: Set(opts.target_shmonad_deposit_count as i32),
            ..Default::default()
        }
    }
}

impl AccountModel {
    fn available_actions(&self) -> Vec<AccountAction> {
        let mut actions = vec![];

        // swap protocols
        if self.target_ambient_swaps_count > self.current_ambient_swaps_count {
            actions.push(AccountAction::Swap(Dex::Ambient));
        }

        if self.target_bean_swaps_count > self.current_bean_swaps_count {
            actions.push(AccountAction::Swap(Dex::Bean));
        }

        if self.target_hashflow_swaps_count > self.current_hashflow_swaps_count {
            actions.push(AccountAction::Swap(Dex::Hashflow));
        }

        // lending protocols
        if self.target_apriori_deposit_count > self.current_apriori_deposit_count {
            actions.push(AccountAction::Lending(Lending::Apriori));
        }

        if self.target_kinza_deposit_count > self.current_kinza_deposit_count {
            actions.push(AccountAction::Lending(Lending::Kinza));
        }
        if self.target_shmonad_deposit_count > self.current_shmonad_deposit_count {
            actions.push(AccountAction::Lending(Lending::Shmonad));
        }

        actions
    }

    pub fn random_available_action(&self) -> Option<AccountAction> {
        let actions = self.available_actions();

        actions.choose(&mut rand::rng()).copied()
    }

    pub fn signer(&self) -> PrivateKeySigner {
        PrivateKeySigner::from_str(&self.private_key).unwrap() // private keys must be checked during db gen
    }

    pub fn address(&self) -> Address {
        self.signer().address()
    }

    fn proxy(&self) -> Option<rquest::Proxy> {
        self.proxy.as_ref().and_then(|p| rquest::Proxy::all(p).ok())
    }

    pub fn http_client(&self) -> Result<rquest::Client> {
        let mut builder = rquest::Client::builder();

        if let Some(proxy) = self.proxy() {
            builder = builder.proxy(proxy)
        }

        let client = builder.impersonate(Impersonate::Chrome133).build()?;
        Ok(client)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AccountAction {
    Swap(Dex),
    Lending(Lending),
}
