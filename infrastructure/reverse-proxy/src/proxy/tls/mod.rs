use std::sync::Arc;

use pingora::{
    listeners::{TlsAccept, tls::TlsSettings},
    proxy::http_proxy_service,
    server::configuration::ServerConf,
    tls::{
        self,
        ssl::{NameType, SslAlert, SslRef, SslVersion},
    },
};

use crate::{
    WithServerService,
    proxy::{HostConfigPlain, app::ProxyApp, events::WithProxyEvents},
};

mod ssl;

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
            .filter_map(
                |HostConfigTls {
                     proxy_hostname,
                     cert_path,
                     key_path,
                     proxy_internal_address: _,
                     proxy_internal_tls: _,
                 }| {
                    let Ok(cert_bytes) = std::fs::read(cert_path) else {
                        return None;
                    };

                    let Ok(cert) = tls::x509::X509::from_pem(&cert_bytes) else {
                        return None;
                    };

                    let Ok(key_bytes) = std::fs::read(key_path) else {
                        return None;
                    };

                    let Ok(key) = tls::pkey::PKey::private_key_from_pem(&key_bytes) else {
                        return None;
                    };

                    Some((proxy_hostname, cert, key))
                },
            )
            .collect();
        Self(config)
    }
}

#[async_trait::async_trait]
impl TlsAccept for TlsHandler {
    async fn certificate_callback(&self, ssl: &mut SslRef) -> () {
        let Some(sni_provided) = ssl.servername(NameType::HOST_NAME) else {
            return;
        };

        println!("ssl, sni={sni_provided:?}");

        let Some((_, cert, key)) = self.0.iter().find(|x| x.0 == sni_provided) else {
            println!("TlsAccept failure, no cert/key found for {sni_provided}");

            for s in &self.0 {
                println!(" - available sni: {}", &s.0);
            }

            return;
        };

        if let Err(e) = tls::ext::ssl_use_certificate(ssl, cert) {
            println!("error ssl_use_certificate: {e}");
            return;
        }

        // tls::ext::ssl_add_chain_cert(ssl, cert)

        if let Err(e) = tls::ext::ssl_add_chain_cert(ssl, cert) {
            println!("error ssl_use_certificate: {e}");
            return;
        }

        if let Err(e) = tls::ext::ssl_use_private_key(ssl, key) {
            println!("error ssl_use_private_key: {e}");
        }
    }
}

pub fn proxy_service_tls<'server, 'service, EVENTS, SERVICE>(
    server_conf: &'server Arc<ServerConf>,
    listen_addr: &str,
    host_configs: Vec<HostConfigTls>,
) -> impl pingora::services::Service + use<'service, EVENTS, SERVICE>
where
    'service: 'server,
    EVENTS: WithProxyEvents + 'static,
    SERVICE: WithServerService + 'static + Send + Sync,
{
    let plain_host_config = host_configs
        .iter()
        .map(|x| HostConfigPlain {
            proxy_internal_address: x.proxy_internal_address.clone(),
            proxy_internal_tls: x.proxy_internal_tls,
            proxy_hostname: x.proxy_hostname.clone(),
        })
        .collect();

    let proxy_app = ProxyApp::<EVENTS, SERVICE>::new(plain_host_config);

    let mut service = http_proxy_service(server_conf, proxy_app);

    // let cb = TlsHandler::new(host_configs.clone());
    // let cb = Box::new(cb);

    // ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384
    // ssl_protocols TLSv1.2 TLSv1.3
    // let mut tls_settings = TlsSettings::with_callbacks(cb).unwrap();

    let first_config = host_configs
        .first()
        .expect("at least one TLS config is needed");

    let mut tls_settings =
        TlsSettings::intermediate(&first_config.cert_path, &first_config.key_path).unwrap();

    let certificate_cache = ssl::CertificateCache::new(&host_configs);
    tls_settings.set_servername_callback(move |ssl_ref: &mut SslRef, _ssl_alert: &mut SslAlert| {
        certificate_cache.on_ssl_server_name_callback(ssl_ref)
    });

    // TlsSettings::intermediate(cert_path, key_path)

    // breaks everything
    // tls_settings.set_alpn(pingora::protocols::ALPN::H2H1);

    // tls_settings.enable_h2();
    // tls_settings.set_alpn_select_callback(ssl::prefer_h2);

    // tls_settings.set_cipher_list("ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384").unwrap();

    // tls_settings
    //     .set_min_proto_version(Some(SslVersion::TLS1_2))
    //     .unwrap();

    service.add_tls_with_settings(listen_addr, None, tls_settings);

    service
}
