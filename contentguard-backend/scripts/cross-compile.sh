#!/bin/bash
# Cross-compilation script for ContentGuard AI
# Builds binaries for multiple architectures

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}ContentGuard AI - Cross Compilation${NC}"
echo ""

# Targets to build
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-gnu"
    "aarch64-unknown-linux-musl"
    "x86_64-pc-windows-gnu"
)

# Create output directory
mkdir -p dist

# Function to build for a target
build_for_target() {
    local target=$1
    echo -e "${GREEN}Building for $target...${NC}"
    
    # Check if target is installed
    if ! rustup target list --installed | grep -q "$target"; then
        echo -e "${YELLOW}Installing target: $target${NC}"
        rustup target add "$target"
    fi
    
    # Set linker for musl targets
    if [[ $target == *"musl"* ]]; then
        export RUSTFLAGS="-C target-feature=+crt-static"
    else
        export RUSTFLAGS=""
    fi
    
    # Build
    if cargo build --release --target "$target" --features "api,cli" 2>&1; then
        # Copy binary to dist
        local binary_name="contentguard"
        if [[ $target == *"windows"* ]]; then
            binary_name="contentguard.exe"
        fi
        
        local source="target/$target/release/$binary_name"
        local dest="dist/contentguard-${target}"
        
        if [ -f "$source" ]; then
            cp "$source" "$dest"
            # Compress
            if [[ $target != *"windows"* ]]; then
                gzip -f "$dest"
                echo -e "${GREEN}✓ Built: $dest.gz${NC}"
            else
                zip -j "$dest.zip" "$dest"
                echo -e "${GREEN}✓ Built: $dest.zip${NC}"
            fi
        fi
    else
        echo -e "${RED}✗ Failed to build for $target${NC}"
    fi
}

# Build for all targets
for target in "${TARGETS[@]}"; do
    build_for_target "$target"
done

echo ""
echo -e "${GREEN}Cross-compilation complete!${NC}"
echo "Binaries available in ./dist/"
echo ""
ls -lh dist/
