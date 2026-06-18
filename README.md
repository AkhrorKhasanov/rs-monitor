🚀 rs-monitor

rs-monitor is a lightweight and efficient CLI (Command Line Interface) tool written in Rust, designed to monitor the health (uptime) and response latency of websites and servers.

✨ Key Features

Real-time Monitoring: Perform multiple sequential requests to a target URL and monitor results in real-time.

Statistical Analysis: Automatically calculates Min, Max, and Avg (average) latency upon completion.

Error Tracking: Monitors server failures and 4xx/5xx HTTP errors, displaying them in red with a calculated Error Rate percentage.

HTTP Metadata: Analyzes and displays the HTTP status code and response size (in bytes) for every request.

Robustness: Configured with a 5-second connection timeout to prevent hanging.

🛠 Installation and Usage

Prerequisites

Rust must be installed on your machine.

Getting Started

Clone the repository:

Bash
git clone https://github.com/AkhrorKhasanov/rs-monitor.git

cd rs-monitor

Run the tool:

Bash
# Standard monitoring (default 5 requests)
cargo run -- --url https://google.com

# Monitoring with a custom request count
cargo run -- --url https://google.com --count 10

📊 Sample Output

When the tool runs, you will see a clean, color-coded output in your terminal:

Plaintext

Monitoring: https://conference.samdu.uz (5 time(s))

Request 1: 218ms | Status: 200 OK | Size: 138699 bytes

Request 2: 56ms  | Status: 200 OK | Size: 138699 bytes
...

--- Statistics ---

Min: 56.00ms, Max: 218.00ms, Avg: 89.40ms

Error Rate: 0.00%

💻 Tech Stack

Rust: The core programming language.

Reqwest: For sending asynchronous HTTP requests.

Clap: For parsing command-line arguments.

Tokio: The asynchronous runtime.

Colored: For terminal color output.

This project was created to learn Rust and strengthen skills in building robust CLI tools.
