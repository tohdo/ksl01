pub struct Var {
    pub name: String,
}

pub struct VarContext<'a> {
    node: Option<&'a VarContextNode<'a>>,
    mgr: &'a mut VarMgr<'a>
}

struct VarContextNode<'a> {
    pub va: Var,
    pub next: Option<&'a VarContextNode<'a>>
}

impl<'a> VarContext<'a> {

    pub fn new(mgr: &'a mut VarMgr<'a>) -> VarContext<'a>{
        VarContext { node: None, mgr: mgr }
    }

    pub fn lookup (&'a self, name: &str) -> Option<&'a Var>{
        let mut p = &self.node;
        loop {
            match p {
                &Some(ref q) => {
                    if q.va.name == name {
                        return Some(&q.va);
                    } else {
                        p = &(*q).next;
                    }
                }
                &None => { return None; }
            }
        }
    }

    pub fn bind (&'a mut self, name: &str) -> &'a Var{
        let nc = self.mgr.new_context(name);
        return &nc.va;
    }
}

pub struct VarMgr<'a> {
    pool: Vec <VarContextNode<'a>>
}

impl<'a> VarMgr<'a> {
    // This version fails to compile with following error
    // error[E0502]: cannot borrow `*pool` as mutable because it is
    // also borrowed as immutable
    fn new_context(&'a mut self, name: &str) -> &'a VarContextNode<'a> {
        let pool = &mut self.pool;

        let lnode = pool.last();
        // immutable borrow of pool by last()
        
        pool.push(
        // mutable borrow of pool by push()
        
            VarContextNode{ va: Var{name: String::from(name)},
                            next: lnode }
        );
        return pool.last().unwrap();
    }
}
