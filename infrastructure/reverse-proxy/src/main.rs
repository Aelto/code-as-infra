use cai_reverse_proxy::*;

struct ModspotCdnService;
impl cai_reverse_proxy::WithServerService for ModspotCdnService {}
impl cai_reverse_proxy::WithReverseProxy for ModspotCdnService {
    fn register_https(&self, services: &mut Vec<proxy::HostConfigTls>) {
        services.push(proxy::HostConfigTls::new_localhost_service(
            5010,
            "cdn.modspot.dev",
            "/etc/letsencrypt/live/cdn.modspot.dev/fullchain.pem",
            "/etc/letsencrypt/live/cdn.modspot.dev/privkey.pem",
        ));
    }
}

struct PhotographyWebsiteService;
impl cai_reverse_proxy::WithServerService for PhotographyWebsiteService {}
impl cai_reverse_proxy::WithReverseProxy for PhotographyWebsiteService {
    fn register_https(&self, services: &mut Vec<proxy::HostConfigTls>) {
        services.push(proxy::HostConfigTls::new_localhost_service(
            5001,
            "t.hottou.fr",
            "/etc/letsencrypt/live/t.hottou.fr/fullchain.pem",
            "/etc/letsencrypt/live/t.hottou.fr/privkey.pem",
        ));
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
    let logger = logging::global_logger();
    let logger = logging::enable_service_logging(logger, &PhotographyWebsiteService);
    let logger = logging::enable_service_logging(logger, &ModspotCdnService);

    cai_reverse_proxy::service::<
        (
            proxy::context::ProxyCompression,
            proxy::context::RefererFilter<ModspotRefererFilter>,
            proxy::context::RateLimit<ModspotSignupLimiting>,
        ),
        logging::Logger,
    >(&mut server, &ModspotCdnService);

    cai_reverse_proxy::service::<proxy::context::ProxyCompression, logging::Logger>(
        &mut server,
        &PhotographyWebsiteService,
    );

    logger.start().expect("flexi_logger start error");
    server.run_forever();
}
