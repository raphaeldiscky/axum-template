<h1 align="center">Axum Template</h1>

A scalable Axum web API template structured as a Cargo workspace for microservices, with Clean Architecture, PostgreSQL, JWT auth, input validation, and pre-configured tooling.

## Quick Start

Install project tools and dependencies:

```sh
task install_tools
```

Start PostgreSQL and run migrations:

```sh
task db:up
task db:migrate
```

Run the server:

```sh
cargo run -p user-service
```

## Workspace Structure

```
crates/
  app-core/          Shared infrastructure (config, error, db, auth, validation)
  app-user/          User domain (entity, dto, repository, service, handler, routes)
services/
  user-service/      Binary entrypoint
```

Each domain crate follows **Clean Architecture** layers:

```
entity      → Domain model (sqlx::FromRow)
dto         → Request/response types with validation
repository  → Trait + PostgreSQL implementation
service     → Business logic (holds Arc<dyn Repository>)
handler     → Axum handlers
routes      → Router composition with FromRef bounds
```

## API Endpoints

| Method | Path            | Description                   |
| ------ | --------------- | ----------------------------- |
| GET    | /healthz        | Liveness check                |
| GET    | /readyz         | Readiness check (verifies DB) |
| GET    | /api/users      | List all users                |
| POST   | /api/users      | Create a user                 |
| GET    | /api/users/{id} | Get a user                    |
| PUT    | /api/users/{id} | Update a user                 |
| DELETE | /api/users/{id} | Delete a user                 |

## Adding a New Microservice

1. Create `crates/app-<domain>/` — entity, dto, repository, service, handler, routes, migrations
2. Create `services/<name>/` — thin binary with AppState + main.rs
3. Add both to `[workspace.members]` in root `Cargo.toml`
4. Add docker-compose service if a new database is needed

## Configuration

All configuration is via environment variables (see `.env.example`):

| Variable               | Default                                                  | Description                  |
| ---------------------- | -------------------------------------------------------- | ---------------------------- |
| `HOST`                 | 0.0.0.0                                                  | Server bind address          |
| `PORT`                 | 7000                                                     | Server port                  |
| `DATABASE_URL`         | postgres://postgres:postgres@localhost:5432/user_service | PostgreSQL connection string |
| `JWT_SECRET`           | dev-secret-change-in-production                          | JWT signing secret           |
| `JWT_EXPIRATION_HOURS` | 24                                                       | JWT token lifetime           |
| `RUST_LOG`             | info                                                     | Tracing filter               |

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
- **[rust-lang/rustfmt](https://github.com/rust-lang/rustfmt)** - Rust code formatter
- **[rust-lang/rust-clippy](https://github.com/rust-lang/rust-clippy)** - Rust linter with pedantic, nursery, and cargo lint groups
- **[nextest-rs/nextest](https://github.com/nextest-rs/nextest)** - Next-generation test runner for Rust
- **[EmbarkStudios/cargo-deny](https://github.com/EmbarkStudios/cargo-deny)** - Security advisories, license compliance, and dependency checks
- **[bnjbvr/cargo-machete](https://github.com/bnjbvr/cargo-machete)** - Unused dependency detection
- **[go-task/task](https://github.com/go-task/task)** - A task runner / simpler Make alternative
- **[typicode/husky](https://github.com/typicode/husky)** - Git hooks made easy
- **[conventional-changelog/commitlint](https://github.com/conventional-changelog/commitlint)** - Lint commit messages
