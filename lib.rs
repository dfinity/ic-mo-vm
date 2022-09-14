use candid::Nat;
use ic_cdk::{
    api::call::ManualReply,
    export::{candid, Principal},
};
use ic_cdk_macros::*;
use motoko::{
    ast::Delim,
    check::parse,
    vm_types::{Core, Limits},
};
use std::cell::{Cell, RefCell};

thread_local! {
    static CORE: RefCell<Core> = RefCell::new(Core::new(Delim::new()));
    static OWNER: Cell<Principal> = Cell::new(Principal::from_slice(&[]));
}

#[init]
fn init() {
    OWNER.with(|owner| owner.set(ic_cdk::api::caller()));
}

#[query(manual_reply = true)]
fn read() -> ManualReply<Nat> {
    CORE.with(|core| ManualReply::one(Nat::from(core.borrow().counts.step)))
}

#[update]
fn step() {
    let mut limits = Limits::none();
    CORE.with(|core| {
        limits.step_redex(core.borrow().counts.redex + 1);
        core.borrow_mut().step(&limits).unwrap()
    });
}

#[update]
fn load(program: String) {
    let p = parse(&program).unwrap();
    CORE.with(|core| *core.borrow_mut() = Core::new(p))
}

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
