use std::borrow::Cow;
use std::cell::RefCell;

use candid::{CandidType, Principal};
use ic_stable_structures::{
    BoundedStorable, MemoryId, SlicedStorable, StableUnboundedMap, Storable, UnboundedMapStructure,
};
use serde::Deserialize;

use crate::encode;

#[derive(Clone, Debug, CandidType, Default)]
pub struct Advertisement;

impl Advertisement {
    pub fn clear(&mut self) {
        ADVERTISEMENT_MAP.with(|map| {
            map.borrow_mut().clear();
        })
    }
}

#[derive(Debug)]
pub struct PrincipalKey(Principal);

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct AdvertisementArgs {
    pub user: Principal,
    pub token0_name: String,
    pub token1_name: String,
    pub token0_amount: u64,
    pub token1_amount: u64,
    pub token0_price: u64,
    pub token1_price: u64,
    pub token0_id: Principal,
    pub token1_id: Principal,
}
impl Storable for PrincipalKey {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        encode::encode(&self.0).into()
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        PrincipalKey(encode::decode(&bytes))
    }
}

impl BoundedStorable for PrincipalKey {
    const MAX_SIZE: u32 = 29;
    const IS_FIXED_SIZE: bool = true;
}

impl SlicedStorable for AdvertisementArgs {
    const CHUNK_SIZE: ic_stable_structures::ChunkSize = 100;
}

impl Storable for AdvertisementArgs {
    fn to_bytes(&self) -> std::borrow::Cow<'_, [u8]> {
        encode::encode(self).into()
    }

    fn from_bytes(bytes: std::borrow::Cow<'_, [u8]>) -> Self {
        encode::decode(&bytes)
    }
}

thread_local! {
    static ADVERTISEMENT_MAP: RefCell<StableUnboundedMap<PrincipalKey, AdvertisementArgs>> = RefCell::new(StableUnboundedMap::new(MemoryId::new(0)))
}
