use claris_impl::Compiler;

fn main() {
    Compiler::compile_to_png(
        "examples/face_sample.yml".to_string(),
        "examples/face_sample.png".to_string(),
    )
    .unwrap();
}
