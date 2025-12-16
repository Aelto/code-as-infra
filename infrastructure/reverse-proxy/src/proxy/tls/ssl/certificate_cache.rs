use pingora::tls::ssl::{NameType, SniError, SslAlert, SslRef};

use crate::proxy::HostConfigTls;

use super::Certificate;

pub struct CertificateCache {
    certificates: Vec<Certificate>,
}

impl CertificateCache {
    pub fn new(host_configs: &Vec<HostConfigTls>) -> Self {
        Self {
            certificates: host_configs
                .iter()
                .map(|config| {
                    Certificate::new(
                        &config.cert_path,
                        &config.key_path,
                        config.proxy_hostname.clone(),
                    )
                    .expect("failed to load certificate")
                })
                .collect(),
        }
    }

    pub fn on_ssl_server_name_callback(&self, ssl_ref: &mut SslRef) -> Result<(), SniError> {
        let server_name = ssl_ref.servername(NameType::HOST_NAME);

        println!("server_name sni = {server_name:?}");

        if let Some(sni) = server_name {
            let certificate = self.certificates.iter().find(|cert| cert.matches_sni(sni));
            dbg!(&certificate);

            if let Some(certificate) = certificate {
                certificate.set_ssl_context(ssl_ref);
            }
        }

        Ok(())
    }
}
