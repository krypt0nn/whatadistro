<h1 align="center">🦀 whatadistro</h1>

Rust library to identify your linux distribution

## Examples

### Get current distro name

```rust
let distro = whatadistro::identify()
    .expect("Failed to parse os-release file");

println!("Your distro name is {}", distro.name());
```

### Compare current distro with some another

Can be used in an app to display a command to download optional dependency. For example, show `apt install git` for every debian-based system, which are Linux Mint, Ubuntu, Deepin, etc.

```rust
let status = whatadistro::identify()
    .map(|distro| distro.is_similar("arch")) // whatadistro::Distro::Arch can be used as well
    .unwrap_or(false);

println!("Is current system arch-based: {:?}", status);
```

Author: [Nikita Podvirnyy](https://github.com/krypt0nn)

Licensed under [MIT](LICENSE)
