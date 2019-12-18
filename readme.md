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
    servify [OPTIONS] <command>

ARGS:
    <command>    Command to be called as a service

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --method <method>    [Optional] HTTP method for the service [default: GET]
    -u, --uri <uri>          [Optional] URI for the service [default: /]
```

## Example

```bash
servify 'echo "Hello World"'
```

Output:

```
Command: echo "Hello World"
Service: GET http://0.0.0.0:4000/
```

-------

In another terminal:

```bash
curl http://0.0.0.0:4000/
```

Output:

```
{"status":0,"stderr":"","stdout":"Hello World\n"}
```

### Use payload to provide content (as a file) for the command

Write some information on **data** field of JSON payload, and the content will be written to a file and appended to the command:

```bash
servify -m POST 'sed "s/World/Mars/"'
```

Output:

```
Command: sed "s/World/Mars/"
Service: POST http://0.0.0.0:4000/
```

------

In another terminal:

```bash
curl http://0.0.0.0:4000/ -H Content-Type:application/json -d'{"data": "Hello World"}'
```

Output:

```
{"status":0,"stderr":"","stdout":"\"Hello Mars\""}
```

## License

[MIT](https://github.com/whoan/servify/blob/master/LICENSE)
