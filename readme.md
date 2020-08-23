# Servify

Run any command as a service.

## Installation

```bash
# install from the latest commit
cargo install --git https://github.com/whoan/servify.git --branch master
```

## Usage

```bash
servify -h
```

```
USAGE:
    servify [FLAGS] [OPTIONS] <COMMAND>

FLAGS:
    -b, --base64     Decodes payload in Base64
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --method <METHOD>    HTTP method for the service (default: GET)
    -p, --port <PORT>        port for the service (default: 8080)
    -u, --uri <URI>          URI for the service (default: /)

ARGS:
    <COMMAND>    Command to be called as a service
```

## Examples

Run this in a terminal:

```bash
servify 'echo Hello World'
```
```
Command: echo "Hello World"
Service: GET http://0.0.0.0:8080/
```

And this in another terminal:

```bash
curl http://0.0.0.0:8080/
```
```
{"status":0,"stderr":"","stdout":"Hello World\n"}
```

### Use payload to provide content (as a file) for the command

Write some information on **data** field of JSON payload, and the content will be written to a file and appended to the command:

```bash
servify -m POST 'sed s/World/Mars/'
```
```
Command: sed "s/World/Mars/"
Service: POST http://0.0.0.0:8080/
```

In another terminal:

```bash
curl http://0.0.0.0:8080/ -H Content-Type:application/json -d"{\"data\": \"Hello World\"}"
```
```
{"status":0,"stderr":"","stdout":"Hello Mars"}
```

### Provide data as Base64

If you need to provide binary or "complex" (in terms of escape characters) data, you can insert it as base 64 in the payload, and use the switch `-b/--base64` to notify servify that should decode the data in advance:

```bash
servify -m POST --base64 'sed s/World/Mars/'
```
```
Command: sed "s/World/Mars/"
Service: POST http://0.0.0.0:8080/
```

In another terminal:

```bash
curl http://0.0.0.0:8080/ -H Content-Type:application/json -d"{\"data\": \"$(base64 -w0 <<<"Hello World")\"}"
```
```
{"status":0,"stderr":"","stdout":"Hello Mars\n"}
```

## TODO

- Learn Rust and make the code better

## License

[MIT](https://github.com/whoan/servify/blob/master/LICENSE)
