use std::fs::File;
use std::io::Write;
use clap::Parser;

#[derive(Parser)]
struct Args {
    command: String,

    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() {
    let args: Args = Args::parse();
    if args.command == "zip" { zip_file(&args); }
    if args.command == "unzip" { unzip_file(&args); }
}

fn zip_file(args: &Args) -> std::io::Result<()> {
    let text = std::fs::read_to_string(&args.path).expect("File is wrong");
    let mut file_text: String = String::from("");
    for line in text.lines() {
        let compact = line
            .replace(" ", "1")
            .replace("são", "2")
            .replace("não", "3")
            .replace("eu", "4")
            .replace("as", "5")
            .replace("ou", "6")
            .replace("est", "7");
        let result = compact + "0";
        file_text.push_str(&*result);
    }
    let mut file = File::create("src/zip.txt")?;
    file.write_all(file_text.as_ref());
    Ok(())
}

fn unzip_file(args: &Args) -> std::io::Result<()> {
    let text = std::fs::read_to_string(&args.path).expect("File is wrong");
    let mut file_text: String = String::from("");
    for line in text.lines() {
        let compact = line
            .replace("0", "\n")
            .replace("1", " ")
            .replace("2", "são")
            .replace("3", "não")
            .replace("4", "eu")
            .replace("5", "as")
            .replace("6", "ou")
            .replace("7", "est");
        file_text.push_str(&*compact);
    }
    let mut file = File::create("src/unzip.txt")?;
    file.write_all(file_text.as_ref());
    Ok(())
}