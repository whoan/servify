extern crate rustless;
extern crate jsonway;

#[macro_use]
extern crate clap;

use rustless::framework::endpoint;
use rustless::{
    Application, Api, Nesting
};

/// Call any command as a service
#[derive(Clap)]
struct Opts {
    /// Command to be called as a service
    command: String,

    /// [Optional] URI for the service
    #[clap(short = "u", long = "uri", default_value = "/")]
    uri: String,

    /// [Optional] HTTP method for the service
    #[clap(short = "m", long = "method", default_value = "GET")]
    method: String
}

pub struct CommandStatus  {
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
    let opts: Opts = Opts::parse();
    println!("Command: {}", opts.command);
    println!("Uri: {}", opts.uri);
    println!("Method: {}", opts.method);

    let command = opts.command;
    let uri = opts.uri;
    let method = opts.method.to_uppercase();

    let api = Api::build(|api| {

        // Create API according to arument
        api.mount(Api::build(|servify_api| {

            let closure = |endpoint: &mut endpoint::Endpoint| {
                endpoint.handle(move |client, _params| {
                    let command_status = run_command(&command);
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

    println!("Servify server started on http://0.0.0.0:4000/{}", &uri);
    iron::Iron::new(app).http("0.0.0.0:4000").unwrap();
}
