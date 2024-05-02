use dotenv::{dotenv, var};
use mail_send::{self, mail_builder::MessageBuilder, SmtpClientBuilder};

pub async fn send_email(from: &str, subject: &str, body: &str, ip: &str) -> mail_send::Result<()> {
    let env = dotenv().ok();
    env.expect("Failed to load .env file");

    let password = var("SMTP_PASS").expect("SMTP_PASS must be set");
    let email = var("SMTP_USER").expect("SMTP_EMAIL must be set");
    let host = var("SMTP_HOST").expect("SMTP_HOST must be set");
    let target = var("SMTP_TARGET").expect("TARGET_EMAIL must be set");

    let mut client = SmtpClientBuilder::new(host, 587)
        .implicit_tls(false)
        .credentials((email.to_owned(), password))
        .connect()
        .await
        .expect("Failed to connect to SMTP server");

    let message = MessageBuilder::new()
        .from(("JavierIE noreply".to_owned(), email.to_owned()))
        .to(target)
        .subject(format!("JAVIERIE contact form {from} - {subject}"))
        .text_body(format!("{body}\n\n\nINFO:\nIP Address: {ip}"));

    client.send(message).await?;

    Ok(())
}

pub async fn send_verification_email(to: &str, token: &str) -> mail_send::Result<()> {
    let env = dotenv().ok();
    env.expect("Failed to load .env file");

    let password = var("SMTP_PASS").expect("SMTP_PASS must be set");
    let email = var("SMTP_USER").expect("SMTP_EMAIL must be set");
    let host = var("SMTP_HOST").expect("SMTP_HOST must be set");
    let url = var("URL").expect("URL must be set");

    let mut client = SmtpClientBuilder::new(host, 587)
        .implicit_tls(false)
        .credentials((email.to_owned(), password))
        .connect()
        .await
        .expect("Failed to connect to SMTP server");

    let message = MessageBuilder::new()
        .from(("JavierIE noreply".to_owned(), email.to_owned()))
        .to(to)
        .subject("JAVIERIE Email Verification")
        .text_body(format!("Please verify your email address by clicking the link below:\n\n{url}/api/verify/{token}"));

    client.send(message).await?;

    Ok(())
}
