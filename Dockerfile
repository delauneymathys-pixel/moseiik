# On utilise l'image officielle de Rust
FROM rust:latest

# On crée notre dossier de travail dans le conteneur
WORKDIR /usr/src/moseiik

# On copie tout notre code (le .dockerignore bloquera le dossier target/)
COPY . .

# La commande qui s'exécutera automatiquement au lancement
ENTRYPOINT [ "cargo", "test", "--release", "--" ]
