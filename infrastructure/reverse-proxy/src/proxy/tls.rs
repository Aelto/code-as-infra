use pingora::{
    listeners::TlsAccept,
    tls::{self, ssl},
};

#[derive(Clone)]
pub struct HostConfigTls {
    pub proxy_internal_address: String,
    pub proxy_internal_tls: bool,
    pub proxy_hostname: String,
    pub cert_path: String,
    pub key_path: String,
}

impl HostConfigTls {
    pub fn new_localhost_service(
        internal_service_port: usize,
        hostname: &str,
        cert_path: &str,
        key_path: &str,
    ) -> Self {
        Self {
            proxy_internal_address: format!("127.0.0.1:{internal_service_port}"),
            proxy_hostname: hostname.to_owned(),
            cert_path: cert_path.to_owned(),
            key_path: key_path.to_owned(),

            proxy_internal_tls: false,
        }
    }
}

pub struct TlsHandler(Vec<(String, tls::x509::X509, tls::pkey::PKey<tls::pkey::Private>)>);

impl TlsHandler {
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
                    let cert_bytes = std::fs::read(cert_path).expect("cert_path reading error");
                    let cert =
                        tls::x509::X509::from_pem(&cert_bytes).expect("certificate from pem error");

                    let key_bytes = std::fs::read(key_path).expect("key_path reading error");
                    let key = tls::pkey::PKey::private_key_from_pem(&key_bytes)
                        .expect("key from pem error");

                    (proxy_hostname, cert, key)
                },
            )
            .collect();
        Self(config)
    }
}

#[async_trait::async_trait]
impl TlsAccept for TlsHandler {
    async fn certificate_callback(&self, ssl: &mut ssl::SslRef) -> () {
        let Some(sni_provided) = ssl.servername(ssl::NameType::HOST_NAME) else {
            return;
        };

        let Some((_, cert, key)) = self.0.iter().find(|x| x.0 == sni_provided) else {
            return;
        };

        if tls::ext::ssl_use_certificate(ssl, cert).is_ok() {
            if tls::ext::ssl_use_private_key(ssl, key).is_err() {
                println!("error ssl_use_private_key");
            }
        }
    }
}
