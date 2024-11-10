# globalgit
Tool to list all git repos in a directory and show a combined commit log

## Installation

#### rust
To install this tool, you need to have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### globalgit
Now you can install globalgit using cargo
```sh
cargo install --git https://github.com/Progrogogg/globalgit
```

## usage
#### list repos 
```sh
globalgit repos <root_directory>
```

#### show combined log
```sh
globalgit log <root_directory>
```

#### show combined log filtered by author
```sh
globalgit log <root_directory> [<author_name>]
```
