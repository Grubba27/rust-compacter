use std::fs::File;
use std::io::Write;
use clap::Parser;

#[derive(Parser)]
struct Args {
    command: String,

    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();

    let mut file_text: String = String::from("");
    let text = std::fs::read_to_string(&args.path).expect("File is wrong");

    if args.command == "zip" {
        zip_file(text, &mut file_text);
    } else if args.command == "unzip" {
        unzip_file(text, &mut file_text);
    }


    let file_path = format!("src/{}.txt", args.command);
    let mut file = File::create(file_path)?;
    file.write_all(&file_text.as_ref())
}

fn zip_file(text: String, file_text: &mut String) {
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
}

fn unzip_file(text: String, file_text: &mut String) {
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
}