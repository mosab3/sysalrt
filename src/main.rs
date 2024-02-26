use clap::Parser;
use std::thread;
use std::time::{SystemTime, Duration};
use sysalrt::email::send_email;
use sysalrt::usage::{memory_usage, cpu_usage};

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


    // #[arg(short, long, help = "Receiver Email")]
    // limit: 

}

#[test]
fn test() {
    use std::thread::sleep;

    let mut timer: SystemTime = SystemTime::now();

    loop {
        sleep(Duration::from_secs(5));
    
        let since: u64 = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs(),
            Err(e) => panic!("{e:?}")
        };
        
        println!("Since: {since}");

        if since >= 10 {
            timer = SystemTime::now();
        }


    }



}

fn main() {
    let args: Args = Args::parse();
    
    let cooldown: u64 = 3 * 60;

    let mut reading: Vec<u64> = Vec::new();
    let mut timer: SystemTime = SystemTime::now();

    loop {
        thread::sleep(Duration::from_secs(3));

        let monitor_obj: u64 = if args.cpu {
            cpu_usage()
        } else if args.memory {
            memory_usage()
        } else {
            panic!("Flag '--cpu' or '--memory' not provided");
        };

        reading.push(monitor_obj);

        let elapsed: u64 = match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs(),
            Err(e) => panic!("{e:?}")
        };

        #[cfg(debug_assertions)]
        println!("{:?}", reading);

        // if 
        if elapsed >= cooldown {
            let sum: u64 = reading.iter().sum();
            let average: f64 = sum as f64 / reading.len() as f64;

            #[cfg(debug_assertions)]
            println!("Average: {}\nSum: {}", average, sum);

            if average >= 50.0 {
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
