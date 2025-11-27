use async_trait::async_trait;
use http::Response;
use http::StatusCode;
use http::header;
use pingora::apps::http_app::ServeHttp;
use pingora::prelude::HttpPeer;
use pingora::prelude::ProxyHttp;
use pingora::prelude::Result;
use pingora::prelude::Session;
use pingora::protocols::http::ServerSession;

use crate::proxy::HostConfigPlain;

pub struct ProxyApp {
    host_configs: Vec<HostConfigPlain>,
}

impl ProxyApp {
    pub fn new(host_configs: Vec<HostConfigPlain>) -> Self {
        ProxyApp { host_configs }
    }
}

#[async_trait]
impl ProxyHttp for ProxyApp {
    type CTX = ();
    fn new_ctx(&self) {}

    async fn upstream_peer(&self, session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let Some(host_header_value) = session.get_header(header::HOST) else {
            return Err(Box::new(pingora::Error {
                etype: pingora::ErrorType::HTTPStatus(http::StatusCode::BAD_REQUEST.as_u16()),
                esource: pingora::ErrorSource::Upstream,
                retry: pingora::RetryType::Decided(false),
                cause: None,
                context: Some(pingora::ImmutStr::Static("HOST header missing")),
            }));
        };

        let Ok(host_header) = host_header_value.to_str() else {
            return Err(Box::new(pingora::Error {
                etype: pingora::ErrorType::HTTPStatus(http::StatusCode::BAD_REQUEST.as_u16()),
                esource: pingora::ErrorSource::Upstream,
                retry: pingora::RetryType::Decided(false),
                cause: None,
                context: Some(pingora::ImmutStr::Static("HOST header invalid value")),
            }));
        };

        let some_host_config = self
            .host_configs
            .iter()
            .find(|x| x.proxy_hostname == host_header);

        let Some(host_config) = some_host_config else {
            return Err(Box::new(pingora::Error {
                etype: pingora::ErrorType::HTTPStatus(http::StatusCode::BAD_REQUEST.as_u16()),
                esource: pingora::ErrorSource::Upstream,
                retry: pingora::RetryType::Decided(false),
                cause: None,
                context: Some(pingora::ImmutStr::Static("HOST header invalid hostname")),
            }));
        };

        let proxy_to = HttpPeer::new(
            host_config.proxy_internal_address.as_str(),
            host_config.proxy_internal_tls,
            host_config.proxy_hostname.clone(),
        );

        let peer = Box::new(proxy_to);
        Ok(peer)
    }
}

pub struct RedirectApp;

#[async_trait]
impl ServeHttp for RedirectApp {
    async fn response(&self, http_stream: &mut ServerSession) -> Response<Vec<u8>> {
        let host_header = http_stream
            .get_header(header::HOST)
            .unwrap()
            .to_str()
            .unwrap();

        let body = "<html><body>301 Moved Permanently</body></html>"
            .as_bytes()
            .to_owned();

        Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header(header::CONTENT_TYPE, "text/html")
            .header(header::CONTENT_LENGTH, body.len())
            .header(header::LOCATION, format!("https://{host_header}"))
            .body(body)
            .unwrap()
    }
}
