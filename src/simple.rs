use super::*;
use std::path::Path;

pub fn simple_reduce(path: impl AsRef<Path>) -> Result<String, Error> {
    let mut input = String::new();

    let path = path.as_ref();

    if path.parent().is_none() {
        input.push_str(&path.to_string_lossy());
        fix_trailing(&mut input);
        return Ok(input);
    }

    // TODO is this needed?
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
        debug_assert!(!input.is_empty());
        unimplemented!("this should be an error");
    }

    let len = parts.len();

    let mut iter = parts.iter();
    input.push_str(&iter.next().unwrap());
    input.push('/');

    if len > 2 {
        for part in iter.take(parts.len() - 2) {
            for ch in part.chars() {
                input.push(ch);
            }
            input.push('/');
        }
    }

    if len > 1 {
        input.push_str(&parts.last().unwrap());
    }

    fix_trailing(&mut input);
    Ok(input)
}
