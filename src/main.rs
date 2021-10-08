include!("aa/fun.rs");

fn test_include_fun() {
    another_function();
}
fn main() {
    let args = std::env::args();
    println!("{:?}", args);
    test_include_fun();
}