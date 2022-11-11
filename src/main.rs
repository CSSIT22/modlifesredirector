use rocket::response::Redirect;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Redirector"
}

#[get("/<path>")]
fn redirect(path: &str) -> Redirect{
    // let redirectOrigin: &'static str = env!("WEB_URL","ASD");
    let mut origin = "https://s.thistine.com/".to_owned();
    origin.push_str(path);
    Redirect::to(origin)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
    .mount("/", routes![redirect])
}