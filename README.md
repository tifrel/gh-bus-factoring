## Usage

Token via flag:

```sh
cargo run --release -- -l rust -c 50 -t $(cat gh_token)
```

Token via environment:

```sh
GH_AUTHTOKEN=$(cat gh_token) cargo run --release -- -l rust -c 50
```

Long-form flags:

```sh
cargo run --release -- --language rust --project_count 50 --token $(cat gh_token)
```

## Dependencies

- tokio: For bonus points and speed-up
- reqwest: For making HTTP requests
- serde: Translating JSON responses into rust structs
- clap: The go-to for parsing CLI arguments, with auto-generated help texts :)
