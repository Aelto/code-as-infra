use http::HeaderValue;
use pingora::http::ResponseHeader;
use pingora_limits::rate::Rate;

use std::marker::PhantomData;
use std::sync::LazyLock;
use std::time::Duration;

static RATE_LIMITER: LazyLock<Rate> = LazyLock::new(|| Rate::new(Duration::from_mins(5)));

pub struct RateLimit<Options: WithRateLimitOptions> {
    options: PhantomData<Options>,
}

impl<Options: WithRateLimitOptions> super::WithProxyContext for RateLimit<Options> {
    fn new_ctx() -> Self {
        Self {
            options: PhantomData::default(),
        }
    }

    async fn request_filter(
        session: &mut pingora::prelude::Session,
        _ctx: &mut Self,
    ) -> pingora::Result<bool>
    where
        Self: Send + Sync,
    {
        if let Some(host) = session.get_header(http::header::HOST) {
            if !Options::is_host_limited(host) {
                return Ok(false);
            }
        }

        if !session.read_request().await.unwrap_or(false) {
            return Ok(false);
        }

        if !Options::is_uri_path_limited(session.req_header().uri.path()) {
            return Ok(false);
        }

        if let Some(ip) = session.client_addr().and_then(|addr| addr.as_inet()) {
            let requests_count = RATE_LIMITER.observe(ip, 1);

            if requests_count > Options::MAXIMUM_REQ_COUNT {
                // rate limited, return 429
                let mut header = ResponseHeader::build(429, None).unwrap();
                header
                    .insert_header("X-Rate-Limit-Limit", Options::MAXIMUM_REQ_COUNT)
                    .unwrap();
                header.insert_header("X-Rate-Limit-Remaining", "0").unwrap();
                header.insert_header("X-Rate-Limit-Reset", "1").unwrap();

                session.set_keepalive(None);
                session
                    .write_response_header(Box::new(header), true)
                    .await?;
                return Ok(true);
            }
        }

        Ok(false)
    }
}

pub trait WithRateLimitOptions: Send + Sync {
    /// defines how many requests can happen on a limited uri before they are
    /// rejected
    const MAXIMUM_REQ_COUNT: isize;

    /// returns `true` if the path should be rate limited.
    /// # Example
    /// ```rs
    /// fn is_uri_path_limited(path: &str) {
    ///   path.starts_with("/login")
    /// }
    /// ```
    fn is_uri_path_limited(path: &str) -> bool;

    /// returns `true` if the domain mentioned by the HOST header should be rate
    /// limited.
    /// # Example
    /// ```rs
    /// fn is_host_limited(domain: &HeaderValue) -> bool {
    ///   domain == "modspot.dev"
    /// }
    /// ```
    fn is_host_limited(domain: &HeaderValue) -> bool;
}
