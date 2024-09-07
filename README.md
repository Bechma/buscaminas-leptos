# Run Buscaminas (minesweeper in spanish)

First up, make sure Rust is installed and up-to-date ([see here if you need instructions](https://www.rust-lang.org/tools/install)).

If you don’t have it installed already, you can install the "Trunk" tool for running Leptos CSR sites by running the following on the command-line:

```bash
git clone git@github.com:Bechma/buscaminas-leptos.git
cd buscaminas-leptos
cargo install trunk
trunk serve
```

# Build for production

```bash
trunk build --dist dist --release
```
