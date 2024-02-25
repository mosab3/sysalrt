use clap::Parser;
use std::thread;
use std::time::{SystemTime, Duration};
use sysalrt::sub::send_email;
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

// TODO:
//  make sure that the email is not send during a sudden spike:
//    -------------------------------------------------------------------------------------------------------
//    |  a- implement a cooldown checker that check if resource is still spiked after 5 mins.               |
//    |  b- the cooldown should check for the last spike, if still spiked -> send an email, else -> ignore. |
//    -------------------------------------------------------------------------------------------------------
//                                              OR
//    ------------------------------------------------------------------------------------------------------------------------
//    |  a- make a collection that has a list of the percentage (f32).                                                       |
//    |  b- implement a cooldown that check for the average percentage (if collection length is equal to a specific number). |
//    |  c- if collection cooldown length is true, check the average percentage from the collection.                         |
//    |  d- send email if average is equal or more to set resource limit.                                                    |
//    ------------------------------------------------------------------------------------------------------------------------


#[test]
fn test() {
    use std::thread::sleep;

    let mut timer = SystemTime::now();

    loop {
        sleep(Duration::from_secs(5));
    
        let since = match timer.elapsed() {
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
    let args = Args::parse();
    
    // #[derive(Debug)]
    // struct MyObject {
    //     timestamp: Result<Duration, Box<dyn Error>>,
    //     percentage: u64,
    // }

    // let cooldown = 3 * 60;

    let mut reading: Vec<u64> = Vec::new();
    let mut timer = SystemTime::now();

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
        if elapsed >= 3 * 60 {
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


// fn main() {
    
//     let args = Args::parse();
    
//     let mut object_list : Vec<MyObject> = Vec::new();

//     loop {
        
//         thread::sleep(Duration::from_secs(3));
        
//         let monitor_obj: u64 = if args.cpu {
//             cpu_usage()
//         } else if args.memory {
//             memory_usage()
//         } else {
//             panic!("Flag '--cpu' or '--memory' not provided");
//         };

//         let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| Box::new(e) as Box<dyn Error>);

//         object_list.push(MyObject { timestamp: timestamp, percentage: monitor_obj });

//         // let first_obj = object_list.first().unwrap();
//         // let now_time = SystemTime::now();

//         if let Some(first_obj) = object_list.first() {
//             let now_time = SystemTime::now();
//             println!("Object List are:\n{:?}", object_list);
        
//             if let Ok(elapsed) = now_time.duration_since(UNIX_EPOCH) {
//                 if elapsed >= first_obj.timestamp.unwrap_or(Duration::from_secs(0)) + Duration::from_secs(5 * 60) {
//                     let sum: u64 = object_list.iter().map(|obj| obj.percentage).sum();
//                     let average: f64 = sum as f64 / object_list.len() as f64;

//                     if average >= 60.0 {
//                         send_email(
//                             &args.name,
//                             &args.from_email,
//                             &args.password,
//                             &args.server,
//                             &args.to_email,
//                         );

//                         object_list.clear(); // Use clear() to empty the vector
//                     }
//                 }
//             }
//         }

        
        
//         // Check if the first element in the vector is 5 min ago.
//         // if now_time.duration_since(first_obj.timestamp).unwrap() >= Duration::from_secs(3 * 60) {
//         //     let sum: u64 = object_list.iter().map(|obj| obj.percentage).sum();
//         //     let average: f64 = sum as f64 / object_list.len() as f64;

//         //     if average >= 60.0 {

//         //         send_email(
//         //             &args.name,
//         //             &args.from_email,
//         //             &args.password,
//         //             &args.server,
//         //             &args.to_email
//         //         );

//         //         object_list = Vec::new();
//         //     }
//         // }
//     }

// }


