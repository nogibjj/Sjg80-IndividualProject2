use std::time::{Instant};
use psutil::memory::virtual_memory;
use psutil::cpu::CpuPercentCollector;

fn extract_words_from_email(email: &str) -> Vec<&str> {
    let mut words: Vec<&str> = vec![];

    if let Some(index) = email.find('@') {
        let username = &email[..index];
        words = username.split('.').collect();
    }

    words
}

fn main() {
    let email = "example.user@gmail.com"; // Replace with the email address you want to extract from

    let start_time = Instant::now();
    let words = extract_words_from_email(email);
    let end_time = start_time.elapsed();

    println!("Words before @: {:?}", words);
    println!("Execution Time: {:?}", end_time);

    // Measure resource usage
    let memory = virtual_memory().unwrap();
    let cpu_percent_collector = CpuPercentCollector::new().unwrap();

    // Clone the CpuPercentCollector struct before calling the println!() macro
    let mut cpu_percent_collector_clone = cpu_percent_collector.clone();
    println!("CPU Usage: {:.2}% (1-second interval)", cpu_percent_collector_clone.cpu_percent().unwrap());

    println!("Memory Usage: {} bytes", memory.total());
}
