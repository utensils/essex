#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Export for testing
export INSTALL_DIR="${HOME}/.local/bin"
export ESSEX_REPO="utensils/essex"

# Print step information
info() {
    echo -e "${BLUE}==>${NC} $1" >&2
}

# Print success messages
success() {
    echo -e "${GREEN}==>${NC} $1" >&2
}

# Print error messages
error() {
    echo -e "${RED}==>${NC} $1" >&2
    return 1
}

# Detect the operating system and architecture
detect_platform() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"
    
    case "$os" in
        "Darwin")
            case "$arch" in
                "arm64")
                    echo "aarch64-apple-darwin"
                    return 0
                    ;;
                "x86_64")
                    echo "x86_64-apple-darwin"
                    return 0
                    ;;
            esac
            ;;
        "Linux")
            case "$arch" in
                "aarch64"|"arm64")
                    echo "aarch64-unknown-linux-gnu"
                    return 0
                    ;;
                "x86_64")
                    echo "x86_64-unknown-linux-gnu"
                    return 0
                    ;;
            esac
            ;;
    esac
    
    echo "Unsupported platform: $os/$arch" >&2
    return 1
}

# Get the latest version from GitHub API
get_latest_version() {
    local version
    version=$(curl -s "https://api.github.com/repos/${ESSEX_REPO}/releases/latest" | grep '"tag_name":' | cut -d '"' -f 4)
    if [ -z "$version" ]; then
        error "Failed to fetch latest version"
        return 1
    fi
    echo "$version"
}

# Verify SHA256 checksum
verify_checksum() {
    local file="$1"
    local expected_sha="$2"
    local computed_sha
    
    if command -v sha256sum >/dev/null 2>&1; then
        computed_sha=$(sha256sum "$file" | cut -d ' ' -f 1)
    elif command -v shasum >/dev/null 2>&1; then
        computed_sha=$(shasum -a 256 "$file" | cut -d ' ' -f 1)
    else
        error "No SHA256 utility found"
        return 1
    fi
    
    if [ "$computed_sha" != "$expected_sha" ]; then
        error "Checksum verification failed"
        return 1
    fi
    return 0
}

# Download and install Essex
install_essex() {
    local version="$1"
    local platform="$2"
    local temp_dir
    temp_dir=$(mktemp -d)
    local download_url="https://github.com/${ESSEX_REPO}/releases/download/${version}/essex-${platform}.tar.gz"
    
    info "Downloading Essex ${version} for ${platform}..."
    if ! curl -L --fail "$download_url" -o "${temp_dir}/essex.tar.gz"; then
        error "Download failed"
        rm -rf "$temp_dir"
        return 1
    fi
    
    info "Extracting..."
    if ! tar xzf "${temp_dir}/essex.tar.gz" -C "$temp_dir"; then
        error "Extraction failed"
        rm -rf "$temp_dir"
        return 1
    fi
    
    info "Installing to $INSTALL_DIR..."
    mkdir -p "$INSTALL_DIR"
    if ! mv "${temp_dir}/essex" "${INSTALL_DIR}/"; then
        error "Installation failed"
        rm -rf "$temp_dir"
        return 1
    fi
    chmod +x "${INSTALL_DIR}/essex"
    
    # Clean up
    rm -rf "$temp_dir"
    return 0
}

# Print post-installation instructions
print_instructions() {
    echo
    success "Essex has been installed to ${INSTALL_DIR}/essex"
    echo
    echo "To complete the installation, add the following to your shell configuration file"
    echo "(.bashrc, .zshrc, or equivalent):"
    echo
    echo '  export PATH="$HOME/.local/bin:$PATH"'
    echo
    echo "You can do this by running:"
    echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
    echo
    echo "Then reload your shell configuration:"
    echo "  source ~/.bashrc"
    echo
    echo "To verify the installation, run:"
    echo "  essex --version"
    echo
    echo "To get started, run:"
    echo "  essex help"
}

# Main installation function
main() {
    local platform version
    
    platform=$(detect_platform) || exit 1
    version=$(get_latest_version) || exit 1
    
    info "Installing Essex..."
    info "Platform: $platform"
    info "Version: $version"
    
    if ! install_essex "$version" "$platform"; then
        error "Installation failed"
        exit 1
    fi
    
    print_instructions
}

# Run main function only if script is being executed directly
if [ "${BASH_SOURCE[0]}" -ef "$0" ]; then
    main "$@"
fi