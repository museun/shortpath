fn main() {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => std::process::exit(1),
    };

    use std::path::{Component::*, Path};
    let parts: Vec<_> = Path::new(&path)
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

    let mut iter = parts.iter();
    print!("{}/", iter.next().unwrap());
    if parts.len() > 2 {
        iter.take(parts.len() - 2)
            .filter_map(|k| k.chars().next())
            .for_each(|c| print!("{}/", c));
    }
    if parts.len() > 1 {
        print!("{}", parts.last().unwrap())
    }
}
