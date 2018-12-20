#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate menstruation;
extern crate serde_json;

use chrono::{Local, NaiveDate};
use menstruation::get_menu;

#[get("/<mensa_code>")]
fn menu(mensa_code: u16) -> String {
    let today = Local::today().naive_local().format("%Y-%m-%d").to_string();
    menu_date(mensa_code, today)
}

#[get("/<mensa_code>/<date>")]
fn menu_date(mensa_code: u16, date: String) -> String {
    let naive_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d");
    match get_menu(&mensa_code.into(), &naive_date.ok()) {
        Ok(menu_response) => format!("{}", serde_json::to_string_pretty(&menu_response).unwrap()),
        Err(e) => e,
    }
}

fn main() {
    rocket::ignite()
        .mount("/menu", routes![menu, menu_date])
        .launch();
}
