
pub mod ast;
pub mod sym;


#[cfg(test)]
mod tests {
    use ::ast::Stmt;
    use ::ast::Expr;
    use ::sym::Constraint;

    #[test]
    fn print_test() {
        let c = Expr::mk_var("cond");
        let t = Stmt::mk_assign("result", Expr::mk_true());
        let f = Stmt::mk_assign("result", Expr::mk_false());
        let s = Stmt::mk_if(c, t, f);
        println!("cs: {:?}", Constraint::from_stmt(&s));
    }

    #[test]
    fn fmt_test() {
        let c = Expr::mk_var("cond");
        let t = Stmt::mk_assign("result", Expr::mk_true());
        let f = Stmt::mk_assign("result", Expr::mk_false());
        let s = Stmt::mk_if(c, t, f);
        println!("cs: {}", Constraint::from_stmt(&s));
    }
}
