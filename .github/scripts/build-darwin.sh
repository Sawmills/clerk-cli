#!/usr/bin/env bash
set -euo pipefail

target=${1:?expected a Darwin target}
case "$target" in
  x86_64-apple-darwin | aarch64-apple-darwin) ;;
  *) printf 'Darwin build outcome=invalid_target\n' >&2; exit 2 ;;
esac

if [[ $(uname -s) != Linux || $(uname -m) != x86_64 ]]; then
  printf 'Darwin build outcome=unsupported_host\n' >&2
  exit 2
fi

repository_root=$(git rev-parse --show-toplevel)
workdir=$(mktemp -d "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/clerk-darwin.XXXXXXXX")
trap 'rm -rf "$workdir"' EXIT
export CLERK_DARWIN_DIRECTORY="$workdir"
export CLERK_DARWIN_TARGET="$target"

run_quiet() {
  local label=$1
  shift
  local status=0
  "$@" >"$workdir/command.log" 2>&1 || status=$?
  if ((status != 0)); then
    local digest
    digest=$(sha256sum "$workdir/command.log" | cut -d ' ' -f 1)
    printf '%s outcome=failure exit=%s log_sha256=%s\n' "$label" "$status" "$digest" >&2
    return "$status"
  fi
}

if [[ -n ${SDKROOT:-} ]]; then
  printf 'Darwin build outcome=unexpected_SDKROOT\n' >&2
  exit 1
fi
unset SDKROOT CFLAGS CXXFLAGS LDFLAGS BINDGEN_EXTRA_CLANG_ARGS CC CXX AR RANLIB
export CARGO_HOME="$workdir/cargo"
export RUSTUP_HOME="$workdir/rustup"
export CARGO_TARGET_DIR="$workdir/target"
export CARGO_ZIGBUILD_CACHE_DIR="$workdir/zig-cache"
export ZIG_GLOBAL_CACHE_DIR="$workdir/zig-global"
export ZIG_LOCAL_CACHE_DIR="$workdir/zig-local"
export CARGO_BUILD_JOBS=2
export PYTHONPATH="$workdir/zig-package"
mkdir -p "$workdir/bin"

run_quiet cargo-zigbuild-download curl --fail --silent --show-error --location \
  --retry 0 --max-time 180 \
  https://github.com/rust-cross/cargo-zigbuild/releases/download/v0.24.1/cargo-zigbuild-x86_64-unknown-linux-gnu.tar.xz \
  --output "$workdir/cargo-zigbuild.tar.xz"
printf '%s  %s\n' 32581c8e13f27a807bb244aaaac870522f5b51263c766cf77b1a99771ce9fa5c \
  "$workdir/cargo-zigbuild.tar.xz" | sha256sum --check --status
python3 - <<'PY'
import os
import pathlib
import tarfile

directory = pathlib.Path(os.environ["CLERK_DARWIN_DIRECTORY"])
with tarfile.open(directory / "cargo-zigbuild.tar.xz", "r:xz") as archive:
    members = [
        member for member in archive.getmembers()
        if member.isfile() and pathlib.PurePosixPath(member.name).name == "cargo-zigbuild"
    ]
    assert len(members) == 1 and members[0].size < 100_000_000
    binary = directory / "bin/cargo-zigbuild"
    binary.write_bytes(archive.extractfile(members[0]).read())
    binary.chmod(0o700)
PY

wheel=ziglang-0.16.0-py3-none-manylinux_2_12_x86_64.manylinux2010_x86_64.musllinux_1_1_x86_64.whl
run_quiet zig-download curl --fail --silent --show-error --location \
  --retry 0 --max-time 180 \
  "https://files.pythonhosted.org/packages/3e/ed/7b79023aa27ceb5d461ecf761181e7c33c57bbc1a6256a39535d1c7083d2/$wheel" \
  --output "$workdir/$wheel"
printf '%s  %s\n' 9fcda73f62b851dd72a54b710ad40a209896db14cfb13649e62191243556342b \
  "$workdir/$wheel" | sha256sum --check --status
run_quiet zig-install python3 -m pip install --no-deps --no-index --no-cache-dir \
  --disable-pip-version-check --no-compile --target "$workdir/zig-package" "$workdir/$wheel"
printf '#!/bin/sh\nexec python3 -m ziglang "$@"\n' >"$workdir/bin/zig"
chmod 700 "$workdir/bin/zig"
export PATH="$workdir/bin:$PATH"
[[ $(zig version) == 0.16.0 ]]
run_quiet rust-install rustup toolchain install 1.93.0 --profile minimal --no-self-update
run_quiet rust-target rustup target add --toolchain 1.93.0 "$target"

cd "$repository_root"
run_quiet compile timeout --signal=TERM --kill-after=10s 600 \
  cargo +1.93.0 zigbuild --locked --release --target "$target"
python3 - <<'PY'
import hashlib
import json
import os
import pathlib
import struct

target = os.environ["CLERK_DARWIN_TARGET"]
binary = pathlib.Path(os.environ["CLERK_DARWIN_DIRECTORY"]) / "target" / target / "release/clerk"
raw = binary.read_bytes()
expected_cpu, arch = {
    "x86_64-apple-darwin": (0x01000007, "x86_64"),
    "aarch64-apple-darwin": (0x0100000C, "arm64"),
}[target]
header = struct.unpack("<IIIIIIII", raw[:32]) if len(raw) >= 32 else None
assert header and header[0] == 0xFEEDFACF and header[1] == expected_cpu and header[3] == 2
print(json.dumps({
    "target": target,
    "outcome": "built",
    "type": "Mach-O 64-bit executable",
    "arch": arch,
    "bytes": len(raw),
    "sha256": hashlib.sha256(raw).hexdigest(),
    "sdk": "none",
    "runtime": "not_run",
}, sort_keys=True))
PY
mkdir -p "target/$target/release"
install -m 755 "$workdir/target/$target/release/clerk" "target/$target/release/clerk"
