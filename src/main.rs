extern crate rustless;
extern crate hyper;
extern crate iron;
extern crate rustc_serialize as serialize;
extern crate valico;

#[macro_use]
extern crate clap;

use rustless::{
    Application, Api, Nesting
};
use rustless::json::ToJson;

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

fn main() {

    let opts: Opts = Opts::parse();
    println!("Command: {}", opts.command);
    println!("Uri: {}", opts.uri);
    println!("Method: {}", opts.method);

    let uri = opts.uri;

    let api = Api::build(|api| {

        // Create API according to arument
        api.mount(Api::build(|servify_api| {

            servify_api.get(&uri, |endpoint| {

                // Add description
                endpoint.desc(&( "Get ".to_owned() + &uri));

                endpoint.handle(|client, params| {
                    client.json(&params.to_json())
                })
            });

        }));
    });

    let app = Application::new(api);

    println!("Rustless server started! on http://0.0.0.0:4000/{}", &uri);
    iron::Iron::new(app).http("0.0.0.0:4000").unwrap();
}
