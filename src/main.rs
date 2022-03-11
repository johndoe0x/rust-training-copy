use std::env;
// This is working as internal library for the project
// please see https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
mod cli;

fn main() {
    match env::var("TRIN_COPY_PROJECT") {
        Ok(val) => cli::launch_trin(val),
        Err(_)=> println!(
            "Must spcecify Infura key as Environment variable, like:\n\
            TRIN_INFURA_PROJECT_ID=\"your_infura_project_id\" trin"
        ),
    }
}