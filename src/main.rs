use clap::Parser;
use reqwest::Client;
use std::time::Instant;
use tokio::time::{sleep, Duration};
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

    println!("Mobitoring: {} ({} time(s))", args.url, args.count);

    let client = Client::new();
    let mut latencies: Vec<f64> = Vec::new();
    let mut errors = 0;

    for i in 1..=args.count {
        let start = Instant::now();
        match client.get(&args.url).send().await {
            Ok(resp) => {
                let duration = start.elapsed().as_millis();
                latencies.push(duration as f64);
            }
            Err(_) => {
                errors += 1;
                println!("Request {}: Failed", i);
            }
        }
        if i < args.count {
            sleep(Duration::from_secs(1)).await;
        }
    }

    if !latencies.is_empty() {
        let min = latencies.iter().fold(f64::INFINITY, |a, &b| f64::min(a, b));
        let max = latencies.iter().fold(0.0, |a, &b| f64::max(a, b));
        let avg: f64 = latencies.iter().sum::<f64>() / latencies.len() as f64;

        println!("\n--- Statistics ---");
        println!("Min: {:.2}ms, Max: {:.2}ms, Avg: {:.2}ms", min, max, avg);
        println!("Error Rate: {:.2}%", (errors as f64 / args.count as f64) * 100.0);
    }   else {
        println!("No successful requests were made.");
    }

}