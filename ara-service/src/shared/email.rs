
use crate::shared::config::APP_CONFIG;
use failure::Error;
use failure::ResultExt;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::{SmtpTransport, SmtpTransportBuilder};
use lettre::{ClientSecurity, ClientTlsParameters};
use native_tls::TlsConnector;
use std::time::Duration;

pub(crate) fn smtp_server() -> Result<SmtpTransport, Error> {
    let smtp_server = &APP_CONFIG.smtp.host;
    let smtp_port: u16 = APP_CONFIG.smtp.port;
    let smtp_use_tls: bool = APP_CONFIG.smtp.use_tls;
    let smtp_username = &APP_CONFIG.smtp.username;
    let smtp_password = &APP_CONFIG.smtp.password;

    let client_security = if smtp_use_tls {
        let connector = TlsConnector::builder()?
            .build()
            .context("Error building TLS connector")?;

        let tls_parameters = ClientTlsParameters::new(smtp_server.to_string(), connector);
        ClientSecurity::Required(tls_parameters)
    } else {
        ClientSecurity::None
    };
    let mailer = SmtpTransportBuilder::new((smtp_server.as_ref(), smtp_port), client_security)
        .context("Unable to build SmtpTransport")?
        .hello_name(ClientId::Domain("mmrath.com".to_string()))
        .authentication_mechanism(Mechanism::Login)
        .credentials(Credentials::new(
            smtp_username.to_string(),
            smtp_password.to_string(),
        ))
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        .timeout(Some(Duration::from_secs(5)))
        .build();

    Ok(mailer)
}
