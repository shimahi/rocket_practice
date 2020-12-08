#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

// /foo/bar
#[get("/bar")]
fn bar() -> &'static str {
    "FooBar"
}

// 動的なルーティング
#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} ({})", name, age)
}

// ランク
#[get("/<age>")]
fn hello_age(age: u8) -> String {
    format!("Hello, ({})", age)
}
#[get("/<name>", rank = 2)]
fn hello_name(name: String) -> String {
    format!("Hello, {}", name)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello, hello_age, hello_name])
        .mount("/foo", routes![bar])
        .launch();
}
