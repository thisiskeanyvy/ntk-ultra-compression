# NTK Ultra-Compression

Un logiciel de compression de données avancé utilisant des techniques de compression innovantes.

## Caractéristiques

- Interface graphique moderne et intuitive
- Compression de données haute performance
- Support du multi-threading
- Chiffrement AES-256 optionnel
- Détection automatique des fichiers compressés
- Visualisation des métadonnées de compression
- Support multi-plateforme (Windows, macOS, Linux)

## Installation

### Prérequis

- Rust (édition 2021 ou supérieure)
- Node.js (v16 ou supérieure)
- npm ou yarn
- Dépendances système pour Tauri (voir [documentation Tauri](https://tauri.app/v1/guides/getting-started/prerequisites))

### Compilation depuis les sources

1. Clonez le dépôt :
```bash
git clone https://github.com/votre-username/ntk-ultra-compression.git
cd ntk-ultra-compression
```

2. Installez les dépendances et compilez :
```bash
# Installation des dépendances frontend
cd gui
npm install

# Compilation du projet
npm run tauri build
```

Les exécutables compilés seront disponibles dans le dossier `src-tauri/target/release`.

## Utilisation

1. Lancez l'application NTK Ultra-Compression
2. Sélectionnez le fichier à compresser/décompresser
3. Choisissez l'emplacement de sortie
4. Ajustez les options de compression si nécessaire :
   - Niveau de compression (1-9)
   - Activation du chiffrement
   - Mot de passe (si chiffrement activé)
5. Cliquez sur "Compresser" ou "Décompresser"

## Architecture du projet

```
ntk-ultra-compression/
├── core/                    # Bibliothèque core de compression
│   ├── src/                # Code source Rust
│   └── Cargo.toml          # Manifeste du package core
├── gui/                    # Interface graphique Tauri
│   ├── src/               # Code source React/TypeScript
│   ├── src-tauri/        # Code source Rust pour Tauri
│   └── package.json      # Configuration npm
└── Cargo.toml             # Manifeste workspace Rust
```

## Algorithme de compression

NTK Ultra-Compression utilise une combinaison d'algorithmes pour obtenir des taux de compression optimaux :

1. Prétraitement des données
2. Analyse de redondance
3. Compression par dictionnaire
4. Compression entropique

## Sécurité

- Chiffrement AES-256 en mode GCM
- Dérivation de clé sécurisée (PBKDF2)
- Vérification d'intégrité (BLAKE3)

## Licence

Ce projet est sous licence BSD-3-Clause. Voir le fichier `LICENSE` pour plus de détails.

## Auteurs

- Nathan Pelletti
- Thomas Demesse
- Keany Vy Khun
- Litissia Ben Mohand

## Contribution

Les contributions sont les bienvenues ! Veuillez consulter notre guide de contribution avant de soumettre une pull request. 