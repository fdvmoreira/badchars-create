# Bad characters creator

CLI application written in rust to help with bad characters generation when creat ing buffer overflow payloads.

## Installation

### Download Source code

```
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

**Display the help information**
```bash
./badchars_create --help
```
**exclude 3 hexadecimal characters [\x00, \x45, \xf0]**

```bash
./badchars_create --skip "\x00\x45\xf0"
```

**change the output format (defaults to standard output)**
```bash
./badchars_create --format=string
```

**The programming language in which the generated output will be used**
```bash
./badchars_create --language python

```

## Dependencies

```bash
argparse = "0.2.2"
colored = "2.0.0"
```

## License

**MIT License**

Read the [LICENSE](LICENSE) file.

