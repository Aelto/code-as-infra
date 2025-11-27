use std::sync::Arc;

use pingora::proxy::http_proxy_service;
use pingora::server::configuration::ServerConf;

use super::ProxyApp;

#[derive(Clone)]
pub struct HostConfigPlain {
    pub proxy_internal_address: String,
    pub proxy_internal_tls: bool,
    pub proxy_hostname: String,
}

pub fn proxy_service_plain<'server, 'service>(
    server_conf: &'server Arc<ServerConf>,
    listen_addr: &str,
    host_configs: Vec<HostConfigPlain>,
) -> impl pingora::services::Service + use<'service>
where
    'service: 'server,
{
    let proxy_app = ProxyApp::new(host_configs.clone());
    let mut service = http_proxy_service(server_conf, proxy_app);

    service.add_tcp(listen_addr);

    service
}
