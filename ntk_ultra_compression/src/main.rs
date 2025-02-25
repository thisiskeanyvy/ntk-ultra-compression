//by keany-vy.khun
use ntk_ultra_compression::security::stegano::*;
use ntk_ultra_compression::bzip2_compression::Bzip2::*;
use std::process;
use std::io::{self, Write};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
//const DIM: &str = "\x1b[2m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

/*
- Not Implemented (not_implemented) :
Just for developpement use and conception
*/

fn not_implemented() {
    println!("{}Cette fonction n'est pas encore implémentée...{}", YELLOW, RESET);
}

fn help_menu() {
    println!("{}
╔═══════════════════════════════════════════════════════════════════════════╗
║                    Menu d'aide NTK Ultra-Compression                      ║
╠═══════════════════════════════════════════════════════════════════════════╣
║ Utilisation: ntk-ultra-compression <COMMANDE> [OPTIONS]                   ║
║                                                                           ║
║ Commandes:                                                                ║
║   compress <fichier_entree>             Compresser le fichier d'entrée    ║
║   extract <fichier_entree>              Extraire le fichier compressé     ║
║   encrypt <fichier_entree>              Chiffrer le fichier d'entrée      ║
║   decrypt <fichier_entree>              Déchiffrer le fichier d'entrée    ║
║   hide <fichier_image> <fichier>        Cacher un fichier dans une image  ║
║   unhide <fichier_image> <sortie>       Extraire le fichier caché         ║
║   help                                  Afficher ce message d'aide        ║
║                                                                           ║
║ Options:                                                                  ║
║   Des options supplémentaires peuvent être spécifiées après la commande.  ║
║                                                                           ║
║ Exemples:                                                                 ║
║   ntk compress monfichier.txt                                             ║
║   ntk extract monarchive.ntk                                              ║
╚═══════════════════════════════════════════════════════════════════════════╝
    {}", BOLD, RESET);
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

    if args.len() < 2 {
        help_menu();
        return;
    }

    match args[1].as_str() {
        "compress" => {
            if args.len() < 3 {
                println!("{}Erreur : Aucun fichier d'entrée spécifié pour la compression.{}", RED, RESET);
                process::exit(1);
            }
            println!("{}Compression du fichier en cours...{}", YELLOW, RESET);
            compress(&args[2]);
            println!("{}Compression terminée avec succès.{}", GREEN, RESET);
        },
        "extract" => {
            if args.len() < 3 {
                println!("{}Erreur : Aucun fichier d'entrée spécifié pour l'extraction.{}", RED, RESET);
                process::exit(1);
            }
            println!("{}Extraction du fichier en cours...{}", YELLOW, RESET);
            let file_extension = args[2].split('.').last().unwrap_or("");
            decompress(&args[2], file_extension);
            println!("{}Extraction terminée avec succès.{}", GREEN, RESET);
        },
        "encrypt" => not_implemented(),
        "decrypt" => not_implemented(),
        "hide" => {
            if args.len() < 4 {
                println!("{}Erreur : Arguments manquants pour l'opération de dissimulation.{}", RED, RESET);
                process::exit(1);
            }
            println!("{}Dissimulation du fichier dans l'image...{}", YELLOW, RESET);
            let output_path = format!("hidden-{}", args[2]);

            //1st arg-> input img, 2nd arg -> file to hide, 3rd -> output img
            encode(&args[2], &args[3], &output_path);
            println!("{}Fichier dissimulé avec succès.{}", GREEN, RESET);
        },
        "unhide" => {
            if args.len() < 4 {
                println!("{}Erreur : Arguments manquants pour l'opération de révélation.{}", RED, RESET);
                process::exit(1);
            }
            println!("{}Extraction du fichier caché depuis l'image...{}", YELLOW, RESET);

            //1st arg-> input img with hidden data, 2nd arg -> output file with data
            decode(&args[2], &args[3]);
            println!("{}Fichier caché extrait avec succès.{}", GREEN, RESET);
        },
        "help" | "--help" => help_menu(),
        _ => {
            println!("{}Erreur : Commande inconnue '{}'.{}", RED, args[1], RESET);
            help_menu();
            process::exit(1);
        }
    }
}

fn main() {
    clear_screen();
    println!("{}NTK Ultra-Compression 1.0.0 ( https://ntk-ultra-compression.pages.dev/ ){}", BOLD, RESET);
    parse_commands_line();
}