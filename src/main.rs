#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate menstruation;
#[macro_use]
extern crate rocket;
extern crate serde_json;

use chrono::NaiveDate;
use menstruation::{codes, menu};

#[get("/menu/<mensa_code>/<date>")]
fn menu(mensa_code: u16, date: String) -> String {
    let naive_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d");
    match menu::get(&mensa_code.into(), naive_date.ok()) {
        Ok(menu_response) => format!("{}", serde_json::to_string_pretty(&menu_response).unwrap()),
        Err(e) => e,
    }
}

#[get("/codes")]
fn codes() -> String {
    match codes::get() {
        Ok(codes_response) => format!("{}", serde_json::to_string_pretty(&codes_response).unwrap()),
        Err(e) => e,
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![menu, codes])
        .launch();
}
