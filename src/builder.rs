use expr::*;
use vars::*;

pub struct Builder<'a> {
    vc: VarContext<'a>,
    stack: Vec<Opr<'a>>
}

pub enum Opr<'a> {
    Push (Box<Expr<'a>>),
    PushVar (&'a Var)
}

impl<'a> Builder<'a> {
    pub fn new(mgr: &'a mut VarMgr<'a>) -> Builder<'a>{
        return Builder {
            vc: VarContext::new(mgr),
            stack: Vec::new()
        };
    }

    pub fn push_varref(&'a mut self, name: &'a str){
        let va = self.vc.lookup(name).unwrap();
        self.stack.push(Opr::Push(Box::new(Expr::VarRef(va))));
    }

    pub fn push_app(&'a mut self){
        let f = self.stack.pop().unwrap();
        let x = self.stack.pop().unwrap();
        match (f,x) {
            (Opr::Push (fp), Opr::Push(xp)) => {
                self.stack.push(Opr::Push(Box::new(Expr::App {fun: fp, arg: xp})));
            }
            _ => { panic!(); }
        }
    }

    pub fn mk_var(&'a mut self, name: &'a str){
        let va = self.vc.bind(name);
        self.stack.push(Opr::PushVar(va));
    }

    pub fn push_lam(&'a mut self){
        let bp = self.stack.pop().unwrap();
        let vp = self.stack.pop().unwrap();
        match (vp,bp) {
            (Opr::PushVar(va), Opr::Push(b)) => {
                self.stack.push(Opr::Push(Box::new(Expr::Lam {va: va, body: b})));
            }
            _ => { panic!(); }
        }
    }

    pub fn pop(&mut self) -> Box<Expr<'a>> {
        let x = self.stack.pop().unwrap();
        if self.stack.is_empty() {
            match x {
                Opr::Push(xp) => { return xp; }
                _ => { panic!(); }
            }
        } else { panic!(); }
    }
}
