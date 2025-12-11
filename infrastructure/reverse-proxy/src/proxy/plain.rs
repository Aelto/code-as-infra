use std::sync::Arc;

use pingora::proxy::http_proxy_service;
use pingora::server::configuration::ServerConf;

use crate::WithServerService;
use crate::proxy::events::WithProxyEvents;

use super::ProxyApp;

#[derive(Clone)]
pub struct HostConfigPlain {
    pub proxy_internal_address: String,
    pub proxy_internal_tls: bool,
    pub proxy_hostname: String,
}

impl HostConfigPlain {
    pub fn new_localhost_service(internal_service_port: usize, hostname: &str) -> Self {
        Self {
            proxy_internal_address: format!("127.0.0.1:{internal_service_port}"),
            proxy_hostname: hostname.to_owned(),

            proxy_internal_tls: false,
        }
    }
}

pub fn proxy_service_plain<'server, 'service, EVENTS, SERVICE>(
    server_conf: &'server Arc<ServerConf>,
    listen_addr: &str,
    host_configs: Vec<HostConfigPlain>,
) -> impl pingora::services::Service + use<'service, EVENTS, SERVICE>
where
    'service: 'server,
    EVENTS: WithProxyEvents + 'static,
    SERVICE: WithServerService + Send + Sync + 'static,
{
    let proxy_app = ProxyApp::<EVENTS, SERVICE>::new(host_configs.clone());
    let mut service = http_proxy_service(server_conf, proxy_app);

    service.add_tcp(listen_addr);

    service
}
