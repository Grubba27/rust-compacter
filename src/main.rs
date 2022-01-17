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

pub fn zip_file(text: String, file_text: &mut String) {
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

pub fn unzip_file(text: String, file_text: &mut String) {
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


#[cfg(test)]
mod test_module {
    use crate::{unzip_file, zip_file};

    #[test]
    pub fn zip_file_test() {
        let mut file_text: String = String::from("");
        let text = std::fs::read_to_string("src/text.txt").expect("File is wrong");
        zip_file(text, &mut file_text);
        assert_eq!(file_text, "Violet5121azuis0Ros5121vermelh50M41teclado1317á1funcionando0Ou1será1que141317612?0", "text.txt should look like this zip string");
    }

    #[test]
    pub fn unzip_file_test() {
        let mut file_text: String = String::from("");
        let text = std::fs::read_to_string("src/zip.txt").expect("File is wrong");
        unzip_file(text, &mut file_text);
        assert_eq!(file_text, "Violetas são azuis
Rosas são vermelhas
Meu teclado não está funcionando
Ou será que eu não estou são?
", "Text should always look like this string")
    }
}