use crate::expr::Expr;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    VarDec {
        name: String,
        initializer: Option<Expr>,
    },
}
