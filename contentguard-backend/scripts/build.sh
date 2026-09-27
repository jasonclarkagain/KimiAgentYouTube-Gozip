#!/bin/bash
# Build script for ContentGuard AI
# Supports cross-compilation for multiple architectures

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
TARGET="native"
RELEASE=true
FEATURES="api,cli"
GPU=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --target)
            TARGET="$2"
            shift 2
            ;;
        --debug)
            RELEASE=false
            shift
            ;;
        --features)
            FEATURES="$2"
            shift 2
            ;;
        --gpu)
            GPU=true
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --target TARGET    Target architecture (native, x86_64, aarch64)"
            echo "  --debug            Build in debug mode"
            echo "  --features FEATURES Comma-separated list of features"
            echo "  --gpu              Enable GPU acceleration"
            echo "  --help             Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Add GPU feature if requested
if [ "$GPU" = true ]; then
    FEATURES="${FEATURES},vulkan"
fi

echo -e "${GREEN}Building ContentGuard AI...${NC}"
echo "Target: $TARGET"
echo "Release: $RELEASE"
echo "Features: $FEATURES"

# Check for required tools
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo not found. Please install Rust.${NC}"
    exit 1
fi

if ! command -v ffmpeg &> /dev/null; then
    echo -e "${YELLOW}Warning: FFmpeg not found. Runtime will require FFmpeg.${NC}"
fi

# Set build flags
if [ "$RELEASE" = true ]; then
    BUILD_FLAGS="--release"
    BUILD_DIR="target/release"
else
    BUILD_FLAGS=""
    BUILD_DIR="target/debug"
fi

# Set target flags
if [ "$TARGET" != "native" ]; then
    # Check if target is installed
    if ! rustup target list --installed | grep -q "$TARGET"; then
        echo -e "${YELLOW}Installing target: $TARGET${NC}"
        rustup target add "$TARGET"
    fi
    TARGET_FLAGS="--target $TARGET-unknown-linux-gnu"
else
    TARGET_FLAGS=""
fi

# Set RUSTFLAGS for optimization
export RUSTFLAGS="-C target-cpu=sandybridge -C opt-level=3"

# Build
echo -e "${GREEN}Running cargo build...${NC}"
cargo build $BUILD_FLAGS $TARGET_FLAGS --features "$FEATURES"

# Strip binary if release build
if [ "$RELEASE" = true ]; then
    echo -e "${GREEN}Stripping binary...${NC}"
    strip "$BUILD_DIR/contentguard" || true
fi

# Show binary info
echo -e "${GREEN}Build complete!${NC}"
echo "Binary location: $BUILD_DIR/contentguard"
echo "Binary size: $(du -h "$BUILD_DIR/contentguard" | cut -f1)"
echo ""
echo "To run: $BUILD_DIR/contentguard --help"
