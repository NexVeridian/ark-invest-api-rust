A REST API for ARK Invest holdings data, written in rust using [axum](https://github.com/tokio-rs/axum), Redoc/Swagger through [Aide](https://github.com/tamasfe/aide), and parquet using [polars](https://github.com/pola-rs/polars)

[api.NexVeridian.com](https://api.NexVeridian.com)

Not affiliated with Ark Invest

# Install
Copy docker-compose.yml

Create data folder next to docker-compose.yml, `data\parquet\*.parquet` with the ticker in all caps `ARKK.parquet`, get the data from [api.NexVeridian.com](https://api.NexVeridian.com) or [ark-invest-api-rust-data](https://github.com/NexVeridian/ark-invest-api-rust-data)

```
├───data
│   └───parquet
│   	└───*.parquet
├───docker-compose.yml
```

`docker compose up --pull always`

If not using nginx, set environment NGINX = false in docker compose

# Dev Install
## Dev Containers
Install docker, vscode and the [Dev Containers Extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

`git clone`

`Ctrl+Shift+P` **Dev Containers: Open Folder in Container**

Place data in `data\parquet\*.parquet` with the ticker in all caps `ARKK.parquet`, get the data from [api.NexVeridian.com](https://api.NexVeridian.com) or [ark-invest-api-rust-data](https://github.com/NexVeridian/ark-invest-api-rust)

`cargo run`

## Docker Compose
`git clone`

Place data in `data\parquet\*.parquet` with the ticker in all caps `ARKK.parquet`, get the data from [api.NexVeridian.com](https://api.NexVeridian.com) or [ark-invest-api-rust-data](https://github.com/NexVeridian/ark-invest-api-rust)

`docker compose build && docker compose up`

Remove the cargo cache for buildkit with `docker builder prune --filter type=exec.cachemount`

# License
All code in this repository is dual-licensed under either [License-MIT](./LICENSE-MIT) or [LICENSE-APACHE](./LICENSE-Apache) at your option. This means you can select the license you prefer.

[Why dual license](https://github.com/bevyengine/bevy/issues/2373)
