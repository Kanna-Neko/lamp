use clap::Parser;
use lamp::{read_config,Cli};

#[tokio::main]
async fn main() {
    // read config from config file
    let cli = Cli::parse();
    println!("config_path is {}",cli.config);

    let config = read_config(&cli.config);
    println!("hello world, config is: {:?}",config);

    // start api server
    lamp::start(config).await;
}
