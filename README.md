# Bad characters creator

CLI application written in rust to help with bad characters generation when creat ing buffer overflow payloads.

## Installation

### Download Source code

```bash
git clone https://github.com/fdvmoreira/badchars_create.git && cd badchars-create
```

or

```bash
wget https://github.com/fdvmoreira/badchars_create/archive/refs/heads/main.zip -O badchars-create.zi
```

```bash
unzip badchars-create.zip
```

```bash
cd badchars-create
```

### Compilling the application
1. Build and change into artifact's directory
```bash
cargo build --release && cd target/release
```
2. Run the application
```bash
./badchars-create
```

### Download executable

```
# Search in the release tags
```

## Usage

- _Display the help information_
```bash
./badchars_create --help
```
- _Exclude 3 hexadecimal characters [\x00, \x45, \xf0]_

```bash
./badchars_create --skip "\x00\x45\xf0"
```

- _Change the output format (defaults to standard output)_
```bash
./badchars_create --format=string
```

- _The programming language in which the generated output will be used_
```bash
./badchars_create --language python

```

## Dependencies

```yaml
argparse = "0.2.2"
colored = "2.0.0"
```

## License

**MIT License**

Read the [LICENSE](LICENSE) file.

