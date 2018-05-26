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

    pub fn push_var(&mut self, name: & str){
        self.vars.push(Var{name: String::from(name)});
        let v: *const Var = self.vars.last().unwrap();
        self.stack.push(Box::new(Opr::PushVar(v)));
    }

    pub fn push_lam(&mut self){
        let pb = self.stack.pop().unwrap();
        let pv = self.stack.pop().unwrap();
        match (*pv, *pb) {
            (Opr::PushVar(v), Opr::Push(e)) => {
                let e2 = Box::new(Expr::Lam { va: v, body: e });
                self.stack.push(Box::new(Opr::Push(e2)));
                return;
            }
            _ => {}
        }
        panic!();
    }

    pub fn push_varref(&mut self, name: &str){
        let v = self.lookup(name).unwrap();
        let e = Box::new(Expr::VarRef(v));
        self.stack.push(Box::new(Opr::Push(e)));
    }

    pub fn push_apply(&mut self){
        let parg = self.stack.pop().unwrap();
        let pfun = self.stack.pop().unwrap();
        match (*pfun, *parg) {
            (Opr::Push(fun), Opr::Push(arg)) => {
                let e = Box::new(Expr::App{ fun: fun, arg: arg });
                self.stack.push(Box::new(Opr::Push(e)));
                return;
            }
            _ => {}
        }
        panic!();
    }

    pub fn pop(mut self) -> Box<Expr>{
        let p = self.stack.pop().unwrap();
        if let Some(_) = self.stack.last(){ panic!(); }
        if let Opr::Push(e) = *p {
            return e;
        }
        panic!();
    }
}
