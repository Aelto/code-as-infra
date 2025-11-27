use pingora::prelude::*;

mod filtering;
mod proxy;

// references:
// - https://github.com/koompi/pingora-proxy-server/blob/master/Cargo.toml
// - https://github.com/randommm/pingora-reverse-proxy/blob/master/src/service.rs

pub fn start_server() {
    let port_http = 80;
    let port_https = 443;

    let mut server = Server::new(None).expect("pingora server creation");
    server.bootstrap();

    let proxy_service_ssl = proxy::proxy_service_tls(
        &server.configuration,
        &format!("0.0.0.0:{port_https}"),
        vec![
            // proxy::HostConfigTls {
            //     proxy_internal_address: "127.0.0.1:4000".to_owned(),
            //     proxy_internal_tls: false,
            //     proxy_hostname: "somedomain.com".to_owned(),
            //     cert_path: format!("{}/keys/some_domain_cert.crt", env!("CARGO_MANIFEST_DIR")),
            //     key_path: format!("{}/keys/some_domain_key.pem", env!("CARGO_MANIFEST_DIR")),
            // },
            // proxy::HostConfigTls {
            //     proxy_internal_address: "one.one.one.one:443".to_owned(),
            //     proxy_internal_tls: true,
            //     proxy_hostname: "one.one.one.one".to_owned(),
            //     cert_path: format!("{}/keys/one_cert.crt", env!("CARGO_MANIFEST_DIR")),
            //     key_path: format!("{}/keys/one_key.pem", env!("CARGO_MANIFEST_DIR")),
            // },
        ],
    );

    let proxy_service_plain = proxy::proxy_service_plain(
        &server.configuration,
        "0.0.0.0:8082",
        vec![proxy::HostConfigPlain {
            proxy_internal_address: "127.0.0.1:3000".to_owned(),
            proxy_internal_tls: false,
            proxy_hostname: "modspot.dev".to_owned(),
        }],
    );

    let services: Vec<Box<dyn pingora::services::Service>> =
        vec![Box::new(proxy_service_ssl), Box::new(proxy_service_plain)];

    server.add_services(services);
    server.run_forever();
}
