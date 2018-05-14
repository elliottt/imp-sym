
use std::fmt;

#[derive(Debug,Clone)]
pub enum Stmt {
    Assign(String,Box<Expr>),
    If(Box<Expr>,Box<Stmt>,Box<Stmt>),
    Skip,
}

impl Stmt {
    pub fn mk_assign<T: ToString>(str: T, e: Box<Expr>) -> Box<Stmt> {
        Box::new(Stmt::Assign(str.to_string(), e))
    }

    pub fn mk_if(b: Box<Expr>, t: Box<Stmt>, f: Box<Stmt>) -> Box<Stmt> {
        Box::new(Stmt::If(b, t, f))
    }

    pub fn mk_skip() -> Box<Stmt> {
        Box::new(Stmt::Skip)
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Stmt::Assign(ref s, ref rhs) =>
                write!(f, "{} = {}", s, rhs),

            Stmt::If(ref c,ref tb,ref fb) =>
                write!(f, "if ({}) ({}) ({})", c, tb, fb),

            Stmt::Skip =>
                write!(f, "skip"),
        }
    }
}

#[derive(Debug,Clone)]
pub enum Expr {
    Var(String),
    Not(Box<Expr>),
    True,
    False,
}

impl Expr {
    pub fn mk_var<T: ToString>(str: T) -> Box<Expr> {
        Box::new(Expr::Var(str.to_string()))
    }

    pub fn mk_not(e: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Not(e))
    }

    pub fn mk_true() -> Box<Expr> {
        Box::new(Expr::True)
    }

    pub fn mk_false() -> Box<Expr> {
        Box::new(Expr::False)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Var(ref s) =>
                write!(f, "{}", s),

            Expr::Not(ref e) =>
                write!(f, "not {}", e),

            Expr::True =>
                write!(f, "true"),

            Expr::False =>
                write!(f, "false")
        }
    }
}
