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

fn main() {
    let services: Vec<&dyn WithServerService> =
        vec![&ModspotCdnService, &PhotographyWebsiteService];

    type ContextHttps = (
        proxy::context::ProxyCompression,
        proxy::context::RateLimit<ModspotSignupLimiting>,
    );
    type ContextHttp = ();

    cai_reverse_proxy::start_server::<ContextHttps, ContextHttp>(services);
}
