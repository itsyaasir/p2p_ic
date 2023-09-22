use ic_canister::{generate_idl, Idl};
mod canister;
mod state;

pub fn idl() -> String {
    use crate::canister::InitData;
    use crate::state::TradeState;

    let trade_canister_idl = generate_idl!();

    candid::bindings::candid::compile(&trade_canister_idl.env.env, &Some(trade_canister_idl.actor))
}
