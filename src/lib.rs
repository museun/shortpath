mod args;
pub use args::Args;

mod error;
pub use error::{Error, ErrorKind};

mod simple;
mod traverse;
pub use {simple::simple_reduce, traverse::traverse_reduce};

#[cfg(test)]
mod tests;

pub fn get_git_branch() -> Option<String> {
    std::process::Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|s| String::from_utf8(s.stdout).ok())
}

pub trait BoolAsOptional {
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

pub(crate) fn fix_trailing(input: &mut String) {
    debug_assert!(!input.is_empty());

    // this is a hack
    if let Some(index) = input.rfind('\\') {
        input.truncate(index);
    }

    let mut bad = vec![];
    for (i, ch) in input.char_indices() {
        if ch == '\\' {
            bad.push(i);
        }
    }

    for bad in bad {
        input.replace_range(bad..bad + 1, "/");
    }

    if !input.ends_with('/') {
        input.push('/');
    }
}
