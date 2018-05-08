use vars::Var;

pub enum Expr<'a> {
    VarRef (&'a Var),
    App { fun: Box<Expr<'a>>, arg: Box<Expr<'a>> },
    Lam { va: &'a Var, body: Box<Expr<'a>> }
}

