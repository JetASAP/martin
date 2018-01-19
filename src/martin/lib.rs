extern crate iron_test;
extern crate iron;
#[macro_use] extern crate log;
extern crate logger;
extern crate persistent;
extern crate r2d2_postgres;
extern crate r2d2;
extern crate regex;
extern crate rererouter;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate tilejson;

use iron::prelude::Chain;
use logger::Logger;
use persistent::Read;
use rererouter::RouterBuilder;

mod cors;
mod db;
mod routes;
mod tileset;

pub fn chain(conn_string: String) -> iron::Chain {
    let mut router_builder = RouterBuilder::new();
    router_builder.get(r"/index.json", routes::index);
    router_builder.get(r"/(?P<tileset>[\w|\.]*)\.json", routes::tileset);
    router_builder.get(r"/(?P<tileset>[\w|\.]*)/(?P<z>\d*)/(?P<x>\d*)/(?P<y>\d*).pbf", routes::tile);
    let router = router_builder.finalize();

    let mut chain = Chain::new(router);
    
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);

    match db::setup_connection_pool(&conn_string, 10) {
        Ok(pool) => {
            info!("Connected to postgres: {}", conn_string);
            let conn = pool.get().unwrap();
            let tilesets = tileset::get_tilesets(conn).unwrap();
            chain.link(Read::<tileset::Tilesets>::both(tilesets));

            chain.link(Read::<db::DB>::both(pool));
        },
        Err(error) => {
            error!("Can't connect to postgres: {}", error);
            std::process::exit(-1);
        }
    };

    chain.link_after(cors::Middleware);
    chain.link_after(logger_after);

    chain
}