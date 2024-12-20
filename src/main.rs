use argh::FromArgs;
use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::sync::LazyLock;

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
    path: &mut PathBuf, depth: usize, indent: usize,
) -> io::Result<String> {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let prefix = " ".repeat(indent * depth);

    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut res = String::new();

    for line in reader.lines() {
        let line = line?;
        let args = RE_ARGS.captures_iter(&line)
            .map(|c| c[1].to_string())
            .collect::<Vec<_>>();

        let is_ignore = args.contains(&"ignore".to_string());

        if is_ignore {
            res.push_str(&prefix);
            res.push_str("// ");
            res.push_str(&RE_PURE.captures(&line).unwrap()[1]);
            res.push('\n');
            continue;
        }

        if let Some(caps) = RE_MOD.captures(&line) {
            let module = &caps[2];
            let is_pub = caps.get(1).is_some();

            res.push_str(&prefix);
            if is_pub { res.push_str("pub "); }
            res.push_str(&format!("mod {} {{\n", module));

            let bndl = read_module(path, &name, module, depth, indent)?;
            res.push_str(&bndl);

            res.push_str(&prefix);
            res.push_str("}\n");
        } else {
            if !line.is_empty() {
                res.push_str(&prefix);
                res.push_str(&line);
            }
            res.push('\n');
        }
    }

    Ok(res)
}

fn read_module(
    path: &mut PathBuf, name: &str, module: &str,
    depth: usize, indent: usize,
) -> io::Result<String> {
    path.pop();
    path.push(format!("{}.rs", module));
    if path.exists() {
        let res = bundle(path, depth + 1, indent)?;
        path.pop();
        path.push(name);
        return Ok(res);
    }

    path.pop();
    path.push(module);
    path.push("mod.rs");
    if path.exists() {
        let res = bundle(path, depth + 1, indent)?;
        path.pop();
        path.pop();
        path.push(name);
        return Ok(res);
    }

    panic!();
}

static RE_MOD: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r"^\s*(pub\s+)?mod\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*;.*$"
).unwrap());

static RE_ARGS: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r"//.*@(\S+)"
).unwrap());

static RE_PURE: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r"^\s*(.*?)\s*//"
).unwrap());

fn main() -> io::Result<()> {
    let args: Args = argh::from_env();
    let entry = args.entry;
    let output = args.output;
    let indent = args.indent;

    let mut path = PathBuf::from(&entry);
    let bndl = bundle(&mut path, 0, indent)?;

    let mut file = File::create(&output)?;
    file.write_all(bndl.as_bytes())?;

    Ok(())
}
