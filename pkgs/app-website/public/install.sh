#!/bin/sh
# Genotype installer
# curl -fsSL https://genotype-lang.org/install.sh | sh

set -eu

TAG_VERSION="${VERSION:-}"
GH_REPO_NAME="kossnocorp/genotype"
GH_API_HEADER_ACCEPT="Accept: application/vnd.github+json"
GH_API_HEADER_VERSION="X-GitHub-Api-Version: 2026-03-10"

main() {
	case "$(uname -s)" in
	Darwin) os="apple-darwin" ;;
	Linux) os="unknown-linux-gnu" ;;
	*)
		echo "Unsupported OS: $(uname -s)" >&2
		exit 1
		;;
	esac

	case "$(uname -m)" in
	x86_64 | amd64) arch="x86_64" ;;
	arm64 | aarch64) arch="aarch64" ;;
	*)
		echo "Unsupported architecture: $(uname -m)" >&2
		exit 1
		;;
	esac

	# Prefer the native ARM64 binary when running under Rosetta 2.
	if [ "$os" = "apple-darwin" ] && [ "$arch" = "x86_64" ]; then
		if [ "$(sysctl -n sysctl.proc_translated 2>/dev/null)" = "1" ]; then
			arch="aarch64"
		fi
	fi

	if command -v curl >/dev/null 2>&1; then
		download() {
			curl -fL --progress-bar -o "$1" "$2"
		}

		gh_get() {
			curl -fsSL \
				-H "$GH_API_HEADER_ACCEPT" \
				-H "$GH_API_HEADER_VERSION" \
				"$1"
		}
	elif command -v wget >/dev/null 2>&1; then
		download() {
			wget --show-progress -qO "$1" "$2"
		}

		gh_get() {
			wget -qO- \
				--header="$GH_API_HEADER_ACCEPT" \
				--header="$GH_API_HEADER_VERSION" \
				"$1"
		}
	else
		echo "curl or wget is required" >&2
		exit 1
	fi

	if ! command -v sed >/dev/null 2>&1; then
		echo "sed is required" >&2
		exit 1
	fi

	if [ -n "$TAG_VERSION" ]; then
		version_json="$(
			gh_get "https://api.github.com/repos/${GH_REPO_NAME}/releases/tags/${TAG_VERSION}"
		)"
	else
		version_json="$(
			gh_get "https://api.github.com/repos/${GH_REPO_NAME}/releases/latest"
		)"
	fi

	version="$(
		printf '%s\n' "$version_json" |
			sed -n 's/^[[:space:]]*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p'
	)"

	if [ -z "$version" ]; then
		echo "Failed to determine release version from GitHub response." >&2
		exit 1
	fi

	binary="gt-${version}-${arch}-${os}"
	binary_url="https://github.com/${GH_REPO_NAME}/releases/download/${version}/${binary}"
	binary_checksum="$(
		printf '%s\n' "$version_json" |
			sed -n '/"name": "'"$binary"'"/,/"digest":/{
			/"digest":/{
				s/.*"digest":[[:space:]]*"sha256:\([^"]*\)".*/\1/p
				q
			}
		}'
	)"

	if [ -z "$binary_checksum" ]; then
		echo "No release asset or SHA256 digest found for ${binary}" >&2
		exit 1
	fi

	tmp_binary="$(mktemp)"
	trap 'rm -f "$tmp_binary"' EXIT

	echo "Downloading Genotype ${version} (${arch}-${os})..."
	download "$tmp_binary" "$binary_url"

	printf "Verifying checksum... "
	if command -v sha256sum >/dev/null 2>&1; then
		actual_checksum="$(sha256sum "$tmp_binary" | cut -d' ' -f1)"
	elif command -v shasum >/dev/null 2>&1; then
		actual_checksum="$(shasum -a 256 "$tmp_binary" | cut -d' ' -f1)"
	else
		echo "shasum or sha256sum is required" >&2
		exit 1
	fi

	if [ "$actual_checksum" != "$binary_checksum" ]; then
		echo "Checksum mismatch!" >&2
		echo "  expected: $binary_checksum" >&2
		echo "  actual:   $actual_checksum" >&2
		exit 1
	fi
	echo "ok"

	chmod +x "$tmp_binary"

	if [ -w /usr/local/bin ]; then
		install_dir="/usr/local/bin"
	else
		install_dir="$HOME/.local/bin"
		mkdir -p "$install_dir"
	fi

	mv "$tmp_binary" "${install_dir}/gt"

	case ":${PATH}:" in
	*":${install_dir}:"*) ;;
	*)
		echo ""
		echo "Note: ${install_dir} is not on your PATH."
		echo "Add the following to your shell profile:"
		echo "  export PATH=\"${install_dir}:\$PATH\""
		;;
	esac

	echo ""
	echo "Genotype ${version} installed to ${install_dir}/gt"
}

main
