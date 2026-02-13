# zotexon
Headless tool to export a Zotero library, using only the web api.

## CLI Arguments
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

