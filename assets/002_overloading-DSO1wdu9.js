const n = `trait Arith {
    fn add(Self, Self) -> Int;
    fn less(Self, Self) -> Bool;
}

impl Arith for Int {
    fn add(self: Int, other: Int) -> Int {
        int_add(self, other)
    }

    fn less(self: Int, other: Int) -> Bool {
        int_less(self, other)
    }
}

trait ToString {
    fn to_string(Self) -> String;
}

impl ToString for Int {
    fn to_string(self: Int) -> String {
        int_to_string(self)
    }
}

impl ToString for Bool {
    fn to_string(self: Bool) -> String {
        bool_to_string(self)
    }
}

trait Output {
    fn output(Self) -> Unit;
}

impl Output for Int {
    fn output(self: Int) -> Unit {
        println(to_string(self))
    }
}

impl Output for Bool {
    fn output(self: Bool) -> Unit {
        println(to_string(self))
    }
}

fn id[T](x: T) -> T {
    x
}

fn main() {
    let a = id(1) in
    let b = id(2) in
    let c = add(a, b) in
    let _ = output(c) in

    let a = id(3) in
    let b = id(4) in
    let c = less(a, b) in
    let _ = output(c) in
    
    ()
}
`;
export {
  n as default
};
