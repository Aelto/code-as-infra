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

pub fn global_logger(services: impl WithServiceLogging) -> flexi_logger::Logger {
    let logger = flexi_logger::Logger::try_with_str("info")
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
        .append();

    services.enable_logging(logger)
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

pub trait WithServiceLogging {
    fn enable_logging(&self, logger: flexi_logger::Logger) -> flexi_logger::Logger;
}

impl WithServiceLogging for () {
    fn enable_logging(&self, logger: flexi_logger::Logger) -> flexi_logger::Logger {
        logger
    }
}

impl<S: crate::WithReverseProxy> WithServiceLogging for &S {
    fn enable_logging(&self, mut logger: flexi_logger::Logger) -> flexi_logger::Logger {
        let mut configs = Vec::new();
        S::register_https(&mut configs);

        for config in configs {
            let hostname = &config.proxy_hostname;

            logger = logger.add_writer(hostname, file_writer(hostname));
        }

        logger
    }
}

impl<S1: crate::WithReverseProxy, S2: crate::WithReverseProxy> WithServiceLogging for (&S1, &S2) {
    fn enable_logging(&self, mut logger: flexi_logger::Logger) -> flexi_logger::Logger {
        logger = self.0.enable_logging(logger);
        self.1.enable_logging(logger)
    }
}

impl<S1: crate::WithReverseProxy, S2: crate::WithReverseProxy, S3: crate::WithReverseProxy>
    WithServiceLogging for (&S1, &S2, &S3)
{
    fn enable_logging(&self, mut logger: flexi_logger::Logger) -> flexi_logger::Logger {
        logger = self.0.enable_logging(logger);
        logger = self.1.enable_logging(logger);
        self.2.enable_logging(logger)
    }
}

impl<
    S1: crate::WithReverseProxy,
    S2: crate::WithReverseProxy,
    S3: crate::WithReverseProxy,
    S4: crate::WithReverseProxy,
> WithServiceLogging for (&S1, &S2, &S3, &S4)
{
    fn enable_logging(&self, mut logger: flexi_logger::Logger) -> flexi_logger::Logger {
        logger = self.0.enable_logging(logger);
        logger = self.1.enable_logging(logger);
        logger = self.2.enable_logging(logger);
        self.3.enable_logging(logger)
    }
}

impl<
    S1: crate::WithReverseProxy,
    S2: crate::WithReverseProxy,
    S3: crate::WithReverseProxy,
    S4: crate::WithReverseProxy,
    S5: crate::WithReverseProxy,
> WithServiceLogging for (&S1, &S2, &S3, &S4, &S5)
{
    fn enable_logging(&self, mut logger: flexi_logger::Logger) -> flexi_logger::Logger {
        logger = self.0.enable_logging(logger);
        logger = self.1.enable_logging(logger);
        logger = self.2.enable_logging(logger);
        logger = self.3.enable_logging(logger);
        self.4.enable_logging(logger)
    }
}
