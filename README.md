# Bad characters creator

CLI application written in rust to help with bad characters generation when creat ing buffer overflow payloads.

## Installation

### Download Source code

```
git clone https://github.com/fdvmoreira/badchars_create.git
cd badchars-create
```

or

```
wget https://github.com/fdvmoreira/badchars_create/archive/refs/heads/main.zip -O badchars-create.zi
unzip badchars-create.zip
cd badchars-create
```

### Compilling the application

```
# build and change into artifact directory
cargo build --release && cd target/release
#run the application
./badchars-create
```

### Download executable

```
# Search in the release tags
```

## Usage

```
# Display the help information
./badchars_create --help

# exclude 3 hexadecimal characters [\x00, \x45, \xf0]
./badchars_create --skip "\x00\x45\xf0"

# change the output format (defaults to standard output)
./badchars_create --format=string

# the programming language this output will be used for
./badchars_create --language python

```

## Dependencies

```
argparse = "0.2.2"
colored = "2.0.0"
```

## License

**MIT License**

read the [LICENSE](LICENSE) file

## Author

Flavio Moreira <flaviomoreira@fdvmlab.com>

My website - <https://fdvmlab.com>
