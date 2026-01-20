# Tiny Guestbook

A tiny app that lets guests (at wherever you are hosting) leave reviews.

## Deployment

Refer to the `docker-compose.yml` for an example of how to deploy, the database is stored in `/app/data` so it must be mapped to the host for persistance.

## Building with Docker

### Build

```bash
docker build -t guestbook .
```

### Run
```
docker run -p 8080:8080 guestbook
```

### Dev docker compose

Or use docker compose

```bash
docker compose -f docker-compose-dev.yml build
docker compose -f docker-compose-dev.yml up
```

## Development

Simply run the app with `cargo` for development.

```
cargo run
```