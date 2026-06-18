# forst
[![Crates.io](https://img.shields.io/crates/v/forst.svg)](https://crates.io/crates/forst)
[![License](https://img.shields.io/crates/l/forst.svg)](https://github.com/GennaroBiondi/forst)

forst is a simple CLI tool to display a tree-like graph of a directory

## How to install

### Using cargo (recommended)
```bash
cargo install forst
```

### Cloning the repository
```bash
git clone https://github.com/GennaroBiondi/forst
cd forst
cargo install --path .
```

## How to use
run `forst` to display a tree of your current directory.
you can provide a target directory optionally.

### Options
* `-a` | `--all`: Displays dotfiles too

* `-s` | `--sizes`: Displays the file size too

* `-L` | `--max-depth` `[INTEGER]`: Adds a limit to the depth of the tree

* `--no-unicode`: Doesn't use unicode to display the tree
