use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tinysch")]
#[command(about = "Tiny TV CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    AddChannel {
        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        path: String,
    },

    EnableChannel {
        #[arg(short, long)]
        id: i64,

        #[arg(short, long)]
        enable: String,
    },

    ScheduleChannel {
        #[arg(short, long)]
        id: i64,
    },

    ListChannels,

    AddProgram {
        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        desc: String,

        #[arg(short, long)]
        path: String,

        #[arg(short, long)]
        start_time: String,

        #[arg(short, long)]
        end_time: String,

        #[arg(short, long)]
        channel_id: i64,
    },

    EnableProgram {
        #[arg(short, long)]
        id: i64,

        #[arg(short, long)]
        enable: String,
    },

    PlayProgram {
        #[arg(short, long)]
        id: i64,
    },

    ListPrograms,
}
