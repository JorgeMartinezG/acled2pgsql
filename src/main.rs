mod acled;
mod config;
//mod database;
mod schema;

use crate::acled::{Request, Response};
use crate::config::Config;
use clap::Parser;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use log::{debug, info};
use reqwest::blocking::Client;

use schema::acled::incidents;
use std::rc::Rc;

const CHUNK_SIZE: usize = 2000;

#[derive(Parser, Debug)]
#[command(name = "acled2pgsql")]
#[command(author = "Jorge Martinez <jorge.martinezgomez@wfp.org>")]
#[command(version = "1.0")]
#[command(about = "Ingest data into postgresql from acled api", long_about = None)]
struct Cli {
    #[arg(long)]
    config: String,
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let cli = Cli::parse();

    let config = Rc::new(Config::new(&cli.config));

    let client = Client::new();

    let mut conn = PgConnection::establish(&config.db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", config.db_url));

    config.country_codes.iter().for_each(|(iso, code)| {
        info!("Fetching data for country {:?}", iso);
        let mut page = 1;

        loop {
            let request = Request::new(config.clone(), page, *code);

            let resp: Response = client
                .get(&config.acled.api_url)
                .query(&request)
                .send()
                .expect("Failed to run request")
                .json()
                .expect("Failed parsing json response");

            if resp.count == 0 {
                break;
            }

            info!("iso = {} - page = {} - count = {}", iso, page, resp.count);

            resp.data.chunks(CHUNK_SIZE).for_each(|chunk| {
                debug!("Saving {} into database", chunk.len());

                diesel::insert_into(incidents::table)
                    .values(chunk)
                    .execute(&mut conn)
                    .expect("Error saving new post");
            });

            page += 1;
        }
    });
}
