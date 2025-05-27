# Guide utilisateur NTK Ultra-Compression

## Table des matières

1. [Introduction](#introduction)
2. [Interface principale](#interface-principale)
3. [Compression](#compression)
4. [Décompression](#décompression)
5. [Chiffrement](#chiffrement)
6. [Stéganographie](#stéganographie)
7. [Astuces et bonnes pratiques](#astuces-et-bonnes-pratiques)

## Introduction

NTK Ultra-Compression est un logiciel de compression avancé qui permet de :
- Compresser des fichiers avec un excellent taux de compression
- Chiffrer les données de manière sécurisée
- Dissimuler des archives dans des images (stéganographie)

## Interface principale

L'interface est divisée en deux onglets principaux :
- **Compression** : Pour compresser et décompresser des fichiers
- **Stéganographie** : Pour cacher et extraire des archives dans des images

### Barre d'outils
- Sélection de fichiers
- Options de compression
- Indicateur de progression
- Informations sur le fichier

## Compression

1. **Sélection du fichier**
   - Cliquez sur "Select Input File"
   - Choisissez le fichier à compresser
   - Sélectionnez la destination avec "Select Output Location"

2. **Options de compression**
   - Niveau de compression (1-22)
     * 1-5 : Compression rapide
     * 6-16 : Compression équilibrée
     * 17-22 : Compression maximale
   - Chiffrement (optionnel)
   - Mot de passe (si chiffrement activé)

3. **Lancement de la compression**
   - Vérifiez les paramètres
   - Cliquez sur "Compress"
   - Attendez la fin du processus

## Décompression

1. **Sélection du fichier**
   - Choisissez un fichier .ntk
   - Sélectionnez la destination

2. **Déchiffrement (si nécessaire)**
   - Entrez le mot de passe si le fichier est chiffré
   - Le logiciel détecte automatiquement si le fichier est chiffré

3. **Lancement de la décompression**
   - Cliquez sur "Decompress"
   - Attendez la fin du processus

## Chiffrement

Le chiffrement utilise AES-256-GCM :
- Sécurité de niveau militaire
- Protection contre les modifications
- Dérivation de clé sécurisée

Pour utiliser le chiffrement :
1. Activez l'option "Enable Encryption"
2. Entrez un mot de passe fort
3. Conservez le mot de passe en lieu sûr

## Stéganographie

### Cacher une archive

1. **Préparation**
   - Compressez d'abord votre fichier en .ntk
   - Préparez une image PNG de taille suffisante

2. **Dans l'onglet Stéganographie**
   - Sélectionnez l'archive .ntk
   - Choisissez l'image support
   - Sélectionnez la destination
   - Cliquez sur "Cacher l'archive"

### Extraire une archive

1. **Dans l'onglet Stéganographie**
   - Sélectionnez l'image contenant l'archive
   - Choisissez la destination pour l'archive
   - Cliquez sur "Extraire l'archive"

## Astuces et bonnes pratiques

### Compression optimale
- Utilisez le niveau 19 pour un bon compromis
- Regroupez les fichiers similaires
- Évitez de compresser des fichiers déjà compressés

### Sécurité
- Utilisez des mots de passe forts (12+ caractères)
- Ne réutilisez pas les mots de passe
- Sauvegardez vos mots de passe de manière sécurisée

### Stéganographie
- Utilisez des images PNG de bonne qualité
- Vérifiez que l'image est assez grande
- Évitez de réutiliser la même image

### Performance
- Fermez les applications gourmandes en ressources
- Libérez de l'espace disque
- Attendez la fin des opérations en cours

## Dépannage

### Erreurs courantes

1. "Invalid file format"
   - Vérifiez que le fichier n'est pas corrompu
   - Assurez-vous d'avoir le bon mot de passe

2. "Image too small"
   - Utilisez une image plus grande
   - Réduisez la taille de l'archive

3. "Out of memory"
   - Libérez de la mémoire
   - Redémarrez l'application

### Support

Si vous rencontrez des problèmes :
1. Consultez la [FAQ](FAQ.md)
2. Vérifiez les logs
3. Contactez le support 