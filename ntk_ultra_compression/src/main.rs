use ntk_ultra_compression::security::stegano::*;

/*
- Not Implemented (not_implemented) :
Just for developpement use and conception
*/

fn not_implemented() {
    println!("Function is not already implemented...");
}

/*
- Commands Line Parser (parse_commands_line) :
1st arg : ntk compression tool binary
2nd arg : file operation
3rd arg: file or folder input
4th arg : file output
5th and + : configuration options
*/

fn parse_commands_line() {
    let args: Vec<String> = std::env::args().collect();

    for (i, argument) in args.iter().enumerate() {
        if i == 0 {
            continue;
        } else {
            if i == 1 { // file operation treatment
                if argument == "compress" {
                    not_implemented();
                } else if argument == "extract" {
                    not_implemented();
                } else if argument == "encrypt" {
                    not_implemented();
                } else if argument == "decrypt" {
                    not_implemented();
                } else if argument == "hide" {
                    let img_path = &args[2];
                    let file_path = &args[3];
                    let output_path = format!("hidden-{}", img_path);

                    //1st arg-> input img, 2nd arg -> file to hide, 3rd -> output img
                    encode(img_path, file_path, &output_path);
                } else if argument == "unhide" {
                    let img_path = &args[2];
                    let file_path = &args[3];

                    //1st arg-> input img with hidden data, 2nd arg -> output file with data
                    decode(img_path, file_path);
                } else if argument == "help" || argument == "--help" {
                    not_implemented();
                } else {
                    println!("Option does not exist...");
                }
            }

            // if i == 2 { // file or folder input
            //     if argument.starts_with('/') {
            //         println!("Folder");
            //     } else {
            //         println!("File");
            //     }
            // }

            // if i == 3 { // file output
            //     not_implemented();
            // }

            // if i == 4 { // other configuration options
            //     not_implemented();
            // }
        }
    }
}

fn main() {
    println!("NTK Ultra-Compression");
    parse_commands_line();
}