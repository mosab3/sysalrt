use clap::Parser;
use std::thread;
use std::time::{SystemTime, Duration};
use std::fmt::Write;
use std::cmp::min;
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
    
    // Sender SMTP Port Number, NOTE: still not implemented.
    #[arg(short = 'P', long, help = "Sender SMTP Port Number")]
    port: Option<u16>,
    
    // Receiver Email
    #[arg(short, long, help = "Receiver Email")]
    to_email: String,

    #[arg(short, long, action, help = "Monitor CPU usage, if cpu flag is present, memory flag will be ignored")]
    cpu: bool,

    #[arg(short, long, action, help = "Monitor memory usage")]
    memory: bool,

    #[arg(short = 'T', long, help = "The average threshold percentage (f64) of the resource usage")]
    threshold: f64

}

fn main() {
    let args: Args = Args::parse();
    
    let cooldown: u64 = 3 * 60;

    let mut reading: Vec<u64> = Vec::new();
    let mut timer: SystemTime = SystemTime::now();
    let refresh_status = Duration::from_secs(3);

    let total_size = 100;

    let monitor_type = if args.cpu {
        "CPU"
    } else {
        "MEMORY"
    };

    let ui_elements = UiElements::values();

    let template = format!("
    [1/4] OS: {:?}
    [2/4] USER: {:?}
    [3/4] SESSION ID: {:?}
    [3/4] MONITORING: {:?}

    {{spinner:.green}} [{{elapsed_precise}}] [{{wide_bar:.cyan/blue}}] {{percent}}%/100%\n
    ", ui_elements.os, ui_elements.user, ui_elements.session_id, monitor_type);

    let pb = ProgressBar::new(total_size);

    pb.set_style(ProgressStyle::with_template(&template)
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#|-"));
    
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
        thread::sleep(refresh_status);

        let elapsed: u64 = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs(),
            Err(e) => panic!("{e:?}")
        };    

        if elapsed >= cooldown {
            let sum: u64 = reading.iter().sum();
            let average: f64 = sum as f64 / reading.len() as f64;
    
            #[cfg(debug_assertions)]
            println!("Average: {}\nSum: {}", average, sum);
    
            if average >= args.threshold {
                send_email(
                    &args.name,
                    &args.from_email,
                    &args.password,
                    &args.server,
                    &args.to_email,
                )
            }

            timer = SystemTime::now();            
        }

    }
}
