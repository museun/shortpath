use gumdrop::Options;
use lexical_bool::LexicalBool;

#[derive(Clone, Options, Debug)]
struct Args {
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "display current git branch")]
    git_branch: bool,

    #[options(help = "output as json", meta = "bool", default = "true")]
    json: LexicalBool,

    #[options(free, required)]
    path: String,
}

fn main() {
    let args = Args::parse_args_default_or_exit();

    use std::path::{Component::*, Path};
    let parts: Vec<_> = Path::new(&args.path)
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

    let mut string = String::new();

    let mut iter = parts.iter();
    string.push_str(iter.next().unwrap());
    string.push('/');

    if parts.len() > 2 {
        iter.take(parts.len() - 2)
            .filter_map(|k| k.chars().next())
            .for_each(|c| {
                string.push(c);
                string.push('/');
            });
    }
    if parts.len() > 1 {
        string.push_str(&parts.last().unwrap());
    }

    let branch = if args.git_branch {
        get_git_branch().unwrap_or_default()
    } else {
        "".into()
    };

    if *args.json {
        println!(
            "{}",
            serde_json::to_string(&serde_json::json!({
                "path": string,
                "branch": branch.trim(),
            }))
            .expect("valid json")
        );
    } else {
        println!("{}", string);
        println!("{}", branch);
    }
}

fn get_git_branch() -> Option<String> {
    std::process::Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|s| String::from_utf8(s.stdout).ok())
}
