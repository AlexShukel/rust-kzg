#!/bin/bash

set -e

LIB=libblst_from_scratch.a

print_msg () {
  echo "[*]" "$1"
}

###################### parallel configuration ######################

parallel=false

while [[ -n $# ]]; do
  case $1 in
    -p|--parallel)
      parallel=true
      ;;
    -*)
      echo "Unknown parameter: $1"
      exit 1
      ;;
    *)
      break
      ;;
  esac;
  shift
done

###################### building static libs ######################

print_msg "Compiling libblst_from_scratch"
if [[ "$parallel" = true ]]; then
  print_msg "Using parallel version"
  cargo rustc --release --crate-type=staticlib --features=parallel
else
  print_msg "Using non-parallel version"
  cargo rustc --release --crate-type=staticlib
fi

###################### cloning c-kzg-4844 ######################

print_msg "Cloning c-kzg-4844"
git clone https://github.com/ethereum/c-kzg-4844.git
cd c-kzg-4844 || exit 1
git -c advice.detachedHead=false checkout "$C_KZG_4844_GIT_HASH"
git submodule update --init

###################### rust benchmarks ######################

print_msg "Patching rust binding"
git apply < ../rust.patch
cd bindings/rust || exit 1

print_msg "Running rust benchmarks"
cargo bench
cd ../..

###################### java benchmarks ######################

print_msg "Patching java binding"
git apply < ../java.patch
cd bindings/java || exit 1

print_msg "Running java benchmarks"
make build benchmark
cd ../..

###################### go benchmarks ######################

print_msg "Patching go binding"
git apply < ../go.patch

print_msg "Running go benchmarks"
cd bindings/go || exit 1
go test -run ^$ -bench .
cd ../../..

###################### cleaning up ######################

print_msg "Cleaning up"
rm -rf c-kzg-4844
