use super::{
    config_model::{Database, DotEnvyConfig, Server},
    stage::Stage,
};
use anyhow::Result;

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server: Server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is invalid")
            .parse()?,
        time_out: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
    };

    let database: Database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();
    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}
