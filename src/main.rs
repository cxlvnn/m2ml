use std::env;
use std::fs;
use std::process;

struct File {
    path: String,
    contents: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("file path is not provided");
        process::exit(1);
    }
    let file_path = &args[1];
    let file: File = File::read_file(file_path);

    println!("reading file at: {}", file.path);
    println!("file contents: \n{}", file.contents);
}

impl File {
    fn read_file(path: &str) -> File {
        let contents = fs::read_to_string(path).expect("Could not read the file");

        File {
            path: path.to_string(),
            contents,
        }
    }
}
