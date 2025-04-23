const n = `enum Expr {
    Zero,
    Succ(Expr),
    Add(Expr, Expr),
    Mul(Expr, Expr),
}

fn main() {
    let a = Mul(Add(Zero,Zero),Zero) in
    match a {
        Add(Zero,Zero) => print(int_to_string(0)),
        Mul(Zero,x) => print(int_to_string(1)),
        Add(Succ(x),y) => print(int_to_string(2)),
        Mul(x,Zero) => print(int_to_string(3)),
        Mul(Add(x,y),z) => print(int_to_string(4)),
        Add(x,Zero) => print(int_to_string(5)),
        x => print(int_to_string(6)),
    }
}
`;
export {
  n as default
};
