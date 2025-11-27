use cai_reverse_proxy::*;

struct ModspotCdnService;
impl cai_reverse_proxy::WithServerService for ModspotCdnService {}
impl cai_reverse_proxy::WithReverseProxy for ModspotCdnService {
    fn register_https(&self, services: &mut Vec<proxy::HostConfigTls>) {
        services.push(proxy::HostConfigTls::new_localhost_service(
            4000,
            "cdn.modspot.dev",
            "/etc/letsencrypt/live/cdn.modspot.dev.crt",
            "/etc/letsencrypt/live/cdn.modspot.dev.pem",
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
            "/etc/letsencrypt/live/t.hottou.fr.crt",
            "/etc/letsencrypt/live/t.hottou.fr.pem",
        ));
    }
}

fn main() {
    let services: Vec<&dyn WithServerService> =
        vec![&ModspotCdnService, &PhotographyWebsiteService];

    cai_reverse_proxy::start_server(services);
}
