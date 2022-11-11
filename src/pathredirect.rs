use rocket::response::Redirect;

#[get("/<path>")]
pub fn redx(path: &str) -> Redirect{
    let mut origin = "https://s.thistine.com/".to_owned();
    origin.push_str(path);
    Redirect::to(origin)
}