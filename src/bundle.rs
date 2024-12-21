use crate::regex::{RE_MOD, RE_ARGS};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

pub fn bundle(
    path: &mut PathBuf, depth: usize, indent: usize,
) -> io::Result<String> {
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let prefix = " ".repeat(indent * depth);

    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut res = String::new();

    let mut flag = false;
    for line in reader.lines() {
        let line = line?;
        let args = RE_ARGS.captures_iter(&line)
            .map(|c| c[1].to_string())
            .collect::<Vec<_>>();

        let is_ignore = args.contains(&"ignore".to_string());
        if is_ignore {
            flag = true;
            continue;
        }

        if let Some(caps) = RE_MOD.captures(&line) {
            let module = &caps[2];
            let is_pub = caps.get(1).is_some();

            res.push_str(&prefix);
            if is_pub { res.push_str("pub "); }
            res.push_str(&format!("mod {} {{\n", module));

            let bndl = read_module(path, &name, module, depth, indent)?;
            let lines = bndl.lines().collect::<Vec<_>>();

            let start = lines.iter()
                .position(|li| !li.trim().is_empty())
                .unwrap_or(0);
            let end = lines.iter()
                .rposition(|li| !li.trim().is_empty()).map(|i| i + 1)
                .unwrap_or(lines.len());
            for line in &lines[start..end] {
                res.push_str(line);
                res.push('\n');
            }

            res.push_str(&prefix);
            res.push_str("}\n");
        } else {
            if !line.is_empty() {
                res.push_str(&prefix);
                res.push_str(&line);
            }
            if !flag {
                res.push('\n');
            }
        }

        flag = false;
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
        let bndl = bundle(path, depth + 1, indent)?;
        path.pop();
        path.push(name);
        return Ok(bndl);
    }

    path.pop();
    path.push(module);
    path.push("mod.rs");
    if path.exists() {
        let bndl = bundle(path, depth + 1, indent)?;
        path.pop();
        path.pop();
        path.push(name);
        return Ok(bndl);
    }

    panic!();
}
