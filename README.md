# rime_userdb_merger

[Rime-ice](https://github.com/iDvel/rime-ice) User Dictionary File Merge Tool

## Usage

```shell
Usage: rime_userdb_merger [OPTIONS] --main <MAIN> --output <OUTPUT>

Options:
  -m, --main <MAIN>      Main file
  -i, --input <INPUT>    Input file
  -o, --output <OUTPUT>  Output file
  -h, --help             Print help
  -V, --version          Print version
```

```shell
cargo run -- -m rime_ice.userdb.txt \
    -i rime_ice.userdb.txt.1 \
    -i rime_ice.userdb.txt.2 \
    -i rime_ice.userdb.txt.3 \
    -o rime_ice.userdb.merged.txt
```

