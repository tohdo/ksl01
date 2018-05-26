extern crate ksl02;

use ksl02::expr::*;

fn main() {
    let mut b = Builder::new();

    b.push_var("a");
    b.push_varref("a");
    b.push_lam();

    let mut e = b.pop();

    {
        let mut b2 = Builder::new();

        b2.push_var("f");
        b2.push_var("x");
        b2.push_varref("f");
        b2.push_varref("x");
        b2.push_apply();
        b2.push_lam();
        b2.push_lam();

        e = b2.pop();
    }

    println!("hello!");
}


