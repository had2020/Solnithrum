#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// http://127.0.0.1:8000/hello/world
#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

// http://127.0.0.1:8000/wave/had/100
#[get("/<name>/<age>", rank = 2)]
fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

#[post("/send", format = "json", data = "<msg>")]
fn send(msg: &str) {
    Json(MsgAck {
        ok: true,
        echoed: msg.0,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![world])
        .mount("/wave", routes![wave])
}
