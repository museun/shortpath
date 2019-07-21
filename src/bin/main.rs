use gumdrop::Options;
use shortpath::*;

fn main() {
    let args = Args::parse_args_default_or_exit();

    let path = match if args.unique {
        traverse_reduce(&args.path)
    } else {
        simple_reduce(&args.path)
    } {
        Ok(path) => path,
        Err(err) => {
            if *args.json {
                let err = serde_json::json!({
                    "error": err.kind.to_string(),
                    "inner": err.inner,
                })
                .to_string();
                eprintln!("{}", err);
            } else {
                eprintln!("{}", err);
            }
            std::process::exit(1);
        }
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
