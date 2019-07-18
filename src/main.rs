#![allow(dead_code)]

use gumdrop::Options;
use lexical_bool::LexicalBool;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone, Options, Debug)]
struct Args {
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "display current git branch")]
    git_branch: bool,

    #[options(help = "attempt to make the components unique")]
    unique: bool,

    #[options(help = "output as json", meta = "bool", default = "true")]
    json: LexicalBool,

    #[options(free, required)]
    path: String,
}

fn main() {
    let args = Args::parse_args_default_or_exit();

    let path = if args.unique {
        shorten(&args.path, travese_reduce)
    } else {
        shorten(&args.path, simple_reduce)
    };

    let branch = if args.git_branch {
        get_git_branch().unwrap_or_default()
    } else {
        "".into()
    };

    if *args.json {
        println!(
            "{}",
            serde_json::to_string(&serde_json::json!({
                "path": path,
                "branch": branch.trim(),
            }))
            .expect("valid json")
        );
        return;
    }

    println!("{}", path);
    print!("{}", branch);
}

fn simple_reduce(input: &mut String, path: &Path) {
    use std::path::Component::*;
    let parts: Vec<_> = path
        .components()
        .filter_map(|k| match k {
            Normal(p) => Some(p.to_string_lossy()),
            Prefix(p) => Some(p.as_os_str().to_string_lossy()),
            _ => None,
        })
        .collect();

    if parts.is_empty() {
        std::process::exit(1)
    }

    let len = parts.len();

    let mut iter = parts.iter();
    input.push_str(&iter.next().unwrap());
    input.push(std::path::MAIN_SEPARATOR);

    if len > 2 {
        for part in iter.take(parts.len() - 2) {
            for ch in part.chars().next() {
                input.push(ch);
            }
            input.push(std::path::MAIN_SEPARATOR);
        }
    }

    if len > 1 {
        input.push_str(&parts.last().unwrap());
    }
}

fn travese_reduce(input: &mut String, path: &Path) {
    let mut buf = vec![];
    use std::path::Component::*;

    // initial
    buf.push(path.file_stem().unwrap().to_string_lossy().to_string());

    fn unique_other(path: &Path, buf: &mut Vec<String>) {
        // panics if everything isn't right
        fn head(s: impl AsRef<str>) -> char {
            s.as_ref().chars().next().unwrap()
        }

        let (base, parent) = match path
            .file_name()
            .and_then(|base| path.parent().map(|parent| (base, parent)))
        {
            Some((base, parent)) => (base, parent),
            None => return,
        };

        let map = std::fs::read_dir(&parent)
            .expect("valid directory")
            .flatten()
            .filter_map(|d| d.file_type().ok().and_then(|ft| ft.is_dir().as_opt(d)))
            .filter_map(|d| {
                d.path()
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
            })
            .fold(HashMap::<char, usize>::default(), |mut map, s| {
                *map.entry(head(s)).or_default() += 1;
                map
            });

        let base = base.to_string_lossy();
        let h = head(&base);
        if *map.get(&h).unwrap() == 1 {
            buf.push(h.to_string());
        } else {
            buf.push(base.to_string())
        }
    }

    let mut path = path;
    while let Some(parent) = path.parent() {
        if let Some(comp) = parent.components().next() {
            if let RootDir = comp {
                break;
            }
        }
        path = parent;
        unique_other(&parent, &mut buf)
    }

    let mut pb = path.to_path_buf();
    for el in buf.into_iter().rev() {
        pb = pb.join(el)
    }
    input.push_str(&pb.to_string_lossy());
}

fn shorten<P, F>(path: P, reduce: F) -> String
where
    P: AsRef<Path>,
    F: Fn(&mut String, &Path),
{
    let mut string = String::new();
    reduce(&mut string, path.as_ref());
    string
}

fn get_git_branch() -> Option<String> {
    std::process::Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|s| String::from_utf8(s.stdout).ok())
}

trait BoolAsOptional {
    fn as_opt<E>(&self, el: E) -> Option<E>;
}

impl BoolAsOptional for bool {
    fn as_opt<E>(&self, el: E) -> Option<E> {
        if *self {
            Some(el)
        } else {
            None
        }
    }
}
