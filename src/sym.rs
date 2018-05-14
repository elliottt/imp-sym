
use std::fmt;

use ast::Expr;
use ast::Stmt;

#[derive(Debug)]
pub enum Constraint {
    Equal(Box<Expr>, Box<Expr>),
    Or(Box<Constraint>, Box<Constraint>),
    And(Box<Constraint>, Box<Constraint>),
    Imp(Box<Constraint>, Box<Constraint>),
    True(),
}

impl Constraint {

    pub fn mk_equal(a: Box<Expr>, b: Box<Expr>) -> Box<Constraint> {
        Box::new(Constraint::Equal(a,b))
    }

    pub fn mk_or(a: Box<Constraint>, b: Box<Constraint>) -> Box<Constraint> {
        Box::new(Constraint::Or(a,b))
    }

    pub fn mk_and(a: Box<Constraint>, b: Box<Constraint>) -> Box<Constraint> {
        Box::new(Constraint::And(a,b))
    }

    pub fn mk_imp(a: Box<Constraint>, b: Box<Constraint>) -> Box<Constraint> {
        Box::new(Constraint::Imp(a,b))
    }

    pub fn mk_true() -> Box<Constraint> {
        Box::new(Constraint::True())
    }

    pub fn from_stmt(stmt: &Stmt) -> Box<Constraint> {
        match *stmt {
            Stmt::Skip =>
                Constraint::mk_true(),

            Stmt::Assign(ref s,ref e) =>
                Constraint::mk_equal(Expr::mk_var(s), e.clone()),

            Stmt::If(ref b, ref t, ref f) =>
                Constraint::mk_and(
                    Constraint::mk_imp(
                        Constraint::mk_equal(b.clone(), Expr::mk_true()),
                        Constraint::from_stmt(t)),
                    Constraint::mk_imp(
                        Constraint::mk_equal(b.clone(), Expr::mk_false()),
                        Constraint::from_stmt(f))
                    ),
        }
    }

}

impl fmt::Display for Constraint {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Constraint::Equal(ref a, ref b) =>
                write!(f, "{} = {}", a, b),

            Constraint::Imp(ref a,ref b) =>
                write!(f, "({}) => ({})", a, b),

            Constraint::And(ref a,ref b) =>
                write!(f, "({}) /\\ ({})", a, b),

            _ =>
                unimplemented!()
        }
    }

}
