use error::*;

macro_rules! kerr {
    () => (KError { file: file!(), line: line!() });
}

pub struct Var {
    pub name: String
}

pub enum Expr {
    VarRef (*const Var),
    App { fun: Box<Expr>, arg: Box<Expr> },
    Lam { va: *const Var, body: Box<Expr> }
}

pub struct Builder {
    vars: Vec<Var>,
    stack: Vec<Box<Opr>>
}

enum Opr{
    Push (Box<Expr>),
    PushVar (*const Var)
}

impl Builder {

    pub fn new() -> Builder {
        Builder{ vars:Vec::new(), stack: Vec::new() }
    }

    fn lookup(&self, name: &str) -> Option<*const Var>{
        let mut i = self.stack.len();
        while i != 0 {
            i = i - 1;
            let e: & Opr = &*self.stack[i];
            if let Opr::PushVar(v) = e {
                unsafe {
                    let mut v2: &Var = &**v;
                    if v2.name == String::from(name) {
                        return Some(v2);
                    }
                }
            }
        }
        return None;
    }

    pub fn push_var(&mut self, name: & str) -> KResult<()> {
        self.vars.push(Var{name: String::from(name)});
        let v: *const Var = try!(self.vars.last().ok_or(kerr!()));
        self.stack.push(Box::new(Opr::PushVar(v)));
        return Ok(());
    }

    pub fn push_lam(&mut self) -> KResult<()> {
        let pb = try!(self.stack.pop().ok_or(kerr!()));
        let pv = try!(self.stack.pop().ok_or(kerr!()));
        match (*pv, *pb) {
            (Opr::PushVar(v), Opr::Push(e)) => {
                let e2 = Box::new(Expr::Lam { va: v, body: e });
                self.stack.push(Box::new(Opr::Push(e2)));
                return Ok(());
            }
            _ => {}
        }
        return Err(kerr!());
    }

    pub fn push_varref(&mut self, name: &str) -> KResult<()> {
        let v = try!(self.lookup(name).ok_or(kerr!()));
        let e = Box::new(Expr::VarRef(v));
        self.stack.push(Box::new(Opr::Push(e)));
        return Ok(());
    }

    pub fn push_apply(&mut self) -> KResult<()> {
        let parg = try!(self.stack.pop().ok_or(kerr!()));
        let pfun = try!(self.stack.pop().ok_or(kerr!()));
        match (*pfun, *parg) {
            (Opr::Push(fun), Opr::Push(arg)) => {
                let e = Box::new(Expr::App{ fun: fun, arg: arg });
                self.stack.push(Box::new(Opr::Push(e)));
                return Ok(());
            }
            _ => {}
        }
        return Err(kerr!());
    }

    pub fn pop(mut self) -> KResult<Box<Expr>>{
        let p = try!(self.stack.pop().ok_or(kerr!()));
        if ! self.stack.is_empty(){ return Err(kerr!()); }
        if let Opr::Push(e) = *p {
            return Ok(e);
        }
        return Err(kerr!());
    }
}
