use pingora::prelude::*;

mod filtering;
pub mod proxy;

// references:
// - https://github.com/koompi/pingora-proxy-server/blob/master/Cargo.toml
// - https://github.com/randommm/pingora-reverse-proxy/blob/master/src/service.rs

pub trait WithServerService: WithReverseProxy {}

pub trait WithReverseProxy {
    fn register_http(&self, _: &mut Vec<proxy::HostConfigPlain>) {}
    fn register_https(&self, _: &mut Vec<proxy::HostConfigTls>) {}
}

pub fn start_server<CONTEXTTLS, CONTEXTPLAIN>(services: Vec<&dyn WithServerService>)
where
    CONTEXTTLS: proxy::context::WithProxyContext + 'static,
    CONTEXTPLAIN: proxy::context::WithProxyContext + 'static,
{
    let port_https = 443;

    let mut server = Server::new(None).expect("pingora server creation");
    server.bootstrap();

    let mut proxy_tls_configs = Vec::new();
    let mut proxy_plain_configs = Vec::new();

    for service in services {
        WithReverseProxy::register_http(service, &mut proxy_plain_configs);
        WithReverseProxy::register_https(service, &mut proxy_tls_configs);
    }

    let proxy_service_ssl = proxy::proxy_service_tls::<CONTEXTTLS>(
        &server.configuration,
        &format!("0.0.0.0:{port_https}"),
        proxy_tls_configs,
    );

    let proxy_service_plain = proxy::proxy_service_plain::<CONTEXTPLAIN>(
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
