# FAQ NTK Ultra-Compression

## Questions générales

### Q: Qu'est-ce que NTK Ultra-Compression ?
R: C'est un logiciel de compression avancé qui permet de :
- Compresser des fichiers avec un excellent taux de compression
- Chiffrer les données de manière sécurisée
- Cacher des archives dans des images (stéganographie)

### Q: Quels systèmes d'exploitation sont supportés ?
R: NTK Ultra-Compression fonctionne sur :
- Windows 10 et plus récent
- macOS 10.15 et plus récent
- Linux (distributions récentes)

### Q: Quelle est la taille maximale des fichiers ?
R: La limite dépend de :
- La mémoire disponible
- L'espace disque
- Le système de fichiers
En pratique, vous pouvez compresser des fichiers de plusieurs Go.

## Compression

### Q: Quel niveau de compression choisir ?
R: Recommandations :
- Niveau 1-5 : Compression rapide
- Niveau 6-16 : Bon compromis
- Niveau 17-22 : Compression maximale
Le niveau 19 est recommandé pour un usage général.

### Q: Pourquoi mes fichiers déjà compressés ne se compriment pas plus ?
R: Les formats déjà compressés (ZIP, JPG, MP3, etc.) ne peuvent généralement pas être compressés davantage.

### Q: Comment obtenir la meilleure compression ?
R: Conseils :
1. Regroupez les fichiers similaires
2. Utilisez un niveau de compression élevé
3. Évitez de compresser des fichiers déjà compressés

## Chiffrement

### Q: Mon mot de passe est-il sûr ?
R: Un bon mot de passe doit :
- Avoir au moins 12 caractères
- Mélanger lettres, chiffres et symboles
- Ne pas être réutilisé
- Être mémorisable

### Q: Que faire si j'oublie mon mot de passe ?
R: Il n'y a malheureusement aucun moyen de récupérer les données sans le mot de passe. Conservez vos mots de passe en lieu sûr.

### Q: Le chiffrement ralentit-il la compression ?
R: L'impact est minime grâce à :
- L'optimisation du code
- L'utilisation d'instructions matérielles
- Le traitement par blocs

## Stéganographie

### Q: Quelle taille d'image dois-je utiliser ?
R: La capacité est d'environ 3 bits par pixel. Par exemple :
- Image 1920x1080 ≈ 750 Ko
- Image 4K ≈ 3 Mo
- Image 8K ≈ 12 Mo

### Q: Quels formats d'image sont supportés ?
R: Actuellement, seul le format PNG est supporté car il :
- Est sans perte
- Préserve les modifications LSB
- Est largement compatible

### Q: L'image sera-t-elle visiblement modifiée ?
R: Non, les modifications sont imperceptibles car :
- Seul le bit le moins significatif est modifié
- Les changements sont distribués uniformément
- L'œil humain ne peut pas détecter ces modifications

## Problèmes courants

### Q: "Invalid file format" lors de la décompression
Solutions :
1. Vérifiez que le fichier n'est pas corrompu
2. Assurez-vous d'avoir le bon mot de passe
3. Réessayez avec le fichier original

### Q: "Out of memory" pendant la compression
Solutions :
1. Fermez les applications inutiles
2. Augmentez la mémoire virtuelle
3. Compressez par parties plus petites

### Q: L'application ne démarre pas
Vérifications :
1. Prérequis système installés
2. Droits d'administration si nécessaire
3. Antivirus ne bloque pas l'application

## Performance

### Q: Comment accélérer la compression ?
Optimisations :
1. Utilisez un SSD
2. Fermez les applications gourmandes
3. Choisissez un niveau de compression plus bas

### Q: Pourquoi la stéganographie est-elle lente ?
R: Le processus est plus lent car :
- Chaque pixel doit être modifié individuellement
- Les images sont volumineuses
- La précision est primordiale

### Q: Combien de RAM est nécessaire ?
R: Recommandations :
- Minimum : 4 Go
- Recommandé : 8 Go
- Optimal : 16 Go ou plus

## Divers

### Q: Comment contribuer au projet ?
R: Vous pouvez :
1. Signaler des bugs
2. Proposer des améliorations
3. Contribuer au code
Consultez CONTRIBUTING.md pour plus d'informations.

### Q: Où trouver de l'aide supplémentaire ?
R: Ressources disponibles :
1. Documentation en ligne
2. Forum communautaire
3. Support technique
4. Issues GitHub 