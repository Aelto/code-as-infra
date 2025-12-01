use pingora::listeners::tls::TlsSettings;
use pingora::proxy::http_proxy_service;
use pingora::server::configuration::ServerConf;
use std::sync::Arc;

mod tls;
pub use tls::HostConfigTls;
pub use tls::TlsHandler;

mod app;
use app::ProxyApp;

pub mod context;

mod plain;
pub use plain::HostConfigPlain;
pub use plain::proxy_service_plain;

use crate::proxy::context::WithProxyContext;

// TODO: move it to the tls module
pub fn proxy_service_tls<'server, 'service, CONTEXT>(
    server_conf: &'server Arc<ServerConf>,
    listen_addr: &str,
    host_configs: Vec<HostConfigTls>,
) -> impl pingora::services::Service + use<'service, CONTEXT>
where
    'service: 'server,
    CONTEXT: WithProxyContext + 'static,
{
    let plain_host_config = host_configs
        .iter()
        .map(|x| HostConfigPlain {
            proxy_internal_address: x.proxy_internal_address.clone(),
            proxy_internal_tls: x.proxy_internal_tls,
            proxy_hostname: x.proxy_hostname.clone(),
        })
        .collect();

    let proxy_app = ProxyApp::<CONTEXT>::new(plain_host_config);
    let mut service = http_proxy_service(server_conf, proxy_app);

    let cb = TlsHandler::new(host_configs);
    let cb = Box::new(cb);
    let tls_settings = TlsSettings::with_callbacks(cb).unwrap();
    service.add_tls_with_settings(listen_addr, None, tls_settings);

    service
}
