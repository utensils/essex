#!/bin/bash

# shellcheck source=./download_cli.sh
source "$(dirname "$0")/download_cli.sh"

# Colors for test output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

total_tests=0
failed_tests=0

# Test helper functions
assert_equals() {
    local expected="$1"
    local actual="$2"
    local message="$3"
    ((total_tests++))
    
    if [ "$expected" = "$actual" ]; then
        echo -e "${GREEN}✓${NC} $message"
        return 0
    else
        echo -e "${RED}✗${NC} $message"
        echo "  Expected: $expected"
        echo "  Got:      $actual"
        ((failed_tests++))
        return 1
    fi
}

assert_success() {
    local message="$1"
    shift
    ((total_tests++))
    
    if "$@"; then
        echo -e "${GREEN}✓${NC} $message"
        return 0
    else
        echo -e "${RED}✗${NC} $message"
        ((failed_tests++))
        return 1
    fi
}

assert_failure() {
    local message="$1"
    shift
    ((total_tests++))
    
    if ! "$@"; then
        echo -e "${GREEN}✓${NC} $message"
        return 0
    else
        echo -e "${RED}✗${NC} $message"
        ((failed_tests++))
        return 1
    fi
}

# Unit Tests
echo -e "\n${YELLOW}Running unit tests...${NC}"

# Test platform detection
test_platform_detection() {
    echo "Testing platform detection..."
    local platform os arch expected_platform
    os="$(uname -s)"
    arch="$(uname -m)"
    echo "Debug - OS: $os, Arch: $arch"
    
    case "$os" in
        "Darwin")
            case "$arch" in
                "arm64")
                    expected_platform="aarch64-apple-darwin"
                    ;;
                "x86_64")
                    expected_platform="x86_64-apple-darwin"
                    ;;
            esac
            ;;
        "Linux")
            case "$arch" in
                "aarch64"|"arm64")
                    expected_platform="aarch64-unknown-linux-gnu"
                    ;;
                "x86_64")
                    expected_platform="x86_64-unknown-linux-gnu"
                    ;;
            esac
            ;;
    esac
    
    platform=$(detect_platform)
    echo "Debug - Detected platform: $platform"
    echo "Debug - Expected platform: $expected_platform"
    
    assert_equals "$expected_platform" "$platform" "Should detect correct platform"
}

# Test version fetching
test_version_fetching() {
    echo "Testing version fetching..."
    local version
    version=$(get_latest_version)
    assert_success "Version should be retrieved successfully" test -n "$version"
    assert_success "Version should start with 'v'" echo "$version" | grep -q "^v"
}

# Test checksum verification
test_checksum_verification() {
    echo "Testing checksum verification..."
    local temp_dir
    temp_dir=$(mktemp -d)
    cd "$temp_dir" || exit 1
    
    # Create a test file with known content
    echo "test" > test.txt
    
    # Create checksum file (f2ca1bb6c7e907d06dafe4687e579fce76b37e4e93b7605022da52e6ccc26fd2 is SHA256 of "test\n")
    echo "f2ca1bb6c7e907d06dafe4687e579fce76b37e4e93b7605022da52e6ccc26fd2  test.txt" > test.txt.sha256
    
    # Test verification
    assert_success "Checksum verification should pass for valid file" verify_checksum test.txt
    
    # Test with invalid checksum
    echo "invalid_sha  test.txt" > test.txt.sha256
    assert_failure "Checksum verification should fail for invalid SHA" verify_checksum test.txt
    
    # Clean up
    cd - > /dev/null || exit 1
    rm -rf "$temp_dir"
}

# Integration Tests
test_full_installation() {
    echo -e "\n${YELLOW}Running integration tests...${NC}"
    echo "Testing full installation process..."
    local test_dir
    test_dir=$(mktemp -d)
    export HOME="$test_dir"
    export INSTALL_DIR="${test_dir}/.local/bin"
    
    # Run installation
    assert_success "Installation should complete successfully" main
    
    # Verify binary exists and is executable
    assert_success "Binary should exist" test -f "${INSTALL_DIR}/essex"
    assert_success "Binary should be executable" test -x "${INSTALL_DIR}/essex"
    
    # Clean up
    rm -rf "$test_dir"
}

# Run all tests
run_tests() {
    test_platform_detection
    test_version_fetching
    test_checksum_verification
    test_full_installation
    
    echo
    if [ $failed_tests -eq 0 ]; then
        echo -e "${GREEN}All tests passed! ($total_tests tests)${NC}"
        return 0
    else
        echo -e "${RED}$failed_tests test(s) failed out of $total_tests total tests${NC}"
        return 1
    fi
}

# Run tests
run_tests