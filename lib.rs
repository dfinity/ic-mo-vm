use candid::Nat;
use ic_cdk::{
    api::call::ManualReply,
    export::{candid, Principal},
};
use ic_cdk_macros::*;
use motoko::{
    ast::Delim,
    check::parse,
    vm_types::{Core, Interruption, Limit, Limits},
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

#[update(manual_reply = true)]
fn step() -> ManualReply<Nat> {
    CORE.with(|core| {
        let limits = Limits::none().step(core.borrow().counts.step + 1);
        loop {
            match core.borrow_mut().step(&limits) {
                Err(Interruption::Done(_)) => {
                    break;
                }
                Err(Interruption::Limit(Limit::Step)) => {
                    break;
                }
                Err(e) => {
                    println!("warning: {:?}", e);
                    break;
                }
                Ok(_) => {}
            }
        }
        ManualReply::one(Nat::from(core.borrow().counts.step))
    })
}

#[update(manual_reply = true)]
fn redex() -> ManualReply<Nat> {
    CORE.with(|core| {
        let limits = Limits::none().redex(core.borrow().counts.redex + 1);
        loop {
            match core.borrow_mut().step(&limits) {
                Err(Interruption::Done(_)) => {
                    break;
                }
                Err(Interruption::Limit(Limit::Redex)) => {
                    break;
                }
                Err(e) => {
                    println!("warning: {:?}", e);
                    break;
                }
                Ok(_) => {}
            }
        }
        ManualReply::one(Nat::from(core.borrow().counts.redex))
    })
}

#[update]
fn load(program: String) {
    let p = parse(&program).unwrap();
    CORE.with(|core| *core.borrow_mut() = Core::new(p))
}
