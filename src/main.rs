// use color_eyre::eyre::{Ok, Result};
use color_eyre::eyre::Result;
use envconfig::Envconfig;
extern crate pretty_env_logger;
#[macro_use] extern crate log;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}


#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "NATS_URl")]
    pub nats_url: String,

    #[envconfig(from = "DB_PORT")]
    pub db_port: Option<u16>,

    #[envconfig(from = "HTTP_PORT", default = "8080")]
    pub http_port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    color_eyre::install()?;
    let config = Config::init_from_env().unwrap();
    let client = async_nats::connect("demo.nats.io").await?;
    let jetstream = async_nats::jetstream::new(client);



    Ok(())
}
