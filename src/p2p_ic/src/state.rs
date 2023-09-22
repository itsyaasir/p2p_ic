use std::cell::RefCell;

use candid::CandidType;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};

#[derive(Clone, Debug, CandidType, Default)]
pub struct TradeState {
    pub ads: Advertisement,
    pub trades: Trade,
}

#[derive(Clone, Debug, CandidType, Default)]
pub struct Advertisement;

#[derive(Clone, Debug, CandidType, Default)]
pub struct Trade;

impl Advertisement {
    pub fn clear(&mut self) {
        ADS_MAP.with(|ads_map| {});
    }
}

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static ADS_MAP: RefCell<StableBTreeMap<u64,u64, Memory>> =
        MEMORY_MANAGER.with(|mm|
            RefCell::new(
                StableBTreeMap::init(mm.borrow().get(MemoryId::new(1)))
            )
        );

    static TRADE_MAP: RefCell<StableBTreeMap<u64,u64, Memory>> =
        MEMORY_MANAGER.with(|mm|
            RefCell::new(
                StableBTreeMap::init(mm.borrow().get(MemoryId::new(2)))
            )
        );
}
