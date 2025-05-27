#!/bin/bash

# Couleurs pour les messages
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${YELLOW}Building NTK Ultra-Compression...${NC}"

# Vérifier les prérequis
echo -e "\n${YELLOW}Checking prerequisites...${NC}"

# Vérifier Rust
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed${NC}"
    exit 1
fi

# Vérifier Node.js
if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js is not installed${NC}"
    exit 1
fi

# Vérifier npm
if ! command -v npm &> /dev/null; then
    echo -e "${RED}Error: npm is not installed${NC}"
    exit 1
fi

# Installer les dépendances du frontend
echo -e "\n${YELLOW}Installing frontend dependencies...${NC}"
cd gui
npm install
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Failed to install frontend dependencies${NC}"
    exit 1
fi

# Compiler le frontend et le backend avec Tauri
echo -e "\n${YELLOW}Building application...${NC}"
npm run tauri build
if [ $? -ne 0 ]; then
    echo -e "${RED}Error: Build failed${NC}"
    exit 1
fi

echo -e "\n${GREEN}Build completed successfully!${NC}"
echo -e "The application bundle can be found in ${YELLOW}gui/src-tauri/target/release/${NC}"

# Créer les packages de distribution si on est sur Linux
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo -e "\n${YELLOW}Creating distribution packages...${NC}"
    
    # Créer le .deb
    cargo install cargo-deb
    cd src-tauri
    cargo deb
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Created .deb package${NC}"
    else
        echo -e "${RED}Failed to create .deb package${NC}"
    fi
    
    # Créer le .rpm
    cargo install cargo-rpm
    cargo rpm build
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Created .rpm package${NC}"
    else
        echo -e "${RED}Failed to create .rpm package${NC}"
    fi
    
    cd ..
fi

echo -e "\n${GREEN}All done!${NC}" 