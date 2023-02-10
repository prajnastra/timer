# Timer
A simple cli timer with progress bar & sound alert.

## Prerequisites

### Clone the source code

First clone the source code:

```sh
git clone https://github.com/prajnastra/timer.git
cd timer 
```

### Install the Rust compiler with `rustup`

1. Install [`rustup.rs`](https://rustup.rs/).

3. To make sure you have the right Rust compiler installed, run

   ```sh
   rustup override set stable
   rustup update stable
   ```

## Building

### Linux 

```sh
cargo build --release
```

### Install
```sh
mkdir -p ~/.local/bin
cp target/release/timer ~/.local/bin/
```

## Usage
```bash
timer -t 00:00:10 -v 0.5
```
