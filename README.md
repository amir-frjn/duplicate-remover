# duplicate-remover

A full-featured Rust program that identifies and removes duplicate files and directories on your system.  

At a glance:
- Scans your system to discover files.
- Calculates file hashes and stores them in a SQLite database (`files.db`).
- Lets you review duplicates and decide whether to remove them through a simple command-line interface (CLI).

---

## Features
- Fast file scanning with customizable search options.
- Multiple hashing algorithms supported.
- SQLite-based cache for efficient lookups.
- Interactive CLI for reviewing and removing duplicates.
- Option to remove all files with the same hash at once.

---

## Installation
```bash
git clone https://github.com/your-username/duplicate-remover.git
cd duplicate-remover
cargo build --release
```
---

## Usage samples
```bash
duplicate-remover gen-cache [options]
duplicate-remover show
duplicate-remover rm
duplicate-remover rm --hash <hash>
duplicate-remover help
```
