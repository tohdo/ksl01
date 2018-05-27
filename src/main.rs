extern crate ksl01;

use ksl01::expr::*;
use ksl01::error::*;

fn main() -> KResult<()>{
    let mut b = Builder::new();

    try!(b.push_var("a"));
    try!(b.push_varref("a"));
    try!(b.push_lam());

    let mut e = try!(b.pop());

    {
        let mut b2 = Builder::new();

        try!(b2.push_var("f"));
        try!(b2.push_var("x"));
        try!(b2.push_varref("f"));
        try!(b2.push_varref("x"));
        try!(b2.push_apply());
        try!(b2.push_lam());
        try!(b2.push_lam());

        e = try!(b2.pop());
    }

    println!("hello!");
    return Ok(());
}


