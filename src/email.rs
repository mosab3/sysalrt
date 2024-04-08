use tera::{Context, Tera};
use std::error::Error;
use lazy_static::lazy_static;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::client::{Tls, TlsParameters};

// This will compile the template on build alongside the binary file.
lazy_static! {
    static ref TERA: Tera = {
        let template_string = include_str!("templates/index.html");
        let mut tera = Tera::default(); // Create a default Tera instance
        tera.add_raw_template("index.html", template_string).unwrap(); // Add the template string
        tera // Return the Tera instance
    };
}

fn template(session_id: u64, monitor_type: &str) -> Result<String, Box<dyn Error>> {

    // Using the tera Context struct
    let mut context = Context::new();
    context.insert("session_id", &session_id);
    context.insert("monitor_type", &monitor_type);

    // Render
    Ok(TERA.render("index.html", &context)?)
}

pub fn send_email(
    name: &String,

    from_email: &String,

    password: &String,
    
    server: &String,
    
    port: u16,
    
    to_email: &String,

    session_id: u64,

    monitor_type: &str,

    use_tls: bool
) {

    match template(session_id, monitor_type) {
        Ok(res) => {            
            let from = format!("{name} <{from_email}>");
            let email = Message::builder()
            .from(from.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject("URGENT: System Resource Alert")
            .header(ContentType::TEXT_HTML)
            .body(String::from(res))
            .unwrap();
        
        let creds = Credentials::new(from_email.to_owned(), password.to_owned());

        // Open a SMTP transport
        let mailer: SmtpTransport = if use_tls {

            let tls_params = TlsParameters::builder(server.to_string())
                .build()
                .unwrap();
            SmtpTransport::relay(&server)
            .unwrap()
            .credentials(creds)
            .port(port)
            .tls(Tls::Required(tls_params))
            .build()
        } else {

            SmtpTransport::relay(&server)
            .unwrap()
            .credentials(creds)
            .port(port)
            .build()
        };
        
        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("An email was send!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
        },
        Err(e) => {
            panic!("{}", e);
        }
    }

}
