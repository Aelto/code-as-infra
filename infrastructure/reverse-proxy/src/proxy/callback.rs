use pingora::{
    listeners::TlsAccept,
    tls::{self, ssl},
};

use crate::proxy::HostConfigTls;

pub struct Callback(Vec<(String, tls::x509::X509, tls::pkey::PKey<tls::pkey::Private>)>);

impl Callback {
    pub fn new(config: Vec<HostConfigTls>) -> Self {
        let config = config
            .into_iter()
            .map(
                |HostConfigTls {
                     proxy_hostname,
                     cert_path,
                     key_path,
                     proxy_internal_address: _,
                     proxy_internal_tls: _,
                 }| {
                    let cert_bytes = std::fs::read(cert_path).unwrap();
                    let cert = tls::x509::X509::from_pem(&cert_bytes).unwrap();

                    let key_bytes = std::fs::read(key_path).unwrap();
                    let key = tls::pkey::PKey::private_key_from_pem(&key_bytes).unwrap();

                    (proxy_hostname, cert, key)
                },
            )
            .collect();
        Self(config)
    }
}

#[async_trait::async_trait]
impl TlsAccept for Callback {
    async fn certificate_callback(&self, ssl: &mut ssl::SslRef) -> () {
        let sni_provided = ssl.servername(ssl::NameType::HOST_NAME).unwrap();
        dbg!(&sni_provided);
        let (_, cert, key) = self.0.iter().find(|x| x.0 == sni_provided).unwrap();
        tls::ext::ssl_use_certificate(ssl, cert).unwrap();
        tls::ext::ssl_use_private_key(ssl, key).unwrap();
    }
}
