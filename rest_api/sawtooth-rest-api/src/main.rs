use actix_web::HttpServer;
use actix_web::{get, post, web, App};
use structopt::clap::AppSettings::ColoredHelp;
use structopt::StructOpt;
use url::Url;

pub(crate) mod errors;
pub(crate) mod proto;
pub(crate) mod routes;

const MAX_SIZE_DEFAULT_STR: &str = "10485760";
const MAX_SIZE_DEFAULT: usize = 10485760;

#[derive(Clone, Debug, StructOpt, PartialEq, PartialOrd)]
#[structopt(
    about = "Starts the REST API application and connects to a specified validator.",
    rename_all = "kebab-case",
    setting(ColoredHelp)
)]
struct RestApiConfig {
    #[structopt(
        short = "B",
        long,
        help = "identify host and port for API to run on",
        default_value = "http://127.0.0.1:8008"
    )]
    bind: Url,

    #[structopt(
        short = "C",
        long,
        help = "specify URL to connect to a running validator",
        default_value = "tcp://localhost:4004"
    )]
    connect: Url,

    #[structopt(
        short = "t",
        long,
        help = "set time (in seconds) to wait for validator response",
        default_value = "300"
    )]
    timeout: f64,

    #[structopt(
        long,
        help = "the max size (in bytes) of a request body",
        default_value = MAX_SIZE_DEFAULT_STR
    )]
    client_max_size: usize,

    #[structopt(
        short = "v",
        long,
        help = "enable more verbose output to stderr",
        default_value,
        parse(from_occurrences)
    )]
    verbose: u8,

    #[structopt(
        long,
        help = "specify host and port for Open TSDB database used for metrics"
    )]
    opentsdb_url: Option<Url>,

    #[structopt(long, help = "specify name of database for storing metrics")]
    opentsdb_db: Option<String>,

    #[structopt(
        long,
        help = "specify username for Open TSDB database used for metrics"
    )]
    opentsdb_username: Option<String>,

    #[structopt(
        long,
        help = "specify password for Open TSDB database used for metrics"
    )]
    opentsdb_password: Option<String>,
}

fn main() -> anyhow::Result<()> {
    actix_web::rt::System::new("main").block_on(async move {
        let config: RestApiConfig = RestApiConfig::from_args();
        // config.bind.
        HttpServer::new(|| App::new())
            .bind(&*config.bind.socket_addrs(|| None)?)?
            .run()
            .await
            .map_err(Into::into)
    })
}
