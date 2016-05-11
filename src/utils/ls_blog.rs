use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use utils;
use std::sync::{Arc, RwLock};
use serde_json;
use serde_json::Value;
use std::collections::BTreeMap;
use std::time::SystemTime;

///
/// list blogs in config.blog
/// return a json string
///
pub fn ls_blogs() -> Option<String> {
    let mut ret = None;
    let need_update = match G_BLOGS.read() {
        Ok(blogs) => {
            let now = SystemTime::now();
            let last_time = blogs.last_modify_time;
            let interval = now.duration_since(last_time);
            match interval {
                Ok(interval) => {
                    let out_date = interval.as_secs() > 5 * 60u64;
                    if !out_date {
                        ret = blogs.list.as_ref().clone();
                    }
                    out_date
                },
                Err(_) => false,
            }
        },
        Err(_) => false,
    };
    if need_update {
        println!("update blog");
        let blog_path = utils::load_config::blog_path();
        let blog_path = Path::new(&blog_path);
        let path_list = ls_html(&blog_path).unwrap();
        ret = paths_to_json_str(&path_list);
        let result = G_BLOGS.write();
        if let Ok(mut blogs) = result {
            println!("write to blogs");
            blogs.list = Arc::new(ret.clone()).clone();
        }
    }
    ret
}

fn pathbuf_to_json(pb: &PathBuf) -> Option<Value> {
    let fullname = pb.file_name()
        .and_then(|s| s.to_str())
        .map(|os_str| os_str.to_string());
    let filename = pb.file_stem()
        .and_then(|s| s.to_str())
        .map(|os_str| os_str.to_string());
    
    if let Some(full) = fullname {
        if let Some(file) = filename {
            let mut map = BTreeMap::new();
            map.insert("url".to_string(), Value::String(full));
            map.insert("name".to_string(), Value::String(file));
            return Some(Value::Object(map));
        }
    }
    None
}

fn paths_to_json_str(files: &Vec<PathBuf>) -> Option<String> {
    let path_strs: Vec<Value> = files.iter()
                                      .filter_map(|pb| {
                                          pathbuf_to_json(&pb)
                                      })
                                      .collect();
    let s = serde_json::to_string(&path_strs);
    s.ok()
}


fn ls_html(path: &Path) -> io::Result<Vec<PathBuf>> {
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

lazy_static! {
    static ref G_BLOGS: RwLock<BlogsKeeper> = RwLock::new(BlogsKeeper::new());
}

type BKValue = Option<String>;

struct BlogsKeeper {
    list: Arc<BKValue>,
    last_modify_time: SystemTime,
}

impl BlogsKeeper {
    fn new() -> BlogsKeeper {
        let blog_path = utils::load_config::blog_path();
        let blog_path = Path::new(&blog_path);
        let path_list = ls_html(&blog_path).unwrap();
        let json = paths_to_json_str(&path_list);
        BlogsKeeper {
            list: Arc::new(json).clone(),
            last_modify_time: SystemTime::now(),
        }
    }
}

#[test]
fn test_ls_html() {
    let path = Path::new("./public/");
    let results = ls_html(&path);
    assert!(results.unwrap().len() != 0);
}
