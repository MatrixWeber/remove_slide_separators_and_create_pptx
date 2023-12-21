use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::process::Command;
use std::io::prelude::*;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long,  default_value = "../../rust_training.md")]
    file: String,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let file_content = read_md_file(&args.file)?;
    if !file_content.is_empty()
    {
        let separators = vec!["-s-", "-v-", "[<](#/)"];
        let file_content_after_replacement = remove_separators(file_content, separators);
        println!("{}", file_content_after_replacement);

        create_new_md_file_for_pptx("rust_training_for_pptx.md", file_content_after_replacement)?;

        execute_a_bash_command("pandoc --from markdown --to pptx ".to_owned() + "rust_training_for_pptx.md" + " -o rust_training.pptx")?;
    }

    Ok(())
}

fn execute_a_bash_command(command: String) -> Result<(), Error> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()?;

    if !output.stdout.is_empty()
    {
        let output = String::from_utf8_lossy(&output.stdout);
        println!("Output: {}", output);
    }
    Ok(())
}

fn create_new_md_file_for_pptx(file_name: &str, file_content_after_replacement: String) -> Result<(), Error> {
    let mut md_file_for_pptx = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)?;
    md_file_for_pptx.write((file_content_after_replacement).as_ref())?;

    Ok(())
}

fn remove_separators(file_content: String, separators: Vec<&str>) -> String {
    let mut file_content_after_replacement = file_content;
    for separator in separators
    {
        file_content_after_replacement = file_content_after_replacement.replace(&separator, "");
    }

    file_content_after_replacement
}

fn read_md_file(file_path: &String) -> Result<String, Error> {
    let mut md_file = File::open(file_path)?;
    let mut file_content = String::new();
    md_file.read_to_string(&mut file_content)?;
    Ok(file_content)
}