use tera::{Context, Tera};
use std::error::Error;
use lazy_static::lazy_static;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

// This will compile the template on build alongside the binary file.
lazy_static! {
    static ref TERA: Tera = {
        let template_string = include_str!("templates/index.html");
        let mut tera = Tera::default(); // Create a default Tera instance
        tera.add_raw_template("index.html", template_string).unwrap(); // Add the template string
        tera // Return the Tera instance
    };
}

fn template(name: &String, email: &String) -> Result<String, Box<dyn Error>> {

    // Using the tera Context struct
    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("email", &email);

    // Render
    Ok(TERA.render("index.html", &context)?)
}

pub fn send_email(
    // Sender Name
    name: &String,

    // Sender Email
    from_email: &String,

    // Sender Password
    password: &String,
    
    // SMTP Server Domain
    server: &String,
    
    // SMTP Port Number
    // port: u16,
    
    // Receiver Email
    to_email: &String,

) {

    match template(&name, &from_email) {
        Ok(res) => {            
            let from = format!("{name} <{from_email}>");
            let email = Message::builder()
            .from(from.parse().unwrap())
            // .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject("Happy new year")
            .header(ContentType::TEXT_HTML)
            .body(String::from(res))
            .unwrap();
        
        let creds = Credentials::new(from_email.to_owned(), password.to_owned());
        
        // Open a SMTP transport
        let mailer = SmtpTransport::relay(&server)
            .unwrap()
            .credentials(creds)
            .build();
        
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
