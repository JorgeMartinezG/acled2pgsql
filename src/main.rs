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

    let client = Client::new();

    let database_url = "postgres://gis:gis@localhost:5433/wfp";

    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    config.country_codes.iter().for_each(|(iso, code)| {
        println!("Fetching data for country {:?}", iso);
        let mut page = 1;

        loop {
            let request = Request::new(config.clone(), page, *code);

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

            println!("iso = {} - page = {} - count = {}", iso, page, resp.count);

            resp.data.chunks(2000).for_each(|chunk| {
                println!("Saving {} into database", chunk.len());

                diesel::insert_into(incidents::table)
                    .values(chunk)
                    .execute(&mut conn)
                    .expect("Error saving new post");
            });

            page += 1;
        }
    });
}
