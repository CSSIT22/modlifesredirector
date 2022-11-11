
#[macro_use] extern crate rocket;

mod index;
mod pathredirect;



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index::index])
    .mount("/", routes![pathredirect::redx])
}