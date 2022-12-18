use rocket::response::Redirect;

#[get("/<path>")]
pub fn redx(path: &str) -> Redirect{
    let mut origin = "https://ss.modlifes.me/shortlink/redirect?shorten=".to_owned();
    origin.push_str(path);
    Redirect::to(origin)
}