mod cli;
mod db;
mod model;
mod utils;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};
use db::init_db;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let cli = Cli::parse();

    let pool = init_db().await?;

    match cli.command {
        Commands::AddChannel { name, path } => {
            commands::channel::add_channel(&pool, &name, &path).await?;
        }

        Commands::EnableChannel { id, enable } => {
            let enable_bool = match enable.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("--enable must be true or false"),
            };

            commands::channel::enable_channel(&pool, id, enable_bool).await?;
        }

        Commands::ListChannels => {
            commands::channel::list_channels(&pool).await?;
        }

        Commands::AddProgram { name, desc, path, start_time, end_time, channel_id } => {
            commands::program::add_program(&pool, &name, &desc, &path, &start_time, &end_time, channel_id).await?;
        }

        Commands::EnableProgram { id, enable } => {
            let enable_bool = match enable.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("--enable must be true or false"),
            };
            
            commands::program::enable_program(&pool, id, enable_bool).await?;
        }

        Commands::PlayProgram { id } => {
            commands::program::play_program(&pool, id).await?;
        }

        Commands::ListPrograms => {
            commands::program::list_programs(&pool).await?;
        }
    }

    Ok(())
}
