use cai_reverse_proxy::{proxy::context::WithProxyContextCreation, *};

struct ProxyService;
impl cai_reverse_proxy::WithServerService for ProxyService {}
impl cai_reverse_proxy::WithReverseProxy for ProxyService {
    fn register_https(services: &mut Vec<proxy::HostConfigTls>) {
        services.push(proxy::HostConfigTls::new_localhost_service(
            5010,
            "cdn.modspot.dev",
            "/etc/letsencrypt/live/cdn.modspot.dev/fullchain.pem",
            "/etc/letsencrypt/live/cdn.modspot.dev/privkey.pem",
        ));

        services.push(proxy::HostConfigTls::new_localhost_service(
            5001,
            "t.hottou.fr",
            "/etc/letsencrypt/live/t.hottou.fr/fullchain.pem",
            "/etc/letsencrypt/live/t.hottou.fr/privkey.pem",
        ));
    }

    fn host_proxy(hostname: &str) -> Box<dyn proxy::context::WithProxyContext> {
        use proxy::context;

        match hostname {
            "t.hottou.fr" => Box::new(context::ProxyCompression::new_ctx()),
            "cdn.modspot.fr" => Box::new((
                context::ProxyCompression::new_ctx(),
                context::RefererFilter::<ModspotRefererFilter>::new_ctx(),
                context::RateLimit::<ModspotSignupLimiting>::new_ctx(),
            )),
            _ => Box::new(()),
        }
    }
}

struct ModspotSignupLimiting;
impl proxy::context::WithRateLimitOptions for ModspotSignupLimiting {
    const MAXIMUM_REQ_COUNT: isize = 3;

    fn is_host_limited(domain: &http::HeaderValue) -> bool {
        domain == "modspot.dev"
    }

    fn is_uri_path_limited(path: &str) -> bool {
        path.starts_with("/frg/SignupForm/invite/send")
    }
}

struct ModspotRefererFilter;
impl proxy::context::WithRefererFilterOptions for ModspotRefererFilter {
    fn is_referer_allowed(referer: Option<&http::HeaderValue>) -> bool {
        match referer {
            None => false,
            Some(value) => {
                let bytes = value.as_bytes();

                bytes.starts_with(b"https://cdn.modspot.dev")
                    || bytes.starts_with(b"https://api.modspot.dev")
                    || bytes.starts_with(b"https://modspot.dev")
            }
        }
    }
}

fn main() {
    let mut server = cai_reverse_proxy::server();

    use cai_reverse_proxy::proxy::events::logging;
    let logger = logging::global_logger(&ProxyService);

    cai_reverse_proxy::service::<logging::Logger, ProxyService>(&mut server);

    logger.start().expect("flexi_logger start error");
    server.run_forever();
}
