# Documentation technique NTK Ultra-Compression

## Architecture

- [Vue d'ensemble](architecture.md)
- [Core](core/README.md)
- [GUI](gui/README.md)
- [Tests](tests/README.md)

## Composants principaux

### Core (Rust)
- Compression/décompression (zstd)
- Chiffrement (AES-256-GCM)
- Stéganographie (LSB)
- Gestion des fichiers

### GUI (Tauri + React)
- Interface utilisateur
- Communication IPC
- Gestion des événements
- Traitement asynchrone

## Formats

### Format de fichier .ntk
```
[HEADER - 512 bytes]
  - Magic bytes "NTK1"
  - Version
  - Flags (encrypted, etc.)
  - Original filename
  - Original size
  - Metadata (JSON)
[SALT - 16 bytes] (if encrypted)
[NONCE - 12 bytes] (if encrypted)
[DATA]
  - Compressed (and encrypted) blocks
```

### Format stéganographie
```
[PNG HEADER]
[CAPACITY - 32 bits]
[ARCHIVE SIZE - 32 bits]
[ARCHIVE DATA - LSB encoded]
[ORIGINAL IMAGE DATA]
```

## Protocoles

### Compression
1. Lecture du fichier source
2. Division en blocs de 16MB
3. Compression zstd par bloc
4. Chiffrement (optionnel)
5. Écriture du fichier destination

### Stéganographie
1. Vérification de la capacité
2. Encodage de la taille
3. Encodage LSB des données
4. Préservation des données image

## API

### Core
- `Compressor::new()`
- `compress()`
- `decompress()`
- `hide_in_image()`
- `extract_from_image()`

### IPC (Tauri)
- `compress`
- `decompress`
- `get_metadata`
- `hide_in_image`
- `extract_from_image`

## Sécurité

### Chiffrement
- AES-256-GCM
- PBKDF2 (10 000 itérations)
- Nonce unique par fichier
- Authentification des données

### Validation
- Vérification des magic bytes
- Validation des tailles
- Vérification d'intégrité

## Performance

### Optimisations
- Traitement par blocs
- Parallélisation
- Mémoire mappée
- Buffers optimisés

### Benchmarks
- Compression : ~100MB/s
- Décompression : ~200MB/s
- Stéganographie : ~50MB/s

## Développement

### Prérequis
- Rust 1.70+
- Node.js 18+
- Dépendances système

### Build
```bash
# Core
cargo build --release

# GUI
npm run tauri build
```

### Tests
```bash
# Tests unitaires
cargo test

# Tests d'intégration
cargo test --test '*'
```

## Contribution

Voir [CONTRIBUTING.md](../../CONTRIBUTING.md) pour les directives de contribution. 