#[macro_use]
extern crate rocket;

use anyhow::Result;

mod test;
use test::StubQuery;

#[rocket::main]
async fn main() -> Result<()> {
    rocket::build().mount("/", routes![index]).ignite().await?;

    Ok(())
}

#[get("/?<query..>")]
fn index(query: StubQuery) -> String {
    format!("Page {:?}, PageSize {}", query.page, query.page_size)
}
