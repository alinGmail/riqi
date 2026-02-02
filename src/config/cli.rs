use clap::Parser;

#[derive(Debug, Parser)]
#[command(version,about,long_about = None)]
pub struct Args {
    /// the country of holliday
    #[arg(short, long)]
    pub country: Option<String>,

    /// language
    #[arg(short, long)]
    pub language: Option<String>,

    #[arg( long)]
    pub column: Option<u32>,
    
    #[arg( long)]
    pub row: Option<u32>,
}
