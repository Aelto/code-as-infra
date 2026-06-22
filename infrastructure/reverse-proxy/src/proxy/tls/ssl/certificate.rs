use pingora::tls::ssl::{AlpnError, SslContext, SslFiletype, SslMethod, SslRef, select_next_proto};

#[derive(Debug)]
pub struct Certificate {
    #[allow(unused)]
    key_path: String,

    #[allow(unused)]
    cert_path: String,

    hostname: String,
    ssl_context: SslContext,
}

impl Certificate {
    pub fn new(
        cert_path: &str,
        key_path: &str,
        hostname: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            ssl_context: Self::new_ssl_context(cert_path, key_path)?,

            hostname,

            cert_path: cert_path.to_owned(),
            key_path: key_path.to_owned(),
        })
    }

    fn new_ssl_context(
        cert_path: &str,
        key_path: &str,
    ) -> Result<SslContext, Box<dyn std::error::Error>> {
        let mut context = SslContext::builder(SslMethod::tls())?;

        context.set_certificate_chain_file(cert_path)?;
        context.set_private_key_file(key_path, SslFiletype::PEM)?;

        Ok(context.build())
    }

    pub fn matches_sni(&self, hostname: &str) -> bool {
        self.hostname == hostname
    }

    pub fn set_ssl_context(&self, ssl_ref: &mut SslRef) {
        if let Err(e) = ssl_ref.set_ssl_context(&self.ssl_context) {
            println!("error setting ssl context: {e:?}");
        }
    }
}
