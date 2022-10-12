use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use motoko::dynamic::Result;
use motoko::{Core, Dynamic, Share, Shared, Value};
use std::cell::RefCell;
use std::rc::Rc;

type Memory = VirtualMemory<DefaultMemoryImpl>;

#[derive(Clone, Debug, Hash)]
pub struct NewStableMap;

impl Dynamic for NewStableMap {
    fn call(
        &mut self,
        core: &mut Core,
        _inst: &Option<motoko::ast::Inst>,
        args: Shared<Value>,
    ) -> Result {
        let (max_key_size, max_value_size) = args.to_rust()?;
        let map = StableMap::shared(max_key_size, max_value_size);
        Ok(Value::Pointer(core.alloc(map)).share())
    }
}

// Stable map container for MoVM
pub struct StableMap(Rc<RefCell<StableBTreeMap<Memory, String, Vec<u8>>>>);

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

        StableMap(Rc::new(RefCell::new(map))).into_value().share()
    }
}

impl Dynamic for StableMap {
    fn get_index(&self, _core: &Core, index: Shared<Value>) -> Result {
        let result = (&*self.0).borrow().get(&index.to_rust()?);

        Ok(match result {
            Some(vec) => Value::Option(Value::Blob(vec).share()),
            None => Value::Null,
        }
        .share())
    }

    fn set_index(
        &mut self,
        _core: &mut Core,
        index: Shared<Value>,
        value: Shared<Value>,
    ) -> Result<()> {
        drop(
            (&*self.0)
                .borrow_mut()
                .insert(index.to_rust()?, value.to_rust()?),
        );
        Ok(())
    }
}

// TODO: remove once `StableBTreeMap` implements `Clone`
impl Clone for StableMap {
    fn clone(&self) -> Self {
        StableMap(self.0.clone())
    }
}


// TODO: remove once `StableBTreeMap` implements `Debug`
impl core::fmt::Debug for StableMap {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


// TODO: remove once `StableBTreeMap` implements `Hash`
impl core::hash::Hash for StableMap {
    fn hash<H: std::hash::Hasher>(&self, _state: &mut H) {
        todo!()
    }
}
