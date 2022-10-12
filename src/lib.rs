use motoko::{parse, Core, Dynamic};
use std::cell::RefCell;

mod stable_map;

const MAX_KEY_SIZE: u32 = 10;
const MAX_VALUE_SIZE: u32 = 100;

thread_local! {
    // Initialize a MoVM core with a global `StableBTreeMap`.
    static CORE: RefCell<Core> = RefCell::new({
        let mut core = Core::empty();
        let ptr = core.alloc(stable_map::StableMap::shared(MAX_KEY_SIZE, MAX_VALUE_SIZE));
        core.assign_alloc("stableMap", motoko::value::Value::Pointer(ptr));
        core.assign("newStableMap", stable_map::NewStableMap.into_value());
        core
    });
}

fn eval_(prog: String) -> String {
    CORE.with(|core| {
        let prog = parse(&prog).expect("parse error");
        let result = core.borrow_mut().eval_prog(prog).expect("eval error");
        format!("{:?}", result)
    })
}

#[ic_cdk_macros::update]
fn eval(prog: String) -> String {
    eval_(prog)
}

#[ic_cdk_macros::query]
fn eval_query(prog: String) -> String {
    eval_(prog)
}

#[ic_cdk_macros::update]
fn reset() -> () {
    CORE.with(|core| *core.borrow_mut() = Core::empty())
}
