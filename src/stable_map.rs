use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use motoko::vm_types::Interruption;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;

use motoko::dynamic::{Dynamic, Result};
use motoko::shared::{Share, Shared};
use motoko::value::{ToMotoko, Value};

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Stable map container for MoVM
pub struct StableMap(StableBTreeMap<Memory, String, Vec<u8>>);

impl StableMap {
    pub fn shared(max_key_size: u32, max_value_size: u32) -> Shared<Value> {
        // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
        // return a memory that can be used by stable structures.
        let memory = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

        // Initialize a `StableBTreeMap` with `MemoryId(0)`.
        let map = StableBTreeMap::init(
            memory.borrow().get(MemoryId::new(0)),
            max_key_size,
            max_value_size,
        );

        StableMap(map).into_value().share()
    }
}

impl Clone for StableMap {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl core::fmt::Debug for StableMap {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl core::hash::Hash for StableMap {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}

// Rust-Motoko bindings
impl Dynamic for StableMap {
    fn get_index(&self, index: Shared<Value>) -> Result {
        self.0
            .borrow()
            .get(&index.to_rust().map_err(Interruption::ValueError)?)
            .to_shared()
            .map_err(Interruption::ValueError)
    }

    fn set_index(&mut self, index: Shared<Value>, value: Shared<Value>) -> Result<()> {
        drop(self.0.borrow_mut().insert(
            index.to_rust().map_err(Interruption::ValueError)?,
            value.to_rust().map_err(Interruption::ValueError)?,
        ));
        Ok(())
    }
}
