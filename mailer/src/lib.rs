use std::{error::Error};
use lettre::{
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport, message::header::ContentType
};
use lazy_static::lazy_static;
use models::ConfirmUser;
static EMAIL_ADDRESS: &str = "szilubot@gmail.com";
lazy_static!{
    static ref CREDS: Credentials = Credentials::new("szilubot".to_string(), "ikovzntnzvmihrfy".to_string());
}

fn create_mailer() -> Result<SmtpTransport, Box<dyn Error>>{
    Ok(SmtpTransport::relay("smtp.gmail.com")?
        .credentials(CREDS.clone())
        .build())
}

pub fn send_confirmation_email(user: ConfirmUser) -> Result<(), Box<dyn Error>>{
    let mail = Message::builder()
        .from(format!("Szilumester <{EMAIL_ADDRESS}>").parse()?)
        .to(format!("{} <{}>",user.name, user.email).parse()?)
        .subject("Confirmation email at THEWEB")
        .header(ContentType::TEXT_HTML)
        .body(format!("<h1>Welcome {}!\n</h1>Your confirmation email: <a href='{}'>{}</a>", user.name, user.link, user.link))?;
    create_mailer()?.send(&mail)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mailer() {
        send_confirmation_email(
            ConfirmUser {
                name: "TESZTNÉM".to_string(),
                email: "channelszilu@gmail.com".to_string(),
                link: "https://sziluserver.duckdns.org/login".to_string()
            }
        ).unwrap()
    }
}
