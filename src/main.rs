#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Bing!"
}

#[get("/")]
fn world() -> &'static str {
    "Hello, Github!"
}

#[get("/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![world])
        .mount("/name", routes![hello])
}