use clap::Parser;

#[derive(Debug, Parser)]
#[command(version,about,long_about = None)]
pub struct Args {
    /// the region of holliday
    #[arg(short, long)]
    region: Option<String>,

    /// language
    #[arg(short, long)]
    language: Option<String>,
}
