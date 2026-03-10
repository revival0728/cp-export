use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};

fn error(msg: &str) {
    println!("Error: {}", msg);
}

fn open_file(path: &str) -> Result<File, ()> {
    let src_path = Path::new("src");
    let file = match File::open(src_path.join(path)) {
        Ok(f) => f,
        Err(_) => {
            error(&format!("{} not found", path));
            return Err(());
        }
    };
    Ok(file)
}

fn read_file(path: &str) -> Result<String, ()> {
    let mut file = open_file(path)?;
    let mut source = String::new();
    match file.read_to_string(&mut source) {
        Ok(_) => {},
        Err(_) => {
            error(&format!("cannot read {}", path));
            return Err(());
        }
    };
    Ok(source)
}

fn parse_include(s: &mut String) -> Result<(), ()> {
    let lb = match s.find("\"") { Some(p) => p, None => { error(&format!("found syntax error [ {} ]", s)); return Err(()); } };
    let rb = match s.rfind("\"") { Some(p) => p, None => { error(&format!("found syntax error [ {} ]", s)); return Err(()); } };
    *s = read_file(&s[lb+1..rb])?;
    Ok(())
}

fn main() -> Result<(), ()> {
    let mut main_source = read_file("main.rs")?
        .split(|c| c == '\n' || c == '\r')
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    for s in &mut main_source {
        if s.starts_with("include!") {
            parse_include(s)?;
        }
    }
    let mut main_file = match File::create("solution.rs") {
        Ok(f) => f,
        Err(_) => {
            error("cannot open or create solution.rs");
            return Err(());
        }
    };
    match main_file.write_all(main_source.join("\n").as_bytes()) {
        Ok(_) => {},
        Err(_) => {
            error("cannot write to solution.rs");
        }
    };

    Ok(())
}
