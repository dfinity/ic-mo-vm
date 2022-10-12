use motoko::{ast::ToId, check::parse, shared::Share, vm_types::Core};
use std::cell::RefCell;

mod stable_map;

const MAX_KEY_SIZE: u32 = 10;
const MAX_VALUE_SIZE: u32 = 100;

thread_local! {
    // Initialize a MoVM core with a global `StableBTreeMap`.
    static CORE: RefCell<Core> = RefCell::new({
        let mut core = Core::empty();
        let ptr = core.alloc(stable_map::StableMap::shared(MAX_KEY_SIZE, MAX_VALUE_SIZE));
        core.env.insert("stableMap".to_id(), motoko::value::Value::Pointer(ptr).share());
        core
    });
}

fn eval_(prog: String) -> String {
    CORE.with(|core| {
        let p = parse(&prog).expect("parse error");
        let result_val = core.borrow_mut().eval_prog(p).expect("eval error");
        format!("{:?}", result_val)
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
