#!/usr/bin/env bash

# /Users/josephjohnston/zpu copy/zpu/bin/compile-library.sh

# what to pass to cargo build -p, e.g. your_lib_ffi
FFI_TARGET=$1
# buildvariant from our xcconfigs
BUILDVARIANT=$2

RELFLAG=
if [[ "$BUILDVARIANT" != "debug" ]]
then
    RELFLAG=--release
fi

set -euvx

if [[ -n "${DEVELOPER_SDK_DIR:-}" ]]
then
    # Assume we're in Xcode, which means we're probably cross-compiling.
    # In this case, we need to add an extra library search path for build scripts and proc-macros,
    # which run on the host instead of the target.
    # (macOS Big Sur does not have linkable libraries in /usr/lib/.)
    export LIBRARY_PATH="${DEVELOPER_SDK_DIR}/MacOSX.sdk/usr/lib:${LIBRARY_PATH:-}"
fi

# IS_SIMULATOR=0
# if [ "${LLVM_TARGET_TRIPLE_SUFFIX-}" = "-simulator" ]
# then
#     IS_SIMULATOR=1
# fi
# IS_SIMULATOR=1



# # $HOME/.cargo/bin/cargo build -p zpu --lib --target aarch64-apple-ios
# # $HOME/.cargo/bin/cargo build -p zpu --lib --target aarch64-apple-darwin
# # $HOME/.cargo/bin/cargo build -p zpu --lib --target x86_64-apple-darwin

# ARCHS_STANDARD=("x86_64")

# for arch in $ARCHS_STANDARD
# do
#     case "$arch" in
#     x86_64)
#         if [ $IS_SIMULATOR -eq 0 ]
#         then
#             echo "Building for x86_64 $RELFLAG, simulator = false" # >&2
#             # exit 2
#             $HOME/.cargo/bin/cargo build -p $FFI_TARGET --lib $RELFLAG --target x86_64-apple-darwin
#         else
#             echo "Building for x86_64 $RELFLAG, simulator = true"
#             # but for ios sim or for macos sim?
#             export CFLAGS_x86_64_apple_ios="-target x86_64-apple-ios"
#             $HOME/.cargo/bin/cargo build -p $FFI_TARGET --lib $RELFLAG --target x86_64-apple-ios
#         fi
#         ;;
#     arm64)
#         if [ $IS_SIMULATOR -eq 0 ]
#         then
#             echo "Building for arm64 $RELFLAG, simulator = false"
#             $HOME/.cargo/bin/cargo build -p $FFI_TARGET --lib $RELFLAG --target aarch64-apple-ios
#         else
#             echo "Building for arm64 $RELFLAG, simulator = true"
#             $HOME/.cargo/bin/cargo build -p $FFI_TARGET --lib $RELFLAG --target aarch64-apple-ios-sim
#         fi
#     esac
# done

for arch in $ARCHS_STANDARD
do
    case "$arch" in
    x86_64)
        echo "note: Building for macos (x86_64) $RELFLAG"
        $HOME/.cargo/bin/cargo build -p $FFI_TARGET --lib $RELFLAG --target x86_64-apple-darwin
        ;;
    arm64)
        echo "note: Building for ios (arm64) $RELFLAG"
        $HOME/.cargo/bin/cargo build -p $FFI_TARGET --lib $RELFLAG --target aarch64-apple-ios
        ;;
    esac
done


# cargo build -p zpu --lib --target aarch64-apple-ios
# cargo build -p zpu --lib --target x86_64-apple-darwin