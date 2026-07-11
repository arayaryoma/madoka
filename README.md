# madoka

Madoka is a small static-file web server written in Rust (hyper 1.0-rc + tokio). It serves files from per-virtual-host root directories selected by the request's `Host` header, based on a YAML config file (`madoka.conf.yaml` by default, overridable with `--config`/`-c`).

## Commands

- Build: `cargo build` (or `make build_debug` / `make build_release`)
- Lint: `make lint` (runs `cargo clippy -- -D warnings`; CI fails on any warning)
- Format check: `make format_check` (runs `cargo fmt --all -- --check`; enforced in CI)
- Test all: `cargo test`
- Test one: `cargo test <test_name>` (e.g. `cargo test test_read_config_from_file`)
- Run locally: `cargo run -- --config playground/madoka.conf.yaml` — note the sample config's `root` paths are relative to the CWD, so run from `playground/` or adjust paths

Tests read `playground/madoka.conf.yaml` from the repo root as a fixture; keep that file and the expected `Config` in `src/config.rs` tests in sync.

## Architecture

Three source files in `src/`:

- `main.rs` — CLI parsing (clap), tokio TCP accept loop spawning one task per connection, and the `router` function: extracts the `Host` header, finds the matching `Host` entry in the shared `Arc<Config>`, resolves the request path under that host's `root` directory (directories fall back to `index.html`), and serves the file with a guessed MIME type. Only GET is supported; everything else (and missing files/hosts) returns 404.
- `config.rs` — serde/serde_yaml config model: top-level `port`, `root`, and `hosts` list; each host has `host_name`, `root`, and optional `add_header` (a list of single-entry maps so the same header name can appear multiple times, e.g. repeated `origin-trial`). Extra response headers from `add_header` are appended per host.
- `hyper_response_util.rs` — canned 404/500 responses.

## Load testing / playground

`playground/` holds sample sites (`madoka.local`, `blog.madoka.local`) and the sample config. Load testing uses docker compose:

1. `docker compose up -d --build grafana influxdb app` — `app` builds the Dockerfile and serves the playground on port 3001
2. `docker compose run k6 run /scripts/script.js` — k6 scripts live in `load_testing/scripts/`, results go to InfluxDB
3. You can see the Grafana dashboard on http://localhost:3000 (anonymous admin; provisioning in `grafana/`, dashboard JSON in `dashboards/`)

An `nginx_app` service (port 3002) serves the same playground via nginx for comparison.
