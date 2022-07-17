use rocket::tokio::time::{sleep, Duration};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/lower")]
fn lower() -> &'static str {
    "hello world"
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![lower])
        .mount("/", routes![delay])
        .launch()
        .await?;

    Ok(())
}
