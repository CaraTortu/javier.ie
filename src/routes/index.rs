use super::*;

#[get("/")]
pub async fn index() -> Template {
    Template::render("index", context! {})
}
