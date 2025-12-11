use pingora::prelude::*;

mod filtering;
pub mod proxy;

use proxy::context::*;

pub trait WithServerService: WithReverseProxy {}

pub trait WithReverseProxy {
    fn register_http(_: &mut Vec<proxy::HostConfigPlain>) {}
    fn register_https(_: &mut Vec<proxy::HostConfigTls>) {}

    fn host_proxy(hostname: &str) -> Box<dyn WithProxyContext>;
}

pub fn server() -> pingora::server::Server {
    let mut server = Server::new(None).expect("pingora server creation");
    server.bootstrap();
    server
}

pub fn service<EVENTS, SERVICE>(server: &mut pingora::server::Server)
where
    EVENTS: proxy::events::WithProxyEvents + 'static,
    SERVICE: WithServerService + Send + Sync + 'static,
{
    let port_https = 443;
    let mut proxy_tls_configs = Vec::new();
    SERVICE::register_https(&mut proxy_tls_configs);

    let proxy_service_ssl = proxy::proxy_service_tls::<EVENTS, SERVICE>(
        &server.configuration,
        &format!("0.0.0.0:{port_https}"),
        proxy_tls_configs,
    );

    server.add_service(proxy_service_ssl);
}
