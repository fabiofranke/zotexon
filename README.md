# zotexon 🚀

Headless tool to export a Zotero library using only the web API — perfect for servers, CI, containers, devcontainers, and other non‑desktop environments.

## Why zotexon?
- ⚙️ Command-line only — no GUI or desktop dependencies.
- 🌐 Uses only the Zotero web API — no local Zotero desktop required.
- 🔒 API-key based access — simple, portable, and scriptable.
- 🔁 Optional sync mode — automatically export when your library changes.
- 🧩 Minimal runtime footprint — single binary; ideal for CI, containers, Raspberry Pi, and remote development.

> [!TIP]  
> **Example use case — LaTeX in a devcontainer** 🧑‍💻📚
> - Problem: you work on a LaTeX project inside a devcontainer and want your local citations (.bib file) always in sync with your online Zotero library without running the Zotero desktop app.
> - Solution: run zotexon in sync mode inside the devcontainer. The .bib file in your project will be updated automatically whenever the online library changes — seamless citation updates for builds and previews.

## Installation
#### From prebuilt releases (recommended — project is not on crates.io yet):
1. Download the appropriate archive for your platform from the [GitHub Releases page](https://github.com/fabiofranke/zotexon/releases)
2. Unpack and move the `zotexon` binary to your desired location - example on linux:
   ```bash
   tar xzf zotexon-x86_64-unknown-linux-musl.tar.gz
   sudo mv zotexon /usr/local/bin/
   ```
#### Build from source (Rust toolchain needed)
```bash
git clone https://github.com/fabiofranke/zotexon.git
cd zotexon
cargo build --release
# binary at target/release/zotexon
```

## Quick start
1. Create a Zotero API key with read access: https://www.zotero.org/settings/keys/new
2. Single export:
    ```bash
    zotexon --api-key YOUR_KEY --output library.bib
    ```
3. Continuous sync (keeps running, re-exports on changes):
    ```bash
    zotexon --api-key YOUR_KEY --output library.bib --sync
    ```
> [!WARNING]  
> **Caveats**
> - The output file is overwritten on every export.  
>   Do **not** edit or add your own entries to the exported `.bib` – any manual
>   changes will be lost the next time zotexon runs.
> - Ensure the directory containing the output file exists and is writable by the user
>   running zotexon.

## Usage
<!-- cli-help-start -->
```console
$ zotexon --help
A command-line tool to export a Zotero library to a file.

Usage: zotexon [OPTIONS] --api-key <API_KEY> --output <OUTPUT>

Options:
      --api-key <API_KEY>      Zotero API Key with read access to your library. Generate a key in your Zotero settings: https://www.zotero.org/settings/keys/new
  -o, --output <OUTPUT>        File that the library will be exported to
      --format <FORMAT>        Format to be used for the export [default: biblatex] [possible values: biblatex, bibtex]
      --sync                   Let the program listen for changes in the Zotero library and automatically export on every change. Program will run until interrupted (e.g. with Ctrl+C)
      --log-level <LOG_LEVEL>  Set the verbosity of the log output [default: info] [possible values: off, error, warn, info, debug, trace]
  -h, --help                   Print help
  -V, --version                Print version
```
<!-- cli-help-end -->

## Contributing & License
- Contributions welcome — open an issue or PR on GitHub.
- Licensed under MIT.

#### Happy exporting! ✨
