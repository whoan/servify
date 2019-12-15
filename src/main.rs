extern crate rustless;
extern crate hyper;
extern crate iron;
extern crate rustc_serialize as serialize;
extern crate valico;

use rustless::{
    Application, Api, Nesting
};
use rustless::json::ToJson;

fn main() {

    let arguments: Vec<String> = std::env::args().collect();
    let url = if arguments.len() > 1 {
        &arguments[1]
    } else {
        "/"
    };

    let api = Api::build(|api| {

        // Create API according to arument
        api.mount(Api::build(|chats_api| {

            chats_api.get(url, |endpoint| {

                // Add description
                endpoint.desc(&( "Get ".to_owned() + &url));

                endpoint.handle(|client, params| {
                    client.json(&params.to_json())
                })
            });

        }));
    });

    let app = Application::new(api);

    println!("Rustless server started! on 4000");
    iron::Iron::new(app).http("0.0.0.0:4000").unwrap();

}
