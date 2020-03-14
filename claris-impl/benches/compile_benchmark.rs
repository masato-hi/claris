#![feature(test)]

extern crate test;

use claris_impl::Compiler;
use tempfile::tempdir;
use test::Bencher;

#[bench]
fn bench_compile_to_png<'a>(b: &mut Bencher) {
    let tmp_dir = tempdir().unwrap();
    b.iter(|| {
        let src_path = "./benches/compile_benchmark_example.yml".to_string();
        let out_path = tmp_dir
            .path()
            .join("bench_compile_to_png.png")
            .to_str()
            .and_then(|x| Some(x.to_string()))
            .unwrap();
        Compiler::compile_to_png(src_path, out_path).unwrap();
    });
    tmp_dir.close().unwrap();
}
