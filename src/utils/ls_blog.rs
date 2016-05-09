use std::fs;
use std::path::{Path, PathBuf};
use std::io;


pub fn ls_html(path: &Path) -> io::Result<Vec<PathBuf>> {
    let cur_dir = fs::read_dir(path);
    match cur_dir {
        Err(err) => Err(err),
        Ok(mut entries) => {
            Ok(filter_html(&mut entries))
        }
    }
}

fn any_ext(entry: &fs::DirEntry, exts: &Vec<&str>) -> bool {
    let pathbuf = entry.path();
    let ext = pathbuf.extension();
    if let Some(ext) = ext {
        exts.iter().any(|&x| -> bool {
            ext.eq(x)
        })
    }
    else {
        false
    }
}

fn is_html(entry: &fs::DirEntry) -> bool {
    let html_exts = vec!{"html", "htm"};
    any_ext(entry, &html_exts)
}

fn filter_html(dir: &mut fs::ReadDir) -> Vec<PathBuf> {
    let mut results = Vec::new();
    for entry in dir {
        if let Ok(file) = entry {
            if is_html(&file) {
                results.push(file.path());
            }
        }
    }
    results
}

#[test]
fn test_ls_html() {
    let path = Path::new("./public/");
    let results = ls_html(&path);
    assert!(results.unwrap().len() != 0);
}
