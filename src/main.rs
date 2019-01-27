#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate chrono;
extern crate menstruation;
extern crate serde_json;

use chrono::NaiveDate;
use menstruation::{codes, codes::Mensa, menu, menu::Meal, Response};
use rocket_contrib::json::Json;
use rocket::fairing::AdHoc;
use rocket::http::Header;

#[get("/menu/<mensa_code>/<date>")]
fn menu(mensa_code: u16, date: String) -> Option<Json<Response<Meal>>> {
    let naive_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d");
    menu::get(&mensa_code.into(), naive_date.ok()).map(Json).ok()
}

#[get("/codes")]
fn codes() -> Option<Json<Response<Mensa>>> {
    codes::get().map(|codes_response| Json(codes_response)).ok()
}

fn main() {
    rocket::ignite()
        .attach(AdHoc::on_response("CORS", |_, response| {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        }))
        .mount("/", routes![menu, codes])
        .launch();
}
