use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::client::{Tls, TlsParameters};

 
pub fn mail_sender(ext: i32, domain: String) {

    let email = Message::builder()
        .from("<some@example.com>".parse().unwrap())
        .to("<some@example.com>".parse().unwrap())
        .subject("SUBJECT EXAMPLE")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(format!("Body {} content {} example....", ext, domain)))
        .unwrap();

    let creds = Credentials::new("some@example.com".to_owned(), "P4ssw0rD".to_owned());

    let tls = TlsParameters::builder("smtp.example.com".to_owned())
        .dangerous_accept_invalid_certs(true)
        .build().unwrap();

    let mailer = SmtpTransport::starttls_relay("smtp.example.com")
        .unwrap()
        .credentials(creds)
        .tls(Tls::Required(tls))
        .build();

     let _ = mailer.send(&email); 
}

