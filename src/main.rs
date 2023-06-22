mod acled;
mod config;
//mod database;
mod schema;

use crate::acled::{Request, Response};
use crate::config::Config;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use reqwest::blocking::Client;

use schema::acled::incidents;
use std::rc::Rc;

fn main() {
    let config = Rc::new(Config::new("./config.toml"));

    println!("{:?}", config);

    let client = Client::new();

    let mut page = 1;
    let iso = 170;

    let database_url = "postgres://geonode:geonode@localhost:5433/geonode";

    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    loop {
        let request = Request::new(config.clone(), page, iso);

        let resp: Response = client
            .get(&config.acled.api_url)
            .query(&request)
            .send()
            .expect("Failed to run request")
            .json()
            .expect("Failed parsing json");

        if resp.count == 0 {
            break;
        }

        /*

        diesel::insert_into(incidents::table)
            .values(&resp.data[0])
            .execute(&mut conn)
            .expect("Error saving new post");
        */

        page += 1;
    }
}
