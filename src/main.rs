use clap::{Arg, App};
use tempfile::NamedTempFile;
use std::io::{Write};
use rustless::framework::endpoint;
use rustless::{
    Application, Api, Nesting
};

struct CommandStatus {
    status: i32,
    stdout: String,
    stderr: String,
}

fn run_command(command : &String) -> CommandStatus {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("failed to execute process");

    CommandStatus {
        status: output.status.code().unwrap(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    }
}

fn main() {
    let opts = App::new("servify")
        .about("Run any command as a service")
        .version(clap::crate_version!())
        .arg(Arg::with_name("COMMAND")
            .required(true)
            .index(1)
            .help("Command to be called as a service"))
        .arg(Arg::with_name("base64")
            .short("b")
            .long("base64")
            .help("Decodes payload in Base64"))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .takes_value(true)
            .help("port for the service (default: 8080)"))
        .arg(Arg::with_name("uri")
            .short("u")
            .long("uri")
            .value_name("URI")
            .takes_value(true)
            .help("URI for the service (default: /)"))
        .arg(Arg::with_name("method")
            .short("m")
            .long("method")
            .value_name("METHOD")
            .takes_value(true)
            .help("HTTP method for the service (default: GET)"))
        .get_matches();

    let command = String::from(opts.value_of("COMMAND").unwrap());
    let port: i32 = opts.value_of("port").unwrap_or("8080").parse().unwrap();
    let uri = opts.value_of("uri").unwrap_or("");
    let method = opts.value_of("method").unwrap_or("GET").to_uppercase();
    let base64 = opts.is_present("base64");

    let url = format!("0.0.0.0:{}", port);
    println!("Command: {}", command);
    println!("Service: {} http://{}/{}", method, url, uri);

    let api = Api::build(|api| {

        // Create API according to parameters
        api.mount(Api::build(|servify_api| {

            let closure = |endpoint: &mut endpoint::Endpoint| {
                endpoint.handle(move |client, params| {
                    let data = match params.as_object() {
                        None => None,
                        Some(payload) => {
                            match payload.get("data") {
                                None => None,
                                Some(content) => {
                                    if base64 {
                                        Some(base64::decode(&content.to_string().trim_matches('"')))
                                    } else {
                                        Some(std::result::Result::Ok(content.to_string().trim_matches('"').to_string().into_bytes()))
                                    }
                                }
                            }
                        },
                    };

                    let mut tmp_file = NamedTempFile::new().unwrap();
                    let mut request_command = command.clone();
                    match data {
                        None => None,
                        Some(result) => {
                            match result {
                                Err(error) => println!("Error at parsing data: {}", error),
                                Ok(result) => {
                                    tmp_file.write_all(&result).expect("Something did not work well");
                                    request_command.push(' ');
                                    request_command.push_str(tmp_file.path().to_str().unwrap());
                                }
                            }
                            Some("")
                        }
                    };

                    println!("Running {}", request_command);
                    let command_status = run_command(&request_command);
                    let json = jsonway::object(|json| {
                        json.set("status", command_status.status);
                        json.set("stdout", command_status.stdout);
                        json.set("stderr", command_status.stderr);
                    });
                    client.json(&json.unwrap())
                })
            };
            // PATCH method will be available in next release of rustless
            let callback = if method == "GET" {
                Api::get
            } else if method == "POST" {
                Api::post
            } else if method == "PUT" {
                Api::put
            } else if method == "DELETE" {
                Api::delete
            } else {
                panic!("Unknown HTTP method. Please provide GET|POST|PUT|DELETE or none")
            };
            callback(servify_api, &uri, closure);
        }));
    });

    let app = Application::new(api);
    iron::Iron::new(app).http(url).unwrap();
}
