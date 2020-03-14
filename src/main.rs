use simple_logger;

use claris_impl::Compiler;

fn init() {
    simple_logger::init().unwrap();
}
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    init();

    println!("Hello, world!");
    //parse_yaml();
    let s = "sample.yml";
    match Compiler::compile_to_png(s.to_string(), "test.png".to_string()) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}
