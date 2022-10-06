use motoko::{
    check::parse,
    vm_types::Core,
};
use std::cell::RefCell;

thread_local! {
    static CORE: RefCell<Core> = RefCell::new(Core::empty());
}

#[ic_cdk_macros::update]
fn eval(prog: String) -> String {
    CORE.with(|core| {
        let program = parse(&prog).expect("parse error");

        let result_val =
            core.borrow_mut().eval_open_block(
                vec![], program).expect("eval error");

        format!("{:?}", result_val)
    })
}
