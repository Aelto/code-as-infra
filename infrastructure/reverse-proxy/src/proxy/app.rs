use std::marker::PhantomData;

use async_trait::async_trait;
use http::Response;
use http::StatusCode;
use http::header;
use pingora::apps::http_app::ServeHttp;
use pingora::http::ResponseHeader;
use pingora::prelude::HttpPeer;
use pingora::prelude::ProxyHttp;
use pingora::prelude::Result;
use pingora::prelude::Session;
use pingora::protocols::http::ServerSession;

use crate::WithServerService;
use crate::proxy::HostConfigPlain;

pub struct ProxyApp<EVENTS, SERVICE>
where
    EVENTS: super::events::WithProxyEvents + Send + Sync,
    SERVICE: WithServerService + Send + Sync + 'static,
{
    host_configs: Vec<HostConfigPlain>,

    service_type: PhantomData<SERVICE>,
    events: EVENTS,
}

impl<EVENTS, SERVICE> ProxyApp<EVENTS, SERVICE>
where
    EVENTS: super::events::WithProxyEvents + Send + Sync,
    SERVICE: WithServerService + Send + Sync + 'static,
{
    pub fn new(host_configs: Vec<HostConfigPlain>) -> Self {
        ProxyApp {
            host_configs,
            service_type: PhantomData::default(),
            events: EVENTS::new(),
        }
    }

    pub fn get_host_by_index(&self, index: usize) -> Option<&HostConfigPlain> {
        self.host_configs.get(index)
    }
}

#[async_trait]
impl<EVENTS, SERVICE> ProxyHttp for ProxyApp<EVENTS, SERVICE>
where
    EVENTS: super::events::WithProxyEvents + Send + Sync,
    SERVICE: WithServerService + Send + Sync + 'static,
{
    type CTX = super::AppContext;

    fn new_ctx(&self) -> Self::CTX {
        super::AppContext::new(())
    }

    fn response_body_filter(
        &self,
        session: &mut Session,
        body: &mut Option<bytes::Bytes>,
        end_of_stream: bool,
        ctx: &mut Self::CTX,
    ) -> Result<Option<std::time::Duration>>
    where
        Self::CTX: Send + Sync,
    {
        ctx.public
            .response_body_filter(session, body, end_of_stream)
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        _upstream_response: &mut ResponseHeader,
        ctx: &mut Self::CTX,
    ) -> Result<()>
    where
        Self: Send + Sync,
    {
        ctx.public
            .response_filter(_session, _upstream_response)
            .await?;
        Ok(())
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let Some(host_config) = ctx.hostname_cache().host(&self) else {
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

    async fn request_filter(&self, session: &mut Session, ctx: &mut Self::CTX) -> Result<bool>
    where
        Self::CTX: Send + Sync,
    {
        // perform an initial HOSTNAME filtering
        let Some(host_header) = session
            .get_header(header::HOST)
            .and_then(|v| v.to_str().ok())
        else {
            return Ok(true);
        };

        let some_host_config = self
            .host_configs
            .iter()
            .position(|x| x.proxy_hostname == host_header);

        let Some(index) = some_host_config else {
            return Ok(true);
        };

        ctx.hostname_cache().cache_host(index);

        let context = SERVICE::host_proxy(host_header);
        ctx.public = context;

        ctx.public.request_filter(session).await
    }

    async fn logging(
        &self,
        _session: &mut Session,
        _e: Option<&pingora::Error>,
        _ctx: &mut Self::CTX,
    ) where
        Self::CTX: Send + Sync,
    {
        if let Some(host) = _ctx.hostname_cache().host(self) {
            self.events
                .logging(_session, _e, &host.proxy_hostname, &_ctx.internal);
        }
    }
}

#[allow(unused)]
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
