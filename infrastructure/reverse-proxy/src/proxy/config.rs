#[derive(Clone)]
pub struct HostConfigTls {
    pub proxy_internal_address: String,
    pub proxy_internal_tls: bool,
    pub proxy_hostname: String,
    pub cert_path: String,
    pub key_path: String,
}
