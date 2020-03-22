use claris_impl::Compiler;

fn main() {
    Compiler::compile_to_png(
        "examples/node_samples.yml".to_string(),
        "examples/node_samples.png".to_string(),
    )
    .unwrap();
}
