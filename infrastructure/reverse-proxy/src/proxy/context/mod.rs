use pingora::{Result, http::ResponseHeader, proxy::Session};

mod compression;
pub use compression::ProxyCompression;

mod rate_limit;
pub use rate_limit::RateLimit;
pub use rate_limit::WithRateLimitOptions;

mod referer_filter;
pub use referer_filter::RefererFilter;
pub use referer_filter::WithRefererFilterOptions;

pub trait WithProxyContextCreation {
    fn new_ctx() -> Self;
}

#[async_trait::async_trait]
pub trait WithProxyContext: Send + Sync {
    fn response_body_filter(
        &mut self,
        _session: &mut Session,
        _body: &mut Option<bytes::Bytes>,
        _end_of_stream: bool,
    ) -> Result<Option<std::time::Duration>>
    where
        Self: Send + Sync,
    {
        Ok(None)
    }

    async fn response_filter(
        &mut self,
        _session: &mut Session,
        _upstream_response: &mut ResponseHeader,
    ) -> Result<()> {
        Ok(())
    }

    async fn request_filter(&mut self, _session: &mut Session) -> Result<bool> {
        Ok(false)
    }
}

impl WithProxyContext for () {}

#[async_trait::async_trait]
impl<PC1, PC2> WithProxyContext for (PC1, PC2)
where
    PC1: WithProxyContext,
    PC2: WithProxyContext,
{
    fn response_body_filter(
        &mut self,
        _session: &mut pingora::proxy::Session,
        _body: &mut Option<bytes::Bytes>,
        _end_of_stream: bool,
    ) -> Result<Option<std::time::Duration>>
    where
        Self: Send + Sync,
    {
        let one = self
            .0
            .response_body_filter(_session, _body, _end_of_stream)?;
        let two = self
            .1
            .response_body_filter(_session, _body, _end_of_stream)?;

        Ok(one.or(two))
    }

    async fn response_filter(
        &mut self,
        _session: &mut Session,
        _upstream_response: &mut ResponseHeader,
    ) -> Result<()> {
        self.0.response_filter(_session, _upstream_response).await?;
        self.1.response_filter(_session, _upstream_response).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<PC1, PC2, PC3> WithProxyContext for (PC1, PC2, PC3)
where
    PC1: WithProxyContext,
    PC2: WithProxyContext,
    PC3: WithProxyContext,
{
    fn response_body_filter(
        &mut self,
        _session: &mut pingora::proxy::Session,
        _body: &mut Option<bytes::Bytes>,
        _end_of_stream: bool,
    ) -> Result<Option<std::time::Duration>>
    where
        Self: Send + Sync,
    {
        let one = self
            .0
            .response_body_filter(_session, _body, _end_of_stream)?;
        let two = self
            .1
            .response_body_filter(_session, _body, _end_of_stream)?;
        let three = self
            .2
            .response_body_filter(_session, _body, _end_of_stream)?;

        Ok(one.or(two).or(three))
    }

    async fn response_filter(
        &mut self,
        _session: &mut Session,
        _upstream_response: &mut ResponseHeader,
    ) -> Result<()> {
        self.0.response_filter(_session, _upstream_response).await?;
        self.1.response_filter(_session, _upstream_response).await?;
        self.2.response_filter(_session, _upstream_response).await?;

        Ok(())
    }
}
