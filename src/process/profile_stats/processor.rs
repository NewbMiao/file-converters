use anyhow::{Ok, Result};
use askama::Template;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    process::Command,
};

use crate::{file::load_excel_data, Processor};

use super::cli::RunArgs;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ValueType {
    id: u32,
    profile_view: u32,
    phone_clicks: u32,
    view_gallery: u32,
}

#[derive(Template)]
#[template(path = "migration_tpl.txt")]
struct MigrationTemplate<'a> {
    sql: &'a str,
    file_name: &'a str,
}
impl Processor for RunArgs {
    type Item = ValueType;

    fn load_data(&self) -> anyhow::Result<Vec<Self::Item>> {
        let mut data = load_excel_data::<Self::Item>(self.file.as_path()).unwrap();
        // filter with target account ids
        if let (true, Some(ids)) = (self.do_filter, get_target_ids()) {
            data.retain(|v| ids.contains(&v.id));
        }
        // if self.do_filter {
        //     if let Some(ids) = get_target_ids() {
        //         data = data
        //             .into_iter()
        //             .filter_map(|v| {
        //                 if !ids.contains(&v.id) {
        //                     return None;
        //                 }
        //                 Some(v)
        //             })
        //             .collect::<Vec<Self::Item>>();
        //     }
        // }

        Ok(data)
    }

    fn generate_result_in_string(&self, data: &[Self::Item]) -> Result<String> {
        let s_type = self.s_type.as_str();
        let chunk_size = 10000;
        let sql_prefix: &str = "replace into directory_tradie_statistics (id,stats_key,stats_type,profile_views,contact_number_impressions,gallery_impressions) values ";
        let sql = data
            .par_chunks(chunk_size)
            .map(|chunk| {
                format!(
                    "{}{};",
                    sql_prefix,
                    chunk
                        .iter()
                        .map(|v| {
                            format!(
                                "({},'{}','{}',{},{},{})",
                                v.id,
                                self.key,
                                s_type,
                                v.profile_view,
                                v.phone_clicks,
                                v.view_gallery
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        Ok(sql)
    }

    fn write_data(&self, result_str: &str) -> anyhow::Result<()> {
        let tpl = MigrationTemplate {
            sql: result_str,
            file_name: &self.migration_file_name,
        };
        let output = tpl.render()?;
        let output_file = "migration_output.php";
        fs::write(output_file, output).expect("Unable to write migration file");
        println!("migration sql has been generated to file: {}", output_file);
        Ok(())
    }

    fn run_post_script(&self) -> Result<()> {
        if !self.run_post_script {
            return Ok(());
        }
        let script = "src/bin/profile_stats_post.sh";
        let output = Command::new("sh")
            .arg(script)
            .arg(&self.migration_file_name)
            .output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
        Ok(())
    }
}

pub fn get_target_ids() -> Option<Vec<u32>> {
    let contents = fs::read_to_string("id_targets.txt").unwrap_or("".to_string());
    let id: Vec<u32> = contents.lines().filter_map(|v| v.parse().ok()).collect();
    match !id.is_empty() {
        true => Some(id),
        false => None,
    }
}
