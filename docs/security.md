# Sécurité de NTK Ultra-Compression

## Vue d'ensemble

NTK Ultra-Compression implémente plusieurs mécanismes de sécurité pour protéger vos données :
- Chiffrement AES-256-GCM
- Dérivation de clé sécurisée (PBKDF2)
- Stéganographie LSB
- Vérification d'intégrité

## Chiffrement

### AES-256-GCM

Nous utilisons AES-256 en mode GCM (Galois/Counter Mode) qui fournit :
- Chiffrement authentifié
- Protection contre les modifications
- Confidentialité des données

Caractéristiques :
- Clé de 256 bits
- Nonce de 12 octets
- Tag d'authentification de 16 octets

### Dérivation de clé

La clé de chiffrement est dérivée du mot de passe utilisateur via PBKDF2 :
```rust
pbkdf2::pbkdf2_hmac::<sha2::Sha256>(
    password.as_bytes(),
    salt,
    10_000,  // Nombre d'itérations
    &mut key,
);
```

Paramètres :
- Fonction de hachage : SHA-256
- Sel aléatoire : 16 octets
- Itérations : 10 000
- Longueur de clé : 32 octets

## Stéganographie

### Méthode LSB (Least Significant Bit)

La stéganographie utilise la méthode LSB qui :
- Modifie le bit le moins significatif des pixels
- Préserve l'apparence visuelle
- Offre une capacité de ~3 bits par pixel

### Format des données cachées

```
[32 bits] Taille totale des données
[32 bits] Taille du bloc actuel
[N bits]  Données du bloc
```

### Sécurité

- Distribution uniforme des modifications
- Pas de motifs détectables
- Préservation des statistiques de l'image

## Validation des données

### En-tête de fichier

Vérification systématique :
- Magic bytes "NTK1"
- Version du format
- Tailles cohérentes
- Métadonnées valides

### Intégrité des blocs

Pour chaque bloc :
- Vérification de la taille
- Validation du tag GCM
- Contrôle des données décompressées

## Bonnes pratiques

### Mots de passe

Recommandations :
- Minimum 12 caractères
- Mélange de caractères
- Unique pour chaque archive
- Stockage sécurisé

### Stéganographie

Pour une utilisation sûre :
- Images PNG non compressées
- Taille suffisante
- Images uniques
- Éviter les motifs répétitifs

### Stockage

Protection des fichiers :
- Permissions restrictives
- Sauvegarde sécurisée
- Effacement sécurisé
- Isolation des données sensibles

## Limites connues

### Chiffrement

- Pas de perfect forward secrecy
- Vulnérable aux attaques par force brute sur mots de passe faibles
- Pas de protection contre les attaques par canal auxiliaire

### Stéganographie

- Détectable par analyse statistique avancée
- Capacité limitée par la taille de l'image
- Sensible aux modifications de l'image

## Recommandations

### Sécurité maximale

1. Utilisez des mots de passe forts
2. Changez régulièrement les mots de passe
3. Utilisez des images différentes
4. Effacez les fichiers temporaires

### En cas de compromission

1. Changez immédiatement les mots de passe
2. Recréez les archives compromises
3. Utilisez de nouvelles images
4. Vérifiez les logs système

## Audit et tests

### Tests automatisés

```bash
# Tests de sécurité
cargo test --test security
cargo test --test crypto
cargo test --test stego
```

### Vérification manuelle

1. Test de robustesse des mots de passe
2. Validation des données chiffrées
3. Analyse des images stéganographiées
4. Vérification des permissions

## Support

Pour signaler un problème de sécurité :
1. NE PAS créer d'issue publique
2. Contacter security@ntk-compression.com
3. Utiliser la clé PGP fournie
4. Attendre la confirmation 