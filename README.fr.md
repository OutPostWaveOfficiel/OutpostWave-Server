# OutpostWave-Server

Serveur communautaire d'**OutpostWave** — *l'avant-poste des ondes*, alternative open-source à Discord,
orientée gamers et respectueuse de la vie privée.

Un seul binaire, deux modes : **auto-hébergé** (aucune limite, tout est réglable) et **hébergement officiel**.

> État : **squelette** — rien d'utilisable pour l'instant.

## Lancer

```sh
cargo run -p ow-server -- --config config/config.toml
docker compose -f docker/compose.yaml up --build
```

## Licence

[AGPL-3.0](LICENSE) : toute version modifiée proposée en service doit publier son code source.
Les contributions nécessitent l'acceptation du [CLA](CLA.md).
