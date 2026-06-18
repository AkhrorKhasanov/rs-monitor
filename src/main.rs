use clap::Parser;
use colored::*;
use reqwest::ClientBuilder;
use std::time::Instant;
use tokio::time::{Duration, sleep};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
    #[arg(short, long, default_value_t = 5)]
    count: u32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = ClientBuilder::new()
        .connect_timeout(Duration::from_secs(5))
        .build()
        .expect("Client creation failed");
    println!("Monitoring: {} ({} time(s))", args.url, args.count);

    let mut latencies: Vec<f64> = Vec::new();
    let mut errors = 0;

    for i in 1..=args.count {
        let start = Instant::now();
        match client.get(&args.url).send().await {
            Ok(resp) => {
                let duration = start.elapsed().as_millis();
                let status = resp.status();

                let body_bytes = resp.bytes().await.unwrap_or_default();
                let len = body_bytes.len();

                let status_colored = if status.is_success() {
                    status.to_string().green()
                } else {
                    status.to_string().red()
                };

                latencies.push(duration as f64);
                println!(
                    "Request {}: {}ms | Status: {} | Size: {} bytes",
                    i, duration, status_colored, len
                );
            }
            Err(e) => {
                errors += 1;
                println!("Request {}: {} ({})", i, "Failed".red().bold(), e);
            }
        }
        if i < args.count {
            sleep(Duration::from_secs(1)).await;
        }
    }

    if !latencies.is_empty() {
        let min = latencies.iter().fold(f64::INFINITY, |a, &b| f64::min(a, b));
        let max = latencies.iter().fold(f64::NEG_INFINITY, |a, &b| f64::max(a, b));
        let avg: f64 = latencies.iter().sum::<f64>() / latencies.len() as f64;

        println!("\n--- Statistics ---");
        println!("Min: {:.2}ms, Max: {:.2}ms, Avg: {:.2}ms", min, max, avg);
        println!(
            "Error Rate: {:.2}%",
            (errors as f64 / args.count as f64) * 100.0
        );
    } else {
        println!("No successful requests were made.");
    }
}
