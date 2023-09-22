use candid::{CandidType, Principal};
use ic_canister::{init, update, Canister, MethodType, PreUpdate};

use crate::state::TradeState;

#[derive(Canister, Clone, Debug)]
pub struct TradeCanister {
    #[id]
    principal: Principal,
    state: TradeState,
}

impl PreUpdate for TradeCanister {
    fn pre_update(&self, _method_name: &str, _method_type: MethodType) {}
}

#[derive(CandidType)]
pub struct InitData {
    pub owner: Principal,
}

impl TradeCanister {
    #[init]
    pub fn init(&mut self, init: InitData) {
        self.state = TradeState::default();
    }

    #[update]
    pub fn create_new_trade(&mut self, name: String, description: String, price: u64) {}

    #[update]
    pub fn get_trade(&self) -> TradeState {
        panic!("Not implemented")
    }

    #[update]
    pub fn update_trade(&mut self, name: String, description: String, price: u64) {}

    #[update]
    pub fn delete_trade(&mut self) {}
}
