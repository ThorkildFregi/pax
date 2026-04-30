#!/bin/bash

# Definition des couleurs
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Pax Language Installer${NC}"
echo "---------------------------"

OS_TYPE=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH_TYPE=$(uname -m)

if [ "$OS_TYPE" = "linux" ]; then
    ASSET_NAME="pax-linux-x64"
elif [ "$OS_TYPE" = "darwin" ]; then
    ASSET_NAME="pax-macos-x64"
else
    echo -e "${RED}Error: Unsupported OS ($OS_TYPE)${NC}"
    exit 1
fi

REPO="ThorkildFregi/pax"
LATEST_RELEASE_URL=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "browser_download_url" | grep "$ASSET_NAME" | cut -d '"' -f 4 | head -n 1)

if [ -z "$LATEST_RELEASE_URL" ]; then
    echo -e "${RED}Error: Could not find binary for $ASSET_NAME${NC}"
    exit 1
fi

echo -e "Downloading latest version..."
curl -L -o pax_bin "$LATEST_RELEASE_URL"

echo -e "Installing to /usr/local/bin..."
chmod +x pax_bin
sudo mv pax_bin /usr/local/bin/pax

if [ $? -eq 0 ]; then
    echo -e "---------------------------"
    echo -e "${GREEN}Pax has been installed successfully!${NC}"
    echo -e "Type '${BLUE}pax --version${NC}' to test"
else
    echo -e "${RED}Installation failed. Verify sudo permissions.${NC}"
fi