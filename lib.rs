use motoko::{
    check::parse,
    vm_types::Core,
};
use std::cell::RefCell;

thread_local! {
    static CORE: RefCell<Core> = RefCell::new(Core::empty());
}

fn eval_(prog: String) -> String {
    CORE.with(|core| {
        let p = parse(&prog).expect("parse error");

        let result_val =
            core.borrow_mut().eval_prog(p).expect("eval error");

        format!("{:?}", result_val)
    })
}

#[ic_cdk_macros::update]
fn eval(prog: String) -> String {
    eval_(prog)
}

#[ic_cdk_macros::query]
fn eval_q(prog: String) -> String {
    eval_(prog)
}
