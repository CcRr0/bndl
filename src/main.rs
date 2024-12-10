use argh::FromArgs;
use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

#[derive(FromArgs)]
#[argh(description = "bndl")]
struct Args {
    #[argh(positional, description = "entry")]
    entry: String,

    #[argh(option, short = 'o', description = "output")]
    output: String,

    #[argh(option, short = 'i', default = "4", description = "indent")]
    indent: usize,
}

fn bundle(
    path: &mut PathBuf, depth: usize, indent: usize, re: &Regex,
) -> io::Result<String> {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let prefix = " ".repeat(indent * depth);

    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut res = String::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re.captures(&line) {
            let module = caps.get(2).unwrap().as_str();
            let is_pub = caps.get(1).is_some();

            res.push_str(&prefix);
            if is_pub { res.push_str("pub "); }
            res.push_str(&format!("mod {} {{\n", module));

            let bndl = read_module(path, &name, module, depth, indent, re)?;
            res.push_str(&bndl);

            res.push_str(&prefix);
            res.push_str("}\n");
        } else {
            res.push_str(&prefix);
            res.push_str(&line);
            res.push('\n');
        }
    }

    Ok(res)
}

fn read_module(
    path: &mut PathBuf, name: &str, module: &str,
    depth: usize, indent: usize, re: &Regex,
) -> io::Result<String> {
    path.pop();
    path.push(format!("{}.rs", module));
    if path.exists() {
        let res = bundle(path, depth + 1, indent, re)?;
        path.pop();
        path.push(name);
        return Ok(res);
    }

    path.pop();
    path.push(module);
    path.push("mod.rs");
    if path.exists() {
        let res = bundle(path, depth + 1, indent, re)?;
        path.pop();
        path.pop();
        path.push(name);
        return Ok(res);
    }

    panic!();
}

const RE: &str = r"^\s*(pub\s+)?mod\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*;.*$";

fn main() -> io::Result<()> {
    let args: Args = argh::from_env();
    let entry = args.entry;
    let output = args.output;
    let indent = args.indent;

    let mut path = PathBuf::from(&entry);
    let re = Regex::new(RE).unwrap();

    let bndl = bundle(&mut path, 0, indent, &re)?;

    let mut file = File::create(&output)?;
    file.write_all(bndl.as_bytes())?;

    Ok(())
}
