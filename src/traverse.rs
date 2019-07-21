use super::*;
use std::collections::HashMap;
use std::path::Path;

pub fn traverse_reduce(path: impl AsRef<Path>) -> Result<String, Error> {
    use std::path::Component::*;

    let path = path.as_ref();

    let mut input = String::new();
    if path.parent().is_none() {
        input.push_str(&path.to_string_lossy());
        fix_trailing(&mut input);
        return Ok(input);
    }

    let mut buf = vec![];
    // initial
    buf.push(path.file_stem().unwrap().to_string_lossy().to_string());

    fn head(s: impl AsRef<str>) -> char {
        s.as_ref().chars().next().unwrap()
    }

    let mut path = path;
    while let Some(parent) = path.parent() {
        if let Some(RootDir) = parent.components().next() {
            break;
        }

        let base = path.file_stem().expect("valid file stem");
        path = parent;

        let map = std::fs::read_dir(&parent)
            .map_err(|err| Error {
                kind: ErrorKind::InvalidDirectory(parent.to_string_lossy().to_string()),
                inner: err.to_string(),
            })?
            .flatten()
            .filter_map(|d| d.file_type().ok().and_then(|ft| ft.is_dir().as_opt(d)))
            .filter_map(|d| {
                d.path()
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
            })
            .fold(HashMap::<char, usize>::default(), |mut map, s| {
                *map.entry(head(s).to_ascii_lowercase()).or_default() += 1;
                map
            });

        let base = base.to_string_lossy();
        let h = head(&base).to_ascii_lowercase();
        if *map.get(&h).unwrap() == 1 {
            buf.push(h.to_string());
        } else {
            buf.push(base.to_string())
        }
    }

    let mut pb = path.to_path_buf();
    for el in buf.into_iter().rev() {
        pb = pb.join(el)
    }
    input.push_str(&pb.to_string_lossy());

    fix_trailing(&mut input);
    Ok(input)
}
