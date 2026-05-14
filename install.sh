#!/bin/bash
set -euo pipefail

REPO="EzyGang/kaneo-cli"
BIN_NAME="kaneo-cli"

detect_platform() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)  os="linux" ;;
        Darwin) os="darwin" ;;
        *)      echo "Unsupported OS: $os" >&2; exit 1 ;;
    esac

    case "$arch" in
        x86_64|amd64)   arch="x64" ;;
        aarch64|arm64)  arch="arm64" ;;
        *)              echo "Unsupported architecture: $arch" >&2; exit 1 ;;
    esac

    if [ "$os" = "linux" ] && [ "$arch" = "arm64" ]; then
        echo "Linux arm64 is not yet supported" >&2
        exit 1
    fi

    echo "${os}-${arch}"
}

get_latest_version() {
    curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' \
        | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/'
}

main() {
    local platform version url tmpdir install_dir

    platform="$(detect_platform)"
    echo "Detected platform: ${platform}"

    echo "Fetching latest version..."
    version="$(get_latest_version)"
    if [ -z "$version" ]; then
        echo "Error: could not determine latest version" >&2
        exit 1
    fi
    echo "Latest version: ${version}"

    url="https://github.com/${REPO}/releases/download/${version}/${BIN_NAME}-${platform}.tar.gz"
    echo "Downloading ${url}..."

    tmpdir="$(mktemp -d)"
    trap 'rm -rf "$tmpdir"' EXIT

    curl -fsSL "$url" -o "${tmpdir}/${BIN_NAME}.tar.gz"
    tar -xzf "${tmpdir}/${BIN_NAME}.tar.gz" -C "$tmpdir"

    if [ ! -f "${tmpdir}/${BIN_NAME}" ]; then
        echo "Error: binary not found in archive" >&2
        exit 1
    fi

    chmod +x "${tmpdir}/${BIN_NAME}"

    if [ -w "/usr/local/bin" ]; then
        install_dir="/usr/local/bin"
    else
        install_dir="${HOME}/.local/bin"
        mkdir -p "$install_dir"
    fi

    mv "${tmpdir}/${BIN_NAME}" "${install_dir}/${BIN_NAME}"
    echo "Installed ${BIN_NAME} to ${install_dir}/${BIN_NAME}"

    if ! echo "$PATH" | tr ':' '\n' | grep -qx "$install_dir"; then
        echo ""
        echo "NOTE: ${install_dir} is not in your PATH."
        echo "Add it to your shell profile:"
        echo ""
        echo "  export PATH=\"${install_dir}:\$PATH\""
        echo ""
    fi

    echo "Run '${BIN_NAME} --help' to get started."
}

main
