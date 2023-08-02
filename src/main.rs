use std::env;
use std::fs;
use std::process;

struct Args {
    input: Vec<String>,
    output: Option<String>,
    style: Option<String>,
}

fn main() {
    match parse_args() {
        Ok(args) => emit(args),
        Err(_) => usage(),
    }
}

fn emit(args: Args) {
    let mut error = false;
    let mut files = vec![];

    for ref file in args.input {
        let contents = fs::read_to_string(file);
        if let Ok(s) = contents {
            files.push(s);
        } else {
            eprintln!("error: could not read file: {}", file);
            error = true;
        }
    }

    for file in files {
        parse(file);
    }

    if error {
        process::exit(-1);
    }
}

fn parse(s: String) {
    print!("{}", s);
}

fn parse_args() -> Result<Args, ()> {
    let mut result = Args {
        input: vec![],
        output: None,
        style: None,
    };
    let mut args = env::args();
    args.next(); // skip argv[0]

    loop {
        let arg = if let Some(arg) = args.next() {
            arg
        } else {
            break;
        };

        match arg.as_str() {
            "-o" => {
                if let Some(o) = args.next() {
                    result.output = Some(o);
                } else {
                    return Err(());
                }
            }
            "-s" => {
                if let Some(s) = args.next() {
                    result.style = Some(s);
                } else {
                    return Err(());
                }
            }
            _ => result.input.push(arg),
        }
    }

    if result.input.len() > 0 {
        Ok(result)
    } else {
        Err(())
    }
}

fn usage() {
    eprintln!("usage: {} [options] <files>", env::args().nth(0).unwrap());
    eprintln!("options:");
    eprintln!("\t-o <path>\temit html to the file at path");
    eprintln!("\t-s <url>\tlink to the stylesheet at url");
    process::exit(-1);
}
