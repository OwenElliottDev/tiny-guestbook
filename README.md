# Tiny Guestbook

A lightweight, self-hosted guestbook app that lets guests leave star ratings and reviews. Built with Rust and SQLite, the Docker image compresses down to just a few MB.

![Guestbook UI](readme_assets/ui_example.png)

## Features

- Star ratings (1-5) with written reviews
- Sortable entries (by date or name)
- Responsive design for desktop and mobile
- SQLite database
- Multi-architecture Docker images (x86_64 and ARM64)

## Quick Start

Pull and run with Docker Compose:

```yaml
services:
  guestbook:
    image: owenelliottdev/tiny-guestbook:latest
    container_name: tiny-guestbook
    ports:
      - "8080:8080"
    volumes:
      - ./data:/app/data
    restart: unless-stopped
```

```bash
docker compose up -d
```

The guestbook will be available at `http://localhost:8080`. The SQLite database is stored in `/app/data`, so map that volume to persist data across restarts.

## Building from Source

### Docker

```bash
docker build -t guestbook .
docker run -p 8080:8080 guestbook
```

Or use the dev compose file:

```bash
docker compose -f docker-compose-dev.yml build
docker compose -f docker-compose-dev.yml up
```

### Local Development

Requires Rust.

```bash
cargo run
```

## API

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Main guestbook page |
| `GET` | `/sign` | Signing form |
| `POST` | `/api/sign` | Submit a new entry |
| `GET` | `/api/entries?sort=<order>` | Fetch entries (datePostedDesc, datePostedAsc, nameDesc, nameAsc) |
| `GET` | `/api/entry?id=<id>` | Fetch a single entry |
| `DELETE` | `/api/entry?id=<id>` | Delete an entry |
