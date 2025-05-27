# Guide d'installation de NTK Ultra-Compression

Ce guide détaille l'installation de NTK Ultra-Compression sur différents systèmes d'exploitation.

## Prérequis système

### Configuration minimale
- Processeur : Double cœur 2 GHz
- RAM : 4 Go
- Espace disque : 500 Mo
- Système d'exploitation :
  - Windows 10 ou plus récent
  - macOS 10.15 ou plus récent
  - Linux avec glibc 2.31 ou plus récent

### Configuration recommandée
- Processeur : Quad core 3 GHz
- RAM : 8 Go
- Espace disque : 1 Go
- GPU : Compatible OpenGL 3.3+

## Installation des dépendances

### Windows

1. Installez [Rust](https://rustup.rs/)
2. Installez [Node.js](https://nodejs.org/) (LTS)
3. Installez les Visual Studio Build Tools :
```powershell
winget install Microsoft.VisualStudio.2022.BuildTools
```
4. Installez WebView2 :
```powershell
winget install Microsoft.EdgeWebView2Runtime
```

### macOS

1. Installez Xcode Command Line Tools :
```bash
xcode-select --install
```

2. Installez Homebrew :
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

3. Installez les dépendances :
```bash
brew install rust node
```

### Linux (Debian/Ubuntu)

1. Installez les dépendances système :
```bash
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev \
    libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev \
    librsvg2-dev curl
```

2. Installez Rust :
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

3. Installez Node.js :
```bash
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt install -y nodejs
```

### Linux (Fedora)

```bash
sudo dnf install rust cargo nodejs webkit2gtk3-devel \
    openssl-devel gtk3-devel libappindicator-gtk3-devel \
    librsvg2-devel
```

### Linux (Arch)

```bash
sudo pacman -S rust nodejs npm webkit2gtk base-devel \
    openssl gtk3 libappindicator-gtk3 librsvg
```

## Compilation depuis les sources

1. Clonez le dépôt :
```bash
git clone https://github.com/thisiskeanyvy/ntk-ultra-compression.git
cd ntk-ultra-compression
```

2. Installez les dépendances du projet :
```bash
# Installation des dépendances frontend
cd gui
npm install
```

3. Compilez le projet :
```bash
# Mode développement
npm run tauri dev

# Mode production
npm run tauri build
```

Les exécutables compilés seront disponibles dans :
- Windows : `gui/src-tauri/target/release/ntk-ultra-compression.exe`
- macOS : `gui/src-tauri/target/release/ntk-ultra-compression.app`
- Linux : `gui/src-tauri/target/release/ntk-ultra-compression`

## Installation des binaires précompilés

### Windows
1. Téléchargez le dernier installateur `.msi` depuis la page des releases
2. Exécutez l'installateur
3. Suivez les instructions à l'écran

### macOS
1. Téléchargez le fichier `.dmg` depuis la page des releases
2. Montez l'image disque
3. Glissez l'application dans le dossier Applications

### Linux
1. Téléchargez le paquet correspondant à votre distribution :
   - Debian/Ubuntu : `.deb`
   - Fedora : `.rpm`
   - Arch : Disponible sur AUR
2. Installez le paquet :
   ```bash
   # Debian/Ubuntu
   sudo dpkg -i ntk-ultra-compression.deb
   
   # Fedora
   sudo dnf install ntk-ultra-compression.rpm
   
   # Arch (via AUR)
   yay -S ntk-ultra-compression
   ```

## Vérification de l'installation

1. Lancez l'application :
   - Windows : Menu Démarrer > NTK Ultra-Compression
   - macOS : Launchpad > NTK Ultra-Compression
   - Linux : Menu des applications > NTK Ultra-Compression

2. L'interface graphique devrait s'ouvrir sans erreur

## Résolution des problèmes

### Windows
- Erreur WebView2 : Réinstallez le runtime WebView2
- Erreur VCRUNTIME : Installez Visual C++ Redistributable

### macOS
- "App endommagée" : Ouvrez les Préférences Système > Sécurité et confidentialité
- Problèmes de permissions : `xattr -cr /Applications/NTK\ Ultra-Compression.app`

### Linux
- Erreur de bibliothèque : `sudo ldconfig`
- Erreur WebKit : Installez les paquets webkit2gtk-4.0

## Support

Pour plus d'aide :
- Consultez notre [FAQ](docs/FAQ.md)
- Ouvrez une issue sur GitHub
- Contactez le support technique 