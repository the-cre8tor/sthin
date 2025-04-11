Here's how to effectively use cargo-watch with an Actix-web project:

1. First, install cargo-watch:

```bash
cargo install cargo-watch
```

2. Basic Usage for Development:

```bash
# Watch and run
cargo watch -x run

# Watch, build, and run
cargo watch -x 'build --bin my_app' -x 'run --bin my_app'

# Watch with tests
cargo watch -x check -x test -x run
```

3. Best Practice Configuration:

```bash
# Most complete development setup
cargo watch -q -c -w src/ -x run -x test

# What each flag means:
# -q: Quiet mode (less output)
# -c: Clear screen before each run
# -w src/: Watch the src directory
# -x run: Execute `cargo run`
# -x test: Execute `cargo test`
```

4. With Environment Variables:

```bash
# Create .cargo/config.toml
[env]
RUST_LOG = "debug"
DATABASE_URL = "postgres://localhost/mydb"

# Then run
cargo watch -q -c -w src/ -x 'run --bin my_app'
```

5. Full Development Setup Example:

```rust
// main.rs
use actix_web::{web, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup logging
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    log::info!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(web::scope("/api").configure(routes::config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

6. Script for Development:

```bash
# Create a dev.sh script
#!/bin/bash

# Export environment variables
export RUST_LOG=debug
export RUST_BACKTRACE=1
export DATABASE_URL=postgres://localhost/mydb

# Run with cargo-watch
cargo watch -q -c -w src/ \
    -x clippy \
    -x 'test -- --nocapture' \
    -x 'run --bin my_app'
```

7. With Hot Reload:

```bash
# Install systemfd and cargo-watch
cargo install systemfd cargo-watch

# Run with hot reload
systemfd --no-pid -s http::8080 -- cargo watch -x run
```

8. Development with Different Configurations:

```bash
# Watch specific files/directories
cargo watch -w src -w Cargo.toml -x run

# Watch and run multiple commands
cargo watch \
    -w src \
    -w tests \
    -w Cargo.toml \
    -x check \
    -x 'test -- --nocapture' \
    -x 'run --bin my_app'
```

9. With Database Migrations:

```bash
# Watch and run migrations
cargo watch \
    -w migrations \
    -x 'sqlx migrate run' \
    -x run
```

10. VS Code tasks.json Configuration:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "dev",
      "type": "shell",
      "command": "cargo watch -q -c -w src/ -x run",
      "group": {
        "kind": "build",
        "isDefault": true
      }
    }
  ]
}
```

11. Development with Multiple Binaries:

```bash
cargo watch \
    -w src \
    -x 'run --bin api_server' \
    -x 'run --bin background_worker'
```

Best Practices:

1. Use Clear Screen:

```bash
cargo watch -c -x run
```

2. Watch Specific Directories:

```bash
cargo watch -w src -w tests -w migrations -x run
```

3. Include Multiple Commands:

```bash
cargo watch \
    -x check \
    -x clippy \
    -x test \
    -x run
```

4. With Environment Setup:

```bash
#!/bin/bash
# dev.sh

# Load environment variables
source .env

# Run development server
cargo watch \
    -q -c \
    -w src/ \
    -w tests/ \
    -w migrations/ \
    -x clippy \
    -x 'test -- --nocapture' \
    -x 'run --bin my_app'
```

Remember:

- Keep the feedback loop quick
- Include relevant checks
- Watch appropriate directories
- Use clear screen for readability
- Configure proper logging
- Include necessary environment variables

This setup provides a good development experience with:

- Automatic rebuilding
- Test running
- Code checking
- Clear output
- Quick feedback
