extern crate ksl01;

use ksl01::builder::*;

fn main() {
    let mut b = Builder::new();

    b.mk_var("f");
    b.mk_var("x");
    b.push_varref("f");
    b.push_varref("x");
    b.push_app();
    b.push_lam();
    b.push_lam();

    let r = b.pop();
}

