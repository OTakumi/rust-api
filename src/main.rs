use rocket::http::RawStr;
use rocket::serde::json::Json;
use rocket::tokio::time::{sleep, Duration};

use lib::db;
use lib::model::Movie;

// Todo: Deseria
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/movies")]
fn get_movies() -> Json<Option<Vec<Movie>>> {
    Json(db::read_movies())
}

// #[get("/movie/<title>")]
// fn get_movie(title: String) -> Json<Movie> {
//     let mut movie_info = MOVIES_DB::from(title);

//     Json(movie_info)
// }

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![get_movies])
        .mount("/", routes![delay])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let mut index_response = client.get(uri!(super::index)).dispatch();

        assert_eq!(index_response.status(), Status::Ok);
        assert_eq!(index_response.into_string().unwrap(), "Hello World");

        let mut lower_response = client.get(uri!(super::lower)).dispatch();
        assert_eq!(lower_response.status(), Status::Ok);
        assert_eq!(lower_response.into_string().unwrap(), "hello world");
    }
}
