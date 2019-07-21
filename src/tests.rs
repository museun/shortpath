use super::*;

#[test]
fn traverse_at_root_dir() {
    assert_eq!(traverse_reduce("c:/").unwrap(), "c:/");
    assert_eq!(traverse_reduce("D:/").unwrap(), "D:/");
    assert_eq!(traverse_reduce("c:\\").unwrap(), "c:/");
    assert_eq!(traverse_reduce("D:\\").unwrap(), "D:/");
    assert_eq!(traverse_reduce("c:").unwrap(), "c:/");
    assert_eq!(traverse_reduce("D:").unwrap(), "D:/");
}

#[test]
fn simple_at_root_dir() {
    assert_eq!(simple_reduce("c:/").unwrap(), "c:/");
    assert_eq!(simple_reduce("D:/").unwrap(), "D:/");
    assert_eq!(simple_reduce("c:\\").unwrap(), "c:/");
    assert_eq!(simple_reduce("D:\\").unwrap(), "D:/");
    assert_eq!(simple_reduce("c:").unwrap(), "c:/");
    assert_eq!(simple_reduce("D:").unwrap(), "D:/");
}

#[test]
fn simple_nested() {
    let paths = &[
        ("c:/", "c:/"),
        ("c:/dev/", "c:/dev/"),
        ("c:/dev", "c:/dev/"),
        ("c:/dev/foo", "c:/d/foo/"),
        ("c:/dev/foo/bar", "c:/d/f/bar/"),
        ("c:/dev/foo/bar/baz and", "c:/d/f/b/baz and/"),
        ("c:/dev/foo/bar/baz and/q u u x", "c:/d/f/b/b/q u u x/"),
    ];

    for (path, expected) in paths {
        assert_eq!(simple_reduce(path).unwrap(), *expected);
    }
}

// TODO: need a temp fs to test the file traversal
