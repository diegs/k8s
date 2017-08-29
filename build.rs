use std::fs;
use std::path;

extern crate glob;
extern crate protoc_rust;

fn main() {
    let paths = glob::glob("k8s.io/api/**/*.proto").expect("glob");
    for p in paths {
        let p = p.expect("path");
        let mut out = path::PathBuf::from("src");
        out.push(p.parent().expect("parent"));
        let out = out.to_str().expect("str");
        fs::create_dir_all(&out).expect("create_dir_all");
        protoc_rust::run(protoc_rust::Args {
            out_dir: &out,
            input: &[&p.to_str().expect("str")],
            includes: &[],
        }).expect("protoc");
    }
}
