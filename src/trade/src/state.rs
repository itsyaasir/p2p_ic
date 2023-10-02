use std::borrow::Cow;
use std::cell::RefCell;

use candid::{CandidType, Principal};

use ic_stable_structures::{
    BoundedStorable, MemoryId, SlicedStorable, StableUnboundedMap, Storable,
};
use serde::Deserialize;

use crate::advertisement::Advertisement;
use crate::encode;

// use storable_macros::Storable;

#[derive(Clone, Debug, CandidType, Default)]
pub struct TradeState {
    pub ads: Advertisement,
}

impl TradeState {}
