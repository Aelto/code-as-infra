pub struct Logger {}

impl super::WithProxyEvents for Logger {
    fn new() -> Self {
        Self {}
    }

    fn logging(
        &self,
        session: &mut pingora::prelude::Session,
        _e: Option<&pingora::Error>,
        target: &str,
    ) {
        if let Some(ip) = session.client_addr().and_then(|addr| addr.as_inet()) {
            let level = log::Level::Info;
            let uri = &session.req_header().uri;

            log::log!(
              target: target,
              level,
              "[{}] - {}",
              ip,
              uri
            );
        }
    }
}

use flexi_logger::writers::FileLogWriter;

pub fn global_logger() -> flexi_logger::Logger {
    flexi_logger::Logger::try_with_str("info")
        .expect("flexi_logger creation error")
        .log_to_file(
            flexi_logger::FileSpec::default()
                .directory("logs")
                .basename("access.log"),
        )
        .create_symlink("current.access.log")
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepForDays(15),
        )
        // do not truncate the file when the program is restarted
        .append()
}

pub fn enable_service_logging<SERVICE: crate::WithReverseProxy>(
    mut logger: flexi_logger::Logger,
    service: &SERVICE,
) -> flexi_logger::Logger {
    let mut configs = Vec::new();
    service.register_https(&mut configs);

    for config in configs {
        let hostname = &config.proxy_hostname;

        logger = logger.add_writer(hostname, file_writer(hostname));
    }

    logger
}

fn file_writer(hostname: &str) -> Box<FileLogWriter> {
    Box::new(
        FileLogWriter::builder(
            flexi_logger::FileSpec::default()
                .directory("logs")
                .basename(hostname)
                .discriminant(hostname)
                .suffix("access.log"),
        )
        .append()
        .rotate(
            flexi_logger::Criterion::Age(flexi_logger::Age::Day),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepForDays(15),
        )
        .try_build()
        .unwrap(),
    )
}

pub trait WithLoggerOptions: Send + Sync {
    fn hostname() -> &'static str;

    fn file_writer() -> Box<FileLogWriter> {
        Box::new(
            FileLogWriter::builder(
                flexi_logger::FileSpec::default()
                    .directory("logs")
                    .basename(Self::hostname())
                    .discriminant(Self::hostname())
                    .suffix("access.log"),
            )
            .append()
            .rotate(
                flexi_logger::Criterion::Age(flexi_logger::Age::Day),
                flexi_logger::Naming::Timestamps,
                flexi_logger::Cleanup::KeepForDays(15),
            )
            .try_build()
            .unwrap(),
        )
    }
}
