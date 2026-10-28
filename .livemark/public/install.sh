#!/bin/sh

# Installs Fairspec Terminal or Fairspec Desktop from the GitHub releases
#
#   curl -fsSL https://fairspec.org/install.sh | sh
#   curl -fsSL https://fairspec.org/install.sh | sh -s -- terminal
#   curl -fsSL https://fairspec.org/install.sh | sh -s -- desktop
#
# FAIRSPEC_PLATFORM overrides the detected platform (linux-x64, linux-arm64, macos-x64,
# macos-arm64, windows-x64); FAIRSPEC_INSTALL_DIR overrides where the binary goes.

set -eu

REPO="datisthq/fairspec"
PLATFORMS="linux-x64 linux-arm64 macos-x64 macos-arm64 windows-x64"

main() {
  product=${1:-}
  if [ -z "$product" ]; then
    product=$(ask_product)
  fi
  case "$product" in
    terminal) prefix="rust-v" ;;
    desktop) prefix="desktop-v" ;;
    *) fail "Unknown product \"$product\": expected terminal or desktop" ;;
  esac

  platform=${FAIRSPEC_PLATFORM:-$(detect_platform)}
  case " $PLATFORMS " in
    *" $platform "*) ;;
    *) fail "Unsupported platform \"$platform\": set FAIRSPEC_PLATFORM to one of: $PLATFORMS" ;;
  esac
  os=${platform%-*}
  arch=${platform#*-}

  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' EXIT
  curl -fsSL "https://api.github.com/repos/$REPO/releases?per_page=100" -o "$tmp/releases.json"
  version=$(find_version "$tmp/releases.json" "$prefix")
  [ -n "$version" ] || fail "No Fairspec $product release yet: see https://github.com/$REPO/releases"
  asset=$(name_asset "$product" "$os" "$arch" "$version")

  log "Installing: Fairspec $product $version for $platform"
  download_asset "$prefix$version" "$asset" "$tmp"

  case "$product" in
    terminal) install_terminal "$os" "$tmp/$asset" "$tmp" ;;
    desktop) install_desktop "$os" "$tmp/$asset" "$tmp" ;;
  esac
}

ask_product() {
  if ! (: < /dev/tty) 2> /dev/null; then
    fail "No terminal to ask on: pass the product, as in \"sh -s -- terminal\" or \"sh -s -- desktop\""
  fi
  {
    echo "What do you want to install?"
    echo "  1) Fairspec Terminal  the fairspec command, with its MCP server"
    echo "  2) Fairspec Desktop   the desktop application"
  } >&2
  while :; do
    printf "Choose [1-2]: " >&2
    read -r answer < /dev/tty || fail "No answer"
    case "$answer" in
      1 | terminal) echo terminal && return ;;
      2 | desktop) echo desktop && return ;;
    esac
  done
}

detect_platform() {
  case "$(uname -m)" in
    arm64* | aarch64*) arch="arm64" ;;
    *64) arch="x64" ;;
    *) arch="$(uname -m)" ;;
  esac
  case "$(uname -s | tr '[:upper:]' '[:lower:]')" in
    linux) echo "linux-$arch" ;;
    darwin) echo "macos-$arch" ;;
    # Windows on Arm runs the x64 build under emulation
    msys* | mingw* | cygwin* | *_nt*) echo "windows-x64" ;;
    *) echo "unknown-$arch" ;;
  esac
}

# Every section of the monorepo releases on its own tag, so releases/latest may be another one
find_version() {
  sed -n "s/.*\"tag_name\": *\"$2\([^\"]*\)\".*/\1/p" "$1" | head -n 1
}

# GitHub records each asset's sha256 as its "digest", a few lines below its "name"
find_digest() {
  awk -v name="\"name\": \"$2\"" '
    index($0, name) { found = 1 }
    found && /"digest":/ { sub(/.*"sha256:/, ""); sub(/".*/, ""); print; exit }
  ' "$1"
}

name_asset() {
  case "$1-$2" in
    terminal-windows) echo "fairspec-terminal-$2-$3-$4.zip" ;;
    terminal-*) echo "fairspec-terminal-$2-$3-$4.tar.gz" ;;
    desktop-linux) echo "fairspec-desktop-linux-$(name_appimage_arch "$3")-$4.AppImage" ;;
    desktop-macos) echo "fairspec-desktop-macos-$3-$4.dmg" ;;
    desktop-windows) echo "fairspec-desktop-windows-$3-$4.exe" ;;
  esac
}

name_appimage_arch() {
  case "$1" in
    x64) echo "x86_64" ;;
    *) echo "$1" ;;
  esac
}

download_asset() {
  url="https://github.com/$REPO/releases/download/$1/$2"
  log "Downloading: $url"
  curl -fL --progress-bar "$url" -o "$3/$2"
  expected=$(find_digest "$3/releases.json" "$2")
  [ -n "$expected" ] || fail "No checksum published for $2"
  actual=$(hash_file "$3/$2")
  [ "$expected" = "$actual" ] || fail "Checksum mismatch for $2"
  log "Verified: sha256 $actual"
}

hash_file() {
  if command -v sha256sum > /dev/null 2>&1; then
    sha256sum "$1" | cut -d ' ' -f 1
  else
    shasum -a 256 "$1" | cut -d ' ' -f 1
  fi
}

install_terminal() {
  dir=${FAIRSPEC_INSTALL_DIR:-$HOME/.local/bin}
  case "$1" in
    windows)
      binary="fairspec.exe"
      unzip -oq "$2" -d "$3" 2> /dev/null || tar -xf "$2" -C "$3"
      ;;
    *)
      binary="fairspec"
      tar -xzf "$2" -C "$3"
      ;;
  esac
  mkdir -p "$dir"
  mv "$3/$binary" "$dir/$binary"
  chmod +x "$dir/$binary"
  log "Installed: $dir/$binary"
  warn_path "$dir"
  log "Done: run \"fairspec --help\", or \"fairspec mcp\" for the MCP server"
}

install_desktop() {
  case "$1" in
    linux)
      dir=${FAIRSPEC_INSTALL_DIR:-$HOME/.local/bin}
      mkdir -p "$dir"
      mv "$2" "$dir/fairspec-desktop"
      chmod +x "$dir/fairspec-desktop"
      log "Installed: $dir/fairspec-desktop"
      warn_path "$dir"
      log "Done: run \"fairspec-desktop\""
      ;;
    macos)
      dir=${FAIRSPEC_INSTALL_DIR:-/Applications}
      [ -w "$dir" ] || dir="$HOME/Applications"
      mkdir -p "$dir" "$3/mount"
      hdiutil attach -nobrowse -readonly -quiet -mountpoint "$3/mount" "$2"
      app=$(find "$3/mount" -maxdepth 1 -name "*.app" | head -n 1)
      rm -rf "$dir/$(basename "$app")"
      cp -R "$app" "$dir/"
      hdiutil detach -quiet "$3/mount"
      log "Installed: $dir/$(basename "$app")"
      log "Done: open it from $dir"
      ;;
    windows)
      log "Running: the installer"
      "$2"
      log "Done: open Fairspec from the Start menu"
      ;;
  esac
}

warn_path() {
  case ":$PATH:" in
    *":$1:"*) ;;
    *) log "Note: $1 is not on your PATH; add it, as in: export PATH=\"$1:\$PATH\"" ;;
  esac
}

log() {
  echo "$@" >&2
}

fail() {
  echo "Error: $*" >&2
  exit 1
}

main "$@"
