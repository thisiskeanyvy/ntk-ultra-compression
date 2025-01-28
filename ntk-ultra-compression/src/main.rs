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
    for (ref mut i,argument) in (std::env::args()).enumerate() {
        if *i == 0 {
            *i += 1;
        } else {
            if *i == 1 { // file operation treatment
                if argument == "compress" {
                    not_implemented();
                } else if argument == "extract" {
                    not_implemented();
                } else if argument == "encrypt" {
                    not_implemented();
                } else if argument == "decrypt" {
                    not_implemented();
                } else if argument == "hide" {
                    not_implemented();
                } else if argument == "help" || argument == "--help" {
                    not_implemented();
                } else {
                    println!("Option does not exist...");
                }
            }

            if *i == 2 { // file or folder input
                if argument.chars().nth(0).unwrap() == '/' {
                    println!("Folder");
                } else {
                    println!("File");
                }
            }

            if *i == 3 { // file output
                not_implemented();
            }

            if *i == 4 { // other configuration options
                not_implemented();
            }
        }
    }
}

fn main() {
    println!("NTK Ultra-Compression");
    parse_commands_line();
}