use regex::Regex;
use std::fs;
use std::path::PathBuf;

use crate::file::get_file_full_path;
use crate::process::poem::file::write_docx;

use super::file::read_as_lines;
use super::file::save_content;

pub fn handle(target: String) {
    let parts = target.split(" ").collect::<Vec<_>>();
    let (title, author) = (parts[0].to_string(), parts[1].to_string());
    reset_current();
    let lines = poem_search(title.clone(), author.clone());
    let top_line = lines[0].clone();
    println!("find poem, top_line: {}", top_line);

    let audio_type = audio_search(title.clone(), author.to_string());

    let mut tips = vec![
        format!("标题:  唐诗三百首：{} - {}", title, author),
        format!("分类:  唐诗三百首 , {} , {}", author, audio_type),
    ];
    let other_tips = read_as_lines("fixtures/poem_tips.txt".to_string());
    tips.extend_from_slice(&other_tips[..]);
    tips.extend_from_slice(&lines[..]);
    write_docx(&tips);

    mark_as_done(format!("{} {}", title, author));
}
fn mark_as_done(target: String) {
    let filename = "fixtures/titles.txt";
    let rows = read_as_lines(filename.to_string());
    let line = rows
        .iter()
        .find(|line| line.contains(&target))
        .expect("not found target in titles.txt to mark as done");
    if line.contains("done") {
        return;
    }
    let handled_line = format!("{} done", line);
    let updated = rows.join("\n").replace(line, &handled_line);
    save_content(filename.to_string(), updated);
}
pub fn get_next_target() -> String {
    let filename = "fixtures/titles.txt";
    let rows = read_as_lines(filename.to_string());
    rows.iter()
        .find(|line| !line.contains("done"))
        .expect("no more title")
        .to_string()
}

fn poem_search(title: String, author: String) -> Vec<String> {
    let filename = "fixtures/poems.txt";

    let rows = read_as_lines(filename.to_string());

    let mut lines: Vec<String> = Vec::new();
    let mut is_poem = false;

    for decoded_string in rows {
        if decoded_string.contains(format!("《{}》 {}", title, author).as_str()) {
            is_poem = true;
        }

        if decoded_string.contains("====") && is_poem {
            break;
        }
        if is_poem {
            lines.push(decoded_string);
        }
    }
    lines
}

fn reset_current() {
    if !PathBuf::from("current").exists() {
        fs::create_dir("current").expect("Failed to create folder");
    }
    let paths = fs::read_dir("current").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            fs::remove_file(path).expect("Failed to remove file");
        }
    }
}
fn audio_search(title: String, author: String) -> String {
    let fullpath = get_file_full_path("~/Documents/poem");
    let formatted_title = title
        .replace("·", "")
        .split(',')
        .take(2)
        .collect::<String>();

    let audio = glob::glob(&format!(
        "{}/**/*{}*.mp3",
        fullpath.unwrap().to_str().unwrap(),
        formatted_title
    ))
    .unwrap();
    let re_with_author = Regex::new(&format!(r"\d+{}（{}）.mp3", formatted_title, author)).unwrap();
    let re_without_author = Regex::new(&format!(r"\d+{}.mp3", formatted_title)).unwrap();

    let src = audio
        .into_iter()
        .find(|v| {
            let filename = v.as_ref().unwrap().file_name().unwrap().to_str().unwrap();
            re_with_author.is_match(filename) || re_without_author.is_match(filename)
        })
        .unwrap_or_else(|| {
            panic!("no audio found for {}", title);
        })
        .unwrap();
    println!("find audio src: {:?}", src.as_path().to_str().unwrap());
    let dst = format!("current/{}-{}.mp3", title, author);
    fs::copy(src.clone(), dst).expect("Failed to copy audio file");
    let audio_type_folder = src
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    Regex::new(r"[-|\d]+")
        .unwrap()
        .replace_all(&audio_type_folder, "")
        .to_string()
}
