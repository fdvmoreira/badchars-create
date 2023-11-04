# Bad Characters creator

This simple CLI application will help you with bad character generation when creating buffer overflow payloads.

## Installation

### Download Source code

```sh
git clone https://github.com/fdvmoreira/badchars_create.git && cd badchars-create
```

or

```sh
wget https://github.com/fdvmoreira/badchars_create/archive/refs/heads/main.zip -O badchars-create.zip
```

```sh
unzip badchars-create.zip
```

```sh
cd badchars-create
```

### Compiling the application
1. Build and change into the artefact's directory
```sh
cargo build --release && cd target/release
```
2. Run the application
```sh
./badchars-create
```

### Download executable

Search in the release tags


## Usage

- _Display the help information_
```sh
./badchars_create --help
```
- _Exclude 3 hexadecimal characters [\x00, \x45, \xf0]_

```sh
./badchars_create --skip "\x00\x45\xf0"
```

- _Change the output format (defaults to standard output)_
```sh
./badchars_create --format=string
```

- _The programming language in which the generated output will be used_
```sh
./badchars_create --language python

```

## Required Dependencies

```yaml
argparse = "0.2.2"
colored = "2.0.0"
```

## License

**MIT License**

Read the [LICENSE](LICENSE)

