use pingora::{Result, http::ResponseHeader, proxy::Session};

mod compression;
pub use compression::ProxyCompression;

pub trait WithProxyContext: Send + Sync + Sized {
    fn new_ctx() -> Self;

    fn response_body_filter(
        _session: &mut Session,
        _body: &mut Option<bytes::Bytes>,
        _end_of_stream: bool,
        _ctx: &mut Self,
    ) -> Result<Option<std::time::Duration>>
    where
        Self: Send + Sync,
    {
        Ok(None)
    }

    fn response_filter(
        _session: &mut Session,
        _upstream_response: &mut ResponseHeader,
        _ctx: &mut Self,
    ) -> impl std::future::Future<Output = Result<()>> + Send
    where
        Self: Send + Sync,
    {
        async { Ok(()) }
    }
}

impl WithProxyContext for () {
    fn new_ctx() -> Self {
        ()
    }
}

impl<PC1, PC2> WithProxyContext for (PC1, PC2)
where
    PC1: WithProxyContext,
    PC2: WithProxyContext,
{
    fn new_ctx() -> Self {
        (PC1::new_ctx(), PC2::new_ctx())
    }

    fn response_body_filter(
        _session: &mut pingora::proxy::Session,
        _body: &mut Option<bytes::Bytes>,
        _end_of_stream: bool,
        _ctx: &mut Self,
    ) -> Result<Option<std::time::Duration>>
    where
        Self: Send + Sync,
    {
        let one = PC1::response_body_filter(_session, _body, _end_of_stream, &mut _ctx.0)?;
        let two = PC2::response_body_filter(_session, _body, _end_of_stream, &mut _ctx.1)?;

        Ok(one.or(two))
    }

    async fn response_filter(
        _session: &mut Session,
        _upstream_response: &mut ResponseHeader,
        _ctx: &mut Self,
    ) -> Result<()>
    where
        Self: Send + Sync,
    {
        PC1::response_filter(_session, _upstream_response, &mut _ctx.0).await?;
        PC2::response_filter(_session, _upstream_response, &mut _ctx.1).await?;

        Ok(())
    }
}
