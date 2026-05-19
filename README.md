<h1 align="center">Axum Template</h1>

A Cargo workspace template for building Axum microservices with Clean Architecture, PostgreSQL, and pre-configured tooling.

## Quick Start

```sh
task install_tools        # install Rust/Node tools
task start_infra          # start PostgreSQL
task db:migrate           # run migrations
cargo run -p user-service # start server
```

## Structure

```
crates/
  app-core/       Shared infrastructure (config, error, db, auth, validation)
  app-user/       User domain (entity, dto, repository, service, handler, routes)
  app-testing/    Testcontainers helpers for integration tests
services/
  user-service/   Binary entrypoint
deployments/
  docker-compose/ Infrastructure (PostgreSQL)
```

## Adding a New Service

1. Create `crates/app-<domain>/` with entity, dto, repository, service, handler, routes, migrations
2. Create `services/<name>/` with `AppState` + `main.rs`
3. Add both to `[workspace.members]` in root `Cargo.toml`

## Commands

| Command            | Description               |
| ------------------ | ------------------------- |
| `task start_infra` | Start infrastructure      |
| `task stop_infra`  | Stop infrastructure       |
| `task db:migrate`  | Run migrations            |
| `task db:rollback` | Rollback migrations       |
| `task build`       | Build all crates          |
| `task test`        | Run tests                 |
| `task lint`        | Format + clippy           |
| `task deny`        | Security & license checks |

## Technologies & Libraries

- **[tokio-rs/axum](https://github.com/tokio-rs/axum)** - Ergonomic web framework built on Tokio, Tower, and Hyper
- **[tokio-rs/tokio](https://github.com/tokio-rs/tokio)** - Asynchronous runtime for Rust
- **[tower-rs/tower-http](https://github.com/tower-rs/tower-http)** - HTTP middleware (CORS, compression, tracing, timeout)
- **[launchbadge/sqlx](https://github.com/launchbadge/sqlx)** - Async PostgreSQL driver with compile-time checked queries
- **[tokio-rs/tracing](https://github.com/tokio-rs/tracing)** - Structured logging and diagnostics
- **[dtolnay/thiserror](https://github.com/dtolnay/thiserror)** - Derive macro for error types
- **[Keats/validator](https://github.com/Keats/validator)** - Struct validation via derive macros
- **[Keats/jsonwebtoken](https://github.com/Keats/jsonwebtoken)** - JWT encoding and decoding
- **[SergioBenitez/Figment](https://github.com/SergioBenitez/Figment)** - Layered configuration system
- **[allan2/dotenvy](https://github.com/allan2/dotenvy)** - Environment variable loading from .env files
- **[testcontainers/testcontainers-rs](https://github.com/testcontainers/testcontainers-rs)** - Docker-based integration testing
- **[rust-lang/rustfmt](https://github.com/rust-lang/rustfmt)** - Rust code formatter
- **[rust-lang/rust-clippy](https://github.com/rust-lang/rust-clippy)** - Rust linter with pedantic, nursery, and cargo lint groups
- **[nextest-rs/nextest](https://github.com/nextest-rs/nextest)** - Next-generation test runner for Rust
- **[EmbarkStudios/cargo-deny](https://github.com/EmbarkStudios/cargo-deny)** - Security advisories, license compliance, and dependency checks
- **[bnjbvr/cargo-machete](https://github.com/bnjbvr/cargo-machete)** - Unused dependency detection
- **[go-task/task](https://github.com/go-task/task)** - A task runner / simpler Make alternative
- **[evilmartians/lefthook](https://github.com/evilmartians/lefthook)** - Fast and powerful Git hooks manager
- **[conventional-changelog/commitlint](https://github.com/conventional-changelog/commitlint)** - Lint commit messages
