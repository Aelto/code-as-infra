// reference: https://gist.github.com/CodyPubNub/199eda6d491527ee7df6cd6a8ea5aab2

mod certificate;
pub use certificate::Certificate;
pub use certificate::prefer_h2;

mod certificate_cache;
pub use certificate_cache::CertificateCache;
