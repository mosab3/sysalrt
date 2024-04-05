use clap::Parser;
use std::io::Write;
use std::thread;
use std::time::{SystemTime, Duration};
use std::fmt::Write as fmtWrite;
use std::cmp::min;
use std::path::PathBuf;
use std::fs::OpenOptions;
use sysalrt::email::send_email;
use sysalrt::usage::{memory_usage, cpu_usage};
use sysalrt::ui::UiElements;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]


struct Args {
    
    // Receiver Name
    name: String,

    #[arg(short, long, help = "Sender Email")]
    from_email: String,

    // Sender Password
    #[arg(short, long, help = "Sender Password")]
    password: String,
    
    // Sender SMTP Server Domain
    #[arg(short, long, help = "Sender SMTP Server Domain")]
    server: String,
    
    // Sender SMTP Port Number
    #[arg(short = 'P', long, help = "Sender SMTP Port Number", default_value_t = 443)]
    port: u16,
    
    // Receiver Email
    #[arg(short, long, help = "Receiver Email")]
    to_email: String,

    // Monitor CPU
    #[arg(short, long, action, help = "Monitor CPU usage, if cpu flag is present, memory flag will be ignored")]
    cpu: bool,

    // Monitor Memory
    #[arg(short, long, action, help = "Monitor memory usage")]
    memory: bool,

    // Threshold percentage
    #[arg(short = 'T', long, help = "The average threshold percentage (f64) of the resource usage")]
    threshold: f64,

    // Append Session id into file
    #[arg(short, long, help = "Append the session id to an existing file")]
    append: Option<PathBuf>

}

fn main() {
    let args: Args = Args::parse();
    
    const COOLDOWN: u64 = 3 * 60;

    let mut reading: Vec<u64> = Vec::new();
    let mut timer: SystemTime = SystemTime::now();
    const REFRESH_READING: Duration = Duration::from_secs(2);
    const TOTAL_PERCENTAGE: u64 = 100;
    let monitor_type: &str = if args.cpu {
        "CPU"
    } else {
        "MEMORY"
    };

    let ui_elements: UiElements = UiElements::values();

    let template = format!("
    [1/4] OS: {:?}
    [2/4] USER: {:?}
    [3/4] SESSION ID: {:?}
    [4/4] MONITORING: {:?}

    [USAGE] [{{wide_bar:.cyan/blue}}] {{percent}}%/100% {{spinner:.green}} [{{elapsed_precise}}]\n
    ", ui_elements.os, ui_elements.user, ui_elements.session_id, monitor_type);

    let pb = ProgressBar::new(TOTAL_PERCENTAGE);

    pb.set_style(ProgressStyle::with_template(&template)
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn fmtWrite| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#|-"));
    
    // Check if path exist and is a file
    if let Some(ref append) = args.append {
        match append.exists() {
            true => match append.is_file() {
                false => panic!("Append path is not a file"),
                true => (),
            },
            false => panic!("Append path does not exist"),
        }
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open(args.append.unwrap())
        .expect("Error opening append file");

    let data = ui_elements.session_id.to_string() + "\n";

    file.write(&data.as_bytes()).expect("Error writing session id to file");

    loop {
        let monitor_obj: u64 = if args.cpu {
            cpu_usage()
        } else if args.memory {
            memory_usage()
        } else {
            panic!("Flag '--cpu' or '--memory' not provided");
        };
    
        reading.push(monitor_obj);
        
        pb.set_position(min(monitor_obj, 100));
        thread::sleep(REFRESH_READING);

        let elapsed: u64 = timer.elapsed().unwrap().as_secs(); 

        if elapsed >= COOLDOWN && reading.len() as u64 >= 10 {
            let sum: u64 = reading.iter().sum();
            let average: f64 = sum as f64 / reading.len() as f64;
    
            if average >= args.threshold {
                #[cfg(debug_assertions)]
                println!("Average: {}\nSum: {}", average, sum);
        
                send_email(
                    &args.name,
                    &args.from_email,
                    &args.password,
                    &args.server,
                    args.port,
                    &args.to_email,
                    ui_elements.session_id,
                    monitor_type
                );

                timer = SystemTime::now();
                reading.clear()
            } else {
                // Reset Timer and reading vector
                timer = SystemTime::now();
                reading.clear()
                
            }
        }
    }
}
