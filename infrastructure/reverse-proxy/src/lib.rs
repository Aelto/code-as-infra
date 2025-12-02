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

pub fn server() -> pingora::server::Server {
    let mut server = Server::new(None).expect("pingora server creation");
    server.bootstrap();
    server
}

pub fn service<CONTEXT>(server: &mut pingora::server::Server, service: &impl WithServerService)
where
    CONTEXT: proxy::context::WithProxyContext + 'static,
{
    let port_https = 443;
    let mut proxy_tls_configs = Vec::new();
    WithReverseProxy::register_https(service, &mut proxy_tls_configs);

    let proxy_service_ssl = proxy::proxy_service_tls::<CONTEXT>(
        &server.configuration,
        &format!("0.0.0.0:{port_https}"),
        proxy_tls_configs,
    );

    server.add_service(proxy_service_ssl);
}
