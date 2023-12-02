use std::{
    fs::{self, File},
    path::Path,
};

use docx_rs::{Docx, Paragraph, Run, RunFonts};

pub fn read_as_lines(filename: String) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("file not found")
        .split('\n')
        .map(|l| l.replace('\r', "").to_string())
        .collect::<Vec<String>>()
}
pub fn save_content(filename: String, content: String) {
    fs::write(filename, content).expect("write failed");
}

pub fn write_docx(lines: &[String]) {
    let path = Path::new("current/poem.docx");
    let file = File::create(path).unwrap();
    let mut new_doc = Docx::new();
    let mut is_poem = false;
    lines.iter().for_each(|line| {
        if line.contains("【注解】") {
            is_poem = false;
        }
        let (runs, align) = if is_poem {
            (
                vec![Run::new()
                    .add_text(line.as_str())
                    .fonts(RunFonts::new().east_asia("SimSun"))
                    .size(36)],
                docx_rs::AlignmentType::Center,
            )
        } else {
            (
                vec![Run::new()
                    .add_text(line.as_str())
                    .fonts(RunFonts::new().east_asia("SimSun"))
                    .size(27)],
                docx_rs::AlignmentType::Center,
            )
        };
        if !is_poem && line.contains("====") {
            is_poem = true;
        }

        runs.into_iter().for_each(|run| {
            let para = Paragraph::new().add_run(run).align(align);
            new_doc = new_doc.clone().add_paragraph(para);
        });
    });
    new_doc.build().pack(file).unwrap();
}
