# file converters

## Setup

```shell
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# run eg: profile_stats
## option 1: (build then run)
cargo build --bin profile_stats --release
target/release/profile_stats {run|parse}

## option 2: (build and run directly)
cargo run --bin profile_stats -- {run|parse}
```

## profile_stats

### help

```trycmd
$ profile_stats run --help
Usage: profile_stats run [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>
          
  -s, --s-type <S_TYPE>
          [default: weekly] [possible values: weekly, monthly, quarterly]
  -k, --key <KEY>
          [default: "27 Mar - 2 Apr 2023"]
  -m, --migration-file-name <MIGRATION_FILE_NAME>
          [default: ProfileStatsSeedBatchIn0408]
  -d, --do-filter
          
  -r, --run-post-script
          
  -h, --help
          Print help

```

### Run

```trycmd
$ profile_stats run -f fixtures/test.csv -s monthly -m ProfileStatsSeedBatchIn0407
migration sql has been generated to file: migration_output.php

```

### Parse (interactive input)

```shell
$ profile_stats parse
✔ Path of stats xlsx · fixtures/test.csv
✔ Select a stats type · Monthly: 1 Feb - 28 Feb 2023
✔ Filename of this migration · ProfileStatsSeedBatchIn0407
✔ Auto raise phinx migration PR? · false
migration sql has been generated to file: migration_output.php
```

## words

### help

```trycmd
$ words run --help
Usage: words run [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>      
  -w, --w-type <W_TYPE>  [default: mpsc] [possible values: mpsc, rayon]
  -h, --help             Print help

```

### Run

```trycmd
$ words run -f fixtures/words.csv -w mpsc
Generated word counts to file words_output.csv

```

### Parse (interactive input)

```shell
✔ Path of words file (csv) · fixtures/words.csv
✔ Select an analyze type · mpsc
Generated word counts to file words_output.csv
```
