use std::rc::Rc;

#[derive(PartialEq, Hash, Debug)]
enum Expr {
    Add(Rc<Expr>, Rc<Expr>),
    Literal(i64),
}

fn eval(expr: &Expr) -> i64 {
    use Expr::*;
    match expr {
        Add(lhs, rhs) => { eval(lhs) + eval(rhs) }
        Literal(i) => { *i }
    }
}

fn main() {
    let lhs = Rc::new(Expr::Literal(10));
    let rhs = Rc::new(Expr::Literal(10));
    let e2 = Expr::Add(lhs, rhs);
    // let eq = Expr::Literal(10) == Expr::Literal(10);
    println!("yolo {:?}", eval(&e2));
}
