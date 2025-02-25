//by keany-vy.khun
use std::process::Command;
use std::path::Path;
use std::fs;
use std::io::{self, Write};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn main() {
    loop {
        clear_screen();
        println!("{}
╔═══════════════════════════════════════╗
║         NTK-Manager v1.0 🚀           ║
╠═══════════════════════════════════════╣
║ 1. Installer NTK-Ultra-Compression 📦 ║
║ 2. Mettre à jour le logiciel 🔄       ║
║ 3. Lancer NTK-Ultra-Compression 🏃    ║
║ 4. Quitter ❌                         ║
╚═══════════════════════════════════════╝
{}",
            BOLD, RESET
        );

        print!("{}Choisissez une option (1-4): {}", BOLD, RESET);
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Échec de la lecture de l'entrée");

        match choice.trim() {
            "1" => install_ntk(),
            "2" => update_ntk(),
            "3" => launch_ntk(),
            "4" => {
                println!("{}Au revoir! 👋{}", GREEN, RESET);
                break;
            }
            _ => println!("{}Option invalide. Veuillez réessayer.{}", RED, RESET),
        }

        println!("\n{}Appuyez sur Entrée pour continuer...{}", DIM, RESET);
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}

fn install_ntk() {
    println!("\n{}🛠️  Installation de NTK-Ultra-Compression...{}", YELLOW, RESET);
    
    let ntk_ultra_compression_path = Path::new("ntk_ultra_compression");
    let build_path = Path::new("build");

    if ntk_ultra_compression_path.is_dir() {
        println!("{}Construction de la version release de ntk_ultra_compression...{}", YELLOW, RESET);

        let output = Command::new("cargo")
            .current_dir(ntk_ultra_compression_path)
            .args(&["build", "--release"])
            .output()
            .expect("Échec de l'exécution de cargo build --release");

        if output.status.success() {
            println!("{}✅ La construction a réussi !{}", GREEN, RESET);
            
            fs::create_dir_all(build_path).expect("Impossible de créer le dossier build");

            let exec_path = ntk_ultra_compression_path
                .join("target")
                .join("release")
                .join("ntk-ultra-compression");

            if !exec_path.exists() {
                eprintln!("{}❌ L'exécutable n'a pas été trouvé à {:?}{}", RED, exec_path, RESET);
                return;
            }

            let dest_path = build_path.join("ntk-ultra-compression");
            match fs::copy(&exec_path, &dest_path) {
                Ok(_) => println!("{}✅ Exécutable copié dans {:?}{}", GREEN, dest_path, RESET),
                Err(e) => eprintln!("{}❌ Erreur lors de la copie de l'exécutable : {}{}", RED, e, RESET),
            }

            println!("{}🧹 Nettoyage du dossier target de ntk_ultra_compression...{}", YELLOW, RESET);
            let clean_output = Command::new("cargo")
                .current_dir(ntk_ultra_compression_path)
                .arg("clean")
                .output()
                .expect("Échec de l'exécution de cargo clean dans ntk_ultra_compression");

            if clean_output.status.success() {
                println!("{}✅ Nettoyage de ntk_ultra_compression réussi !{}", GREEN, RESET);
            } else {
                eprintln!("{}❌ Erreur lors du nettoyage de ntk_ultra_compression :{}", RED, RESET);
                eprintln!("{}", String::from_utf8_lossy(&clean_output.stderr));
            }
        } else {
            eprintln!("{}❌ La construction a échoué !{}", RED, RESET);
            eprintln!("Erreur :");
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
    } else {
        eprintln!("{}❌ Le dossier ntk_ultra_compression n'existe pas !{}", RED, RESET);
    }

    println!("{}🧹 Nettoyage du dossier target à la racine...{}", YELLOW, RESET);
    let root_clean_output = Command::new("cargo")
        .arg("clean")
        .output()
        .expect("Échec de l'exécution de cargo clean à la racine");

    if root_clean_output.status.success() {
        println!("{}✅ Nettoyage à la racine réussi !{}", GREEN, RESET);
    } else {
        eprintln!("{}❌ Erreur lors du nettoyage à la racine :{}", RED, RESET);
        eprintln!("{}", String::from_utf8_lossy(&root_clean_output.stderr));
    }

    println!("\n{}✅ Installation terminée !{}", GREEN, RESET);
}

fn update_ntk() {
    println!("\n{}🔄 Mise à jour de NTK-Ultra-Compression...{}", YELLOW, RESET);
    
    let ntk_ultra_compression_path = Path::new("ntk_ultra_compression");

    if !ntk_ultra_compression_path.is_dir() {
        eprintln!("{}❌ Le dossier ntk_ultra_compression n'existe pas !{}", RED, RESET);
        return;
    }

    // Récupérer les dernières modifications du dépôt distant
    println!("{}Vérification des mises à jour...{}", YELLOW, RESET);
    let fetch_output = Command::new("git")
        .current_dir(ntk_ultra_compression_path)
        .args(&["fetch"])
        .output()
        .expect("Échec de l'exécution de git fetch");

    if !fetch_output.status.success() {
        eprintln!("{}❌ Erreur lors de la vérification des mises à jour :{}", RED, RESET);
        eprintln!("{}", String::from_utf8_lossy(&fetch_output.stderr));
        return;
    }

    let diff_output = Command::new("git")
        .current_dir(ntk_ultra_compression_path)
        .args(&["diff", "HEAD", "origin/main"])
        .output()
        .expect("Échec de l'exécution de git diff");

    if diff_output.stdout.is_empty() {
        println!("{}✅ Logiciel déjà à jour sur la dernière version !{}", GREEN, RESET);
        return;
    }

    println!("{}Mise à jour en cours...{}", YELLOW, RESET);
    let pull_output = Command::new("git")
        .current_dir(ntk_ultra_compression_path)
        .args(&["pull"])
        .output()
        .expect("Échec de l'exécution de git pull");

    if pull_output.status.success() {
        println!("{}✅ Mise à jour réussie !{}", GREEN, RESET);
    } else {
        eprintln!("{}❌ Erreur lors de la mise à jour :{}", RED, RESET);
        eprintln!("{}", String::from_utf8_lossy(&pull_output.stderr));
    }
}

fn launch_ntk() {
    println!("\n{}🚀 Lancement de NTK-Ultra-Compression...{}", YELLOW, RESET);
    println!("{}
    Pour lancer NTK-Ultra-Compression, utilisez la commande suivante dans votre terminal :
    
    ./build/ntk-ultra-compression [OPTIONS]
    
    Remplacez [OPTIONS] par les options spécifiques dont vous avez besoin.
    Consultez la documentation pour plus d'informations sur les options disponibles.
    {}", DIM, RESET);
}