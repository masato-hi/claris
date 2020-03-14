use simple_logger;

use claris_impl::Compiler;

fn init() {
    simple_logger::init().unwrap();
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
