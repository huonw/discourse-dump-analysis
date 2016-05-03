extern crate regex;
use regex::Regex;
use std::io::prelude::*;
use std::fs::File;

const FROM_1_0_DATE: &'static str = "201(?:6-|5-(?:1|0(?:[6-9]|5-(?:2|1[5-9]))))";

fn load_file(name: &str) -> Option<String> {
    let mut f = match File::open(name) {
        Ok(f) => f,
        Err(_) => return None
    };
    let length = f.metadata().unwrap().len() as usize;
    let mut v = vec![0; length];
    f.read_exact(&mut v).unwrap();
    Some(String::from_utf8(v).unwrap())
}

fn find_region<'a>(name: &str, data: &'a str) -> &'a str
{
    let start = data.find(format!("COPY {} ", name).as_str()).expect(&format!("couldn't find {}'", name));
    let end = data[start..].find("\n\\.").expect(&format!("couldn't find {}'", name));

    &data[start..start+end+4]
}

fn cached_region(name: &str, data: &str) -> String {
    let fname = format!("{}.sql", name);
    match load_file(&fname) {
        Some(s) => s,
        None => {
            let s = find_region(name, data);
            File::create(&fname).unwrap().write(s.as_bytes()).unwrap();
            s.to_owned()
        }
    }

}

fn count_posts(data: &str) {
    let posts = cached_region("posts", data);
    let date_filter = Regex::new(&format!(r"(?m)^.*{}", FROM_1_0_DATE)).unwrap();
    println!("posts: {}", date_filter.find_iter(&posts).count());

}

fn count_likes(data: &str) {
    let likes = cached_region("given_daily_likes", data);
    let line_filter = Regex::new(&format!(r"(?m)^.*\s(\d+)\s*{}", FROM_1_0_DATE)).unwrap();

    let mut sum = 0;
    for c in line_filter.captures_iter(&likes) {
        let integer = c.at(1).unwrap().parse().unwrap();
        sum += integer;
    }
    println!("likes: {}", sum);
}


fn main() {
    let s = load_file("dump.sql").unwrap();
    count_posts(&s);
    count_likes(&s);
}
