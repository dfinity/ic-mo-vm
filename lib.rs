use candid::Nat;
use ic_cdk::{
    api::call::ManualReply,
    export::{candid, Principal},
};
use ic_cdk_macros::*;
use motoko::{
    ast::Delim,
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
    CORE.with(|core| ManualReply::one(Nat::from(core.borrow().counts.redex)))
}

#[update]
fn step() {
    let mut limits = Limits::none();
    limits.step_redex(1);
    CORE.with(|core| core.borrow_mut().step(&limits).unwrap());
}
