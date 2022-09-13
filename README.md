# Memtester-rs

![pipeline](https://github.com/aeyno/memtester-rs/actions/workflows/rust.yml/badge.svg)

A Rust implementation of [memtester](https://pyropus.ca./software/memtester/) to test the memory of a system.
It uses the [libmemtester crate](https://crates.io/crates/libmemtester).

## 💻 Installation

### 📦 From cargo

```bash
cargo install memtester
```

### 🔨 From source

```bash
git clone
cd memtester-rs
cargo install --path .
```

## ⌨ Usage examples

To test 1 GiB of memory:

```bash
memtester-rs 1G
```

To test 2 GiB of memory with 4 cycles:

```bash
memtester-rs 1G --cycles 4
```

## 📃 License

This project is licensed under the GPLv3 licence - see the [LICENSE](LICENSE) file for details
