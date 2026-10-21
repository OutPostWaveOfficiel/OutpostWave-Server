# OutpostWave-Server

*Version française : [README.fr.md](README.fr.md).*

Community server of **OutpostWave** — *the outpost of the airwaves*, an open-source, privacy-first
alternative to Discord for gamers.

One binary, two modes: **self-hosted** (no limits, everything configurable) and **official hosting**.

> Status: **early skeleton** — nothing usable yet.

## Run

```sh
cargo run -p ow-server -- --config config/config.toml
```

With Docker:

```sh
docker compose -f docker/compose.yaml up --build
```

## Layout

| Path | Content |
|---|---|
| `crates/` | Server crates (API, gateway, channels, roles, uploads, SFU, bots, moderation, …) |
| `crates/server/` | The `outpostwave-server` binary |
| `config/config.toml` | Commented configuration |
| `bots/`, `plugins/` | Loaded at startup |
| `migrations/`, `docker/`, `tests/`, `docs/` | |

## License

[AGPL-3.0](LICENSE): anyone running a modified version as a service must publish its source.
Contributions require accepting the [CLA](CLA.md).
