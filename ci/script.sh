set -euxo pipefail

main() {
    cargo check --target $TARGET
    cargo build --target $TARGET --release
    cargo build --target $TARGET --examples

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo test --target $TARGET
    fi
}

main
