const n = `fn fib(x: Int) -> Int {
  match int_less(x, 2) {
    false => int_add(fib(int_sub(x, 1)), fib(int_sub(x, 2))),
    true => 1,
  }
}

fn main() {
    print(int_to_string(fib(10)))
}
`;
export {
  n as default
};
