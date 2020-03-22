use getopts::Options;
use log::debug;
use simple_logger;
use std::path::{Path, PathBuf};
use std::{env, process};

use claris_impl::Compiler;

#[derive(Debug)]
struct Args {
    inputs: Vec<PathBuf>,
    output_dir: Option<PathBuf>,
    force: bool,
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "d",
        "",
        "Set output directory. default: Same directory as input file.",
        "[directory]",
    );
    opts.optflag("f", "", "Overwrite output file.");
    opts.optflag("h", "help", "Print usage");

    let matches = opts
        .parse(&args[1..])
        .unwrap_or_else(|f| panic!(f.to_string()));

    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }

    if matches.free.len() < 1 {
        print_usage(&program, &opts);
    }

    debug!("{:?}", matches.free);

    let inputs = matches
        .free
        .iter()
        .map(|i| -> PathBuf { PathBuf::new().join(i) })
        .collect::<Vec<PathBuf>>();

    let output_dir: Option<PathBuf> = matches
        .opt_str("d")
        .map(|d| -> PathBuf { PathBuf::new().join(d) });

    Args {
        inputs: inputs,
        output_dir: output_dir,
        force: matches.opt_present("f"),
    }
}

fn exec_once(input: &Path, output_dir: Option<PathBuf>, force: bool) {
    debug!(
        "input: {}, output_dir: {:?}",
        input.to_string_lossy(),
        output_dir
    );

    if !input.exists() {
        println!("{} is not exists", input.to_str().unwrap());
        process::exit(1);
    }

    if !input.is_file() {
        println!("{} is not file.", input.to_str().unwrap());
        process::exit(1);
    }

    match input.extension() {
        Some(x) => {
            if x != "yml" {
                println!("{} is not yaml file.", input.to_str().unwrap());
                process::exit(1);
            }
        }
        _ => {
            println!("{} is not yaml file.", input.to_str().unwrap());
            process::exit(1);
        }
    }

    let output_filename = input
        .clone()
        .to_path_buf()
        .file_name()
        .map(|f| -> PathBuf {
            let mut r = PathBuf::new().join(f);
            r.set_extension("png");
            r
        })
        .unwrap();

    let output_path: PathBuf = match output_dir {
        Some(x) => {
            if !x.is_dir() {
                println!("{} is not directory.", x.to_str().unwrap());
                process::exit(1);
            }

            x.join(output_filename)
        }
        None => {
            if input.is_absolute() {
                input.parent().unwrap().to_path_buf().join(output_filename)
            } else {
                env::current_dir().unwrap().join(output_filename)
            }
        }
    };

    debug!(
        "input: {}, output: {}",
        input.to_string_lossy(),
        output_path.to_string_lossy()
    );

    if !force && output_path.exists() {
        println!("{} is already exists.", output_path.to_str().unwrap());
        process::exit(1);
    }

    match Compiler::compile_to_png(
        input.to_str().unwrap().to_string(),
        output_path.to_str().unwrap().to_string(),
    ) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

fn init() {
    simple_logger::init().unwrap();
}

fn main() {
    init();

    let args = parse_args();
    debug!("{:?}", args);

    for input in args.inputs {
        debug!("--------------------begin--------------------");
        exec_once(input.as_path(), args.output_dir.clone(), args.force);
        debug!("---------------------end---------------------");
    }
}
