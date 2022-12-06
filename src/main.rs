//mod examples;

// fn main() {
    // println!("Hello, world!");
    // examples::array_vector::array_vector();
    // examples::guess_game::guess_game();
    // examples::simple_web_service::get();
//}

//#![feature(proc_macro_hygiene, decl_macro)]

use rocket::

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}