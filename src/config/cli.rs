use clap::Parser;

#[derive(Debug, Parser)]
#[command(version,about,long_about = None)]
pub struct Args {
    /// the country of holiday
    #[arg(short, long)]
    pub country: Option<String>,

    /// language
    #[arg(short, long)]
    pub language: Option<String>,

    #[arg(long)]
    pub column: Option<u32>,

    #[arg(long)]
    pub row: Option<u32>,

    #[arg( long, num_args(0..=1),default_missing_value = "true")]
    pub show_lunar: Option<bool>,

    #[arg( long, num_args(0..=1),default_missing_value = "true")]
    pub show_holiday: Option<bool>,

    #[arg(short, long, default_value = "%Y-%m-%d")]
    pub output: Option<String>,
}
