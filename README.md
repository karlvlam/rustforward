# rustforward
Port forward tool in Rust

### Usage

```bash
./rustforward [CONFIG_FILE]
```

### Config Format (as example)

- Forward 8080 to hiauntie.com:80
- Forward 8081 to hiauntie.com:443
```
0.0.0.0:8080 hiauntie.com:80
0.0.0.0:8081 hiauntie.com:443
```

### Build

```bash
cargo build --release
```
