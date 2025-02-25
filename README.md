NTK Ultra-Compression
---------------------

NTK Ultra-Compression est un logiciel de compression de données visant à atteindre des taux de compression inégalés en s'affranchissant des contraintes temporelles habituelles.

<img src="https://raw.githubusercontent.com/thisiskeanyvy/ntk-ultra-compression/refs/heads/main/demo-1.png" alt="demo-1"/>

Caractéristiques principales
----------------------------

*   Compression maximale sans limite de temps
*   Nouveau format de fichier .ntk optimisé
*   Développé en Rust pour la performance et la sécurité
*   Idéal pour les fichiers volumineux (logiciels, jeux vidéo, bases de données)
*   Compression sans perte préservant 100% de l'intégrité des données
*   Options avancées de chiffrement et stéganographie

Utilisation
-----------

`````bash
$ ntk compress <input_file> <output_file.ntk>
`````

``````bash
$ ntk extract <input_file.ntk> <output_file>`
``````

Algorithme
----------

NTK Ultra-Compression utilise une approche de compression adaptative multi-niveaux, combinant :

*   Codage arithmétique contextuel
*   Transformation de Burrows-Wheeler
*   Factorisation de matrices creuses

L'algorithme analyse en profondeur la structure des fichiers pour appliquer dynamiquement les techniques de compression les plus appropriées à chaque segment de données.

Format .ntk
-----------

Structure de métadonnées hiérarchique et compressée, optimisant le stockage des données ultra-compressées. Utilise des techniques comme le hachage perceptuel et la déduplication au niveau des blocs.

Performances
------------

Les taux de compression varient selon les types de fichiers, mais peuvent atteindre jusqu'à 90% de réduction pour certains fichiers volumineux. Les temps de compression/décompression peuvent être longs en raison de l'optimisation poussée.

Compilation
-----------

Requiert Rust 1.55+ et cargo pour lancer NTK-Manager

``````bash
$ cargo run
``````

<p float="left">
  <img src="https://raw.githubusercontent.com/thisiskeanyvy/ntk-ultra-compression/refs/heads/main/demo/install-1.png" alt="install-1" width="48%" />
  <img src="https://raw.githubusercontent.com/thisiskeanyvy/ntk-ultra-compression/refs/heads/main/demo/install-3.png" alt="install-3" width="48%" />
</p>

Mises à jour
-------

Pour mettre à jour le logiciel vers la dernière version vous pouvez utiliser NTK-Manager ou le faire via l'outil git.

``````bash
$ git pull
``````

<img src="https://raw.githubusercontent.com/thisiskeanyvy/ntk-ultra-compression/refs/heads/main/demo/install-2.png" alt="install-2" style="zoom:80%;" />

Licence
-------

Ce projet est sous licence BSD 3-Clause. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

Auteurs
-------

*   Nathan Pelletti
*   Thomas Demesse
*   Keany Vy Khun
*   Litissia Ben Mohand

Développé dans le cadre d'un projet à l'École pour l'informatique et les techniques avancées.
