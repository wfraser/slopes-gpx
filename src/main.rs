use anyhow::{Context, Result};
use chrono::prelude::*;
use std::fs::File;
use std::io::BufRead;

mod gpx;

fn main() -> Result<()> {
    let path = std::env::args().nth(1).expect("need a file path");
    let file = File::open(path).context("failed to open file")?;
    let mut z = zip::ZipArchive::new(file).context("failed to read zip file")?;

    // TODO: Metadata.xml

    let gps_file = z.by_name("GPS.csv").context("failed to get GPS.csv from archive")?;
    let mut points = vec![];
    for line in std::io::BufReader::new(gps_file).lines() {
        let line = line.context("read error")?;
        let mut fields = line.split(',');

        macro_rules! parse {
            ($name:expr) => {
                fields.next()
                    .ok_or_else(|| anyhow::anyhow!(concat!("missing field ", $name)))?
                    .parse::<f64>()
                    .context(concat!("invalid ", $name))?
            }
        }

        let utc_seconds = parse!("timestamp");
        let lat = parse!("latitude");
        let lon = parse!("longitude");
        let ele = parse!("elevation");
        let course = parse!("course");
        let speed = parse!("speed");
        let _dunno1 = fields.next(); // horizontal accuracy in meters?
        let _dunno2 = fields.next(); // vertical accuracy in meters?
        
        points.push(gpx::Point {
            time: FixedOffset::west(0).timestamp(
                      utc_seconds.floor() as i64,
                      (utc_seconds.fract() / 1e-9) as u32),
            lat,
            lon,
            ele,
            course,
            speed,
        });
    }

    gpx::write_gpx(
        std::io::stdout(),
        &[&points[..]])?;

    Ok(())
}
