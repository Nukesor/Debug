#[derive(rocket::form::FromForm, Debug)]
pub struct StubQuery {
    pub page: Option<usize>,
    pub page_size: usize,
}
