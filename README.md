# Working Through Zero2Prod

Run with cargo-watch: `cargo watch -x check -x test -x run`

Linting with clippy: `cargo clippy`

Format: `cargo fmt`

Audit with cargo-audit: `cargo audit`
NOTE: had to install openssl with brew for this and a couple other installs. See this for similar issue https://docs.rs/crate/openssl-sys/0.9.36

Installed sqlx cli for db: `cargo install sqlx-cli --no-default-features --features native-tls,postgres`

Use postgres cli psql as part of init_db script: https://www.timescale.com/blog/how-to-install-psql-on-mac-ubuntu-debian-windows/

Running DB: `scripts/init_db.sh` - if Docker already running `SKIP_DOCKER=true scripts/init_db.sh`
