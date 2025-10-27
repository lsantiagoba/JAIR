#!/bin/bash

# INSTALL_SNAP.sh - Script to install and test the JAIR snap

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}======================================${NC}"
echo -e "${BLUE}  JAIR - Snap Installer${NC}"
echo -e "${BLUE}======================================${NC}"
echo ""

# Remove old snap if installed
echo -e "${BLUE}🗑️  Removing old snap if installed...${NC}"
sudo snap remove jair 2>/dev/null || true

# Install the new snap
echo -e "${BLUE}📦 Installing new snap...${NC}"
sudo snap install --dangerous jair_1.0.0_amd64.snap

# Connect necessary interfaces
echo -e "${BLUE}🔌 Connecting interfaces...${NC}"
sudo snap connect jair:home
sudo snap connect jair:removable-media
sudo snap connect jair:mount-observe

echo ""
echo -e "${GREEN}✅ Snap installed successfully!${NC}"
echo ""
echo -e "${BLUE}To run JAIR, execute:${NC}"
echo -e "${GREEN}  snap run jair${NC}"
echo -e "${GREEN}  or just: jair${NC}"
echo ""
echo -e "${BLUE}To check the installation:${NC}"
echo -e "${GREEN}  snap info jair${NC}"
