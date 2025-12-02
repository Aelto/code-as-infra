use http::HeaderValue;
use pingora::http::ResponseHeader;

use std::marker::PhantomData;

pub struct RefererFilter<Options: WithRefererFilterOptions> {
    options: PhantomData<Options>,
}

impl<Options: WithRefererFilterOptions> super::WithProxyContext for RefererFilter<Options> {
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
        if !Options::is_referer_allowed(session.get_header(http::header::REFERER)) {
            let header = ResponseHeader::build(403, None).unwrap();
            session.set_keepalive(None);
            session
                .write_response_header(Box::new(header), true)
                .await?;
            return Ok(true);
        }

        Ok(false)
    }
}

pub trait WithRefererFilterOptions: Send + Sync {
    /// returns `true` if the referer should be allowed.
    /// # Example
    /// ```rs
    /// fn is_referer_allowed(referer: Option<&HeaderValue>) -> bool {
    ///   path.starts_with("/login")
    /// }
    /// ```
    fn is_referer_allowed(referer: Option<&HeaderValue>) -> bool;
}
