use macros::hashmap;

/// For expansion tests.
fn main() {
    let l = hashmap!(@len: 1, 2, 3);
    println!("{}", l);
    let map = hashmap!("test" => 2, "prout" => 10, "a" => 20, "b" => 30);
    println!("{:?}", map);
}
