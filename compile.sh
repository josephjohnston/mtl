#!/usr/bin/env bash

BUILDVARIANT=$1
PLATFORM=$2

RELFLAG=
if [[ "$BUILDVARIANT" != "debug" ]]
then
    RELFLAG=--release
fi

set -euvx

# if [[ -n "${DEVELOPER_SDK_DIR:-}" ]]
# then
#     # Assume we're in Xcode, which means we're probably cross-compiling.
#     # In this case, we need to add an extra library search path for build scripts and proc-macros,
#     # which run on the host instead of the target.
#     # (macOS Big Sur does not have linkable libraries in /usr/lib/.)
#     export LIBRARY_PATH="${DEVELOPER_SDK_DIR}/MacOSX.sdk/usr/lib:${LIBRARY_PATH:-}"
# fi


for arch in $ARCHS_STANDARD
do
    case "$arch" in
    # x86_64)
    #     echo "note: Building for macos (x86_64) $RELFLAG"
    #     $HOME/.cargo/bin/cargo build -p zpu --lib $RELFLAG --target aarch64-apple-darwin
    #     ;;
    arm64)
        case "$PLATFORM" in
        ios)
            echo "note: Building for aarch64-apple-ios $RELFLAG"
            $HOME/.cargo/bin/cargo build -p zpu --lib $RELFLAG --target aarch64-apple-ios
            ;;
        macos)
            echo "note: Building for aarch64-apple-darwin $RELFLAG"
            $HOME/.cargo/bin/cargo build -p zpu --lib $RELFLAG --target aarch64-apple-darwin
            ;;
        esac
        ;;
    esac
done



# cargo build -p zpu --lib --target aarch64-apple-ios
# cargo build -p zpu --lib --target x86_64-apple-darwin