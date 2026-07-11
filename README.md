# Defend

Retro-neon space shooter game.

## Quick Start

### Self-Hosting (Docker)
Pull and run the official Docker container:
```bash
docker run -d -p 4504:4504 -v /path/to/appdata:/app/data ubermetroid/defend:latest
```

### Local Development
To run locally, ensure you have Rust and Cargo installed:
```bash
cargo run --bin server
```
