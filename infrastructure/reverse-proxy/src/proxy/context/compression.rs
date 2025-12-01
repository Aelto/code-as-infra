use pingora::Result;
use pingora::protocols::http::compression::ResponseCompressionCtx;

pub struct ProxyCompression {
    compressor: ResponseCompressionCtx,
    end_of_stream: bool,
}
impl super::WithProxyContext for ProxyCompression {
    fn new_ctx() -> Self {
        Self {
            compressor: ResponseCompressionCtx::new(7, true, true),
            end_of_stream: false,
        }
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
        if let Some(compressed_data) = _ctx
            .compressor
            .response_body_filter(_body.as_ref(), _end_of_stream)
        {
            *_body = Some(compressed_data);
        }

        if _end_of_stream {
            _ctx.end_of_stream = true;
        }

        Ok(None)
    }

    async fn response_filter(
        _session: &mut pingora::prelude::Session,
        _upstream_response: &mut pingora::http::ResponseHeader,
        _ctx: &mut Self,
    ) -> Result<()>
    where
        Self: Send + Sync,
    {
        _ctx.compressor
            .response_header_filter(_upstream_response, _ctx.end_of_stream);

        Ok(())
    }
}
