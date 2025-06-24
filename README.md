# Sthin

A high-performance URL shortening service built with Rust, Actix-web, SQLx, Redis, and modern observability tooling.

## Features

- **Shorten URLs**: Generate short codes for long URLs.
- **URL Redirection**: Redirect users from short codes to original URLs.
- **URL Statistics**: Track and retrieve usage statistics for shortened URLs.
- **Validation**: Robust input validation and error handling.
- **Observability**: Structured logging and tracing with Bunyan formatting.
- **Persistence**: PostgreSQL for durable storage, Redis for caching.
- **Configurable**: Environment-based configuration with YAML files.
- **Async**: Fully asynchronous using Tokio runtime.

## Project Structure

```
.
├── src/
│   ├── configuration/      # App configuration loading and management
│   ├── error.rs            # Application-wide error types
│   ├── features/           # Business logic (URLs, stats, etc.)
│   ├── infrastructure/     # Database, cache, telemetry, server setup
│   ├── lib.rs              # Library entry point
│   └── main.rs             # Binary entry point
├── configs/                # YAML configuration files
├── migrations/             # SQLx database migrations
├── init-scripts/           # DB initialization scripts
├── Cargo.toml              # Rust crate manifest
├── docker-compose.yml      # Multi-service orchestration
└── README.md               # This file
```

## Getting Started

### Prerequisites

- [Rust (1.76+)](https://rustup.rs/)
- [PostgreSQL](https://www.postgresql.org/)
- [Redis](https://redis.io/)
- [Docker & Docker Compose](https://docs.docker.com/compose/) (optional, for local development)

### Configuration

Configuration is managed via YAML files in [`configs/`](configs/):

- `base.yaml`: Default settings
- `local.yaml`: Local development overrides
- `production.yaml`: Production overrides

You can set the config environment via the `APP_ENVIRONMENT` environment variable (`local`, `production`, etc).

Example `.env`:

```
APP_ENVIRONMENT=local
DATABASE_URL=postgres://user:password@localhost/sthin
REDIS_URI=redis://localhost:6379
```

### Database Setup

Run migrations using SQLx:

```sh
cargo install sqlx-cli
sqlx database setup
sqlx migrate run
```

Or use the provided scripts in [`init-scripts/`](init-scripts/).

### Running the Application

#### With Cargo

```sh
cargo run --bin sthin-runner
```

#### With Docker Compose

```sh
docker-compose up --build
```

### API Endpoints

- `POST   /api/shorten` — Create a new short URL
- `GET    /api/shorten/{code}` — Retrieve original URL by short code
- `PATCH  /api/shorten/{code}` — Update a shortened URL
- `DELETE /api/shorten/{code}` — Delete a shortened URL

See [src/infrastructure/server/routes.rs](src/infrastructure/server/routes.rs) for details.

### Observability

- Logging and tracing are enabled via [Telemetry](src/infrastructure/telemetry/telemetry.rs).
- Logs are output in Bunyan JSON format for easy ingestion.

### Testing

Run all tests:

```sh
cargo test
```

## Dependencies

- [actix-web](https://crates.io/crates/actix-web) — Web framework
- [tokio](https://crates.io/crates/tokio) — Async runtime
- [sqlx](https://crates.io/crates/sqlx) — Async SQL toolkit
- [redis](https://crates.io/crates/redis) — Redis client
- [tracing](https://crates.io/crates/tracing) — Structured logging
- [serde](https://crates.io/crates/serde) — Serialization
- [thiserror](https://crates.io/crates/thiserror), [anyhow](https://crates.io/crates/anyhow) — Error handling
- [validator](https://crates.io/crates/validator) — Input validation

## Contributing

1. Fork the repo
2. Create your feature branch (`git checkout -b feature/foo`)
3. Commit your changes
4. Push to the branch
5. Open a pull request

## License

This project is licensed under the MIT License.

---

\*Roadmap from https://roadmap.sh/projects/url-shortening-service
