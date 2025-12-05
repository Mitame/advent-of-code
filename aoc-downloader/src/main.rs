use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Utc};
use clap::Parser;
use reqwest::{Client, cookie::Jar};
use std::path::PathBuf;
use url::Url;

type Year = i32;
type Day = u32;
type Error = Box<dyn std::error::Error>;

fn build_download_url(year: Year, day: Day) -> Result<Url, Error> {
    Ok(Url::parse(&format!(
        "https://adventofcode.com/{year}/day/{day}/input"
    ))?)
}

async fn download_day(client: &Client, year: Year, day: Day) -> Result<Vec<u8>, Error> {
    let url = build_download_url(year, day)?;
    let body = client.get(url).send().await?.bytes().await?;
    Ok(body.to_vec())
}

fn build_client(session: &str) -> Result<Client, Error> {
    let cookie_jar = Jar::default();
    cookie_jar.add_cookie_str(
        &format!("session={session}"),
        &Url::parse("https://adventofcode.com")?,
    );
    Ok(reqwest::ClientBuilder::new()
        .cookie_provider(cookie_jar.into())
        .build()?)
}

fn all_puzzle_days(until_year: Year, until_day: Day) -> Vec<(Year, Day)> {
    let start_year = 2014;
    let start_short_year = 2025;

    let mut puzzle_days = vec![];

    for year in start_year..=until_year {
        let day_count = if year == until_year {
            until_day
        } else if year >= start_short_year {
            12
        } else {
            24
        };
        for day in 1..=day_count {
            puzzle_days.push((year, day));
        }
    }

    puzzle_days
}

#[derive(Parser)]
struct Args {
    /// Path to output data to
    output: PathBuf,

    /// The session key
    ///
    /// Grab from the browser
    #[arg(env)]
    session: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let client = build_client(&args.session)?;

    // Find out when it is, in UTC-5 as puzzles are released at midnight UTC-5
    let timezone = FixedOffset::west(60 * 60 * 5);
    let now = Utc::now().with_timezone(&timezone);

    eprintln!("{}", now);

    // Collect all puzzle days up to now
    let puzzle_days = all_puzzle_days(now.year(), now.day());

    for (year, day) in puzzle_days {
        // let data = download_day(&client, year, day).await?;
        let output_path = args
            .output
            .join(format!("y{year}"))
            .join(format!("day{day:02}"));
        if output_path.exists() {
            eprintln!("Data exists for {year}:{day:02}, skipping.");
            continue;
        }

        std::fs::create_dir_all(output_path.parent().unwrap())?;
        let data = download_day(&client, year, day).await?;
        std::fs::write(&output_path, &data)?;
        let bytes = data.len();
        eprintln!("Wrote {bytes} bytes to {output_path:?}");
    }

    Ok(())
}
