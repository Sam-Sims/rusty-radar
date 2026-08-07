set default-list

mod firmware 'firmware/Justfile'

fmt:
    cargo +nightly fmt --all
