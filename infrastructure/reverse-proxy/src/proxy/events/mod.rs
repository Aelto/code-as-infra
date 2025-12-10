use pingora::proxy::Session;

pub mod logging;
pub use logging::Logger;

pub trait WithProxyEvents: Send + Sync + Sized {
    fn new() -> Self;

    fn logging(&self, _session: &mut Session, _e: Option<&pingora::Error>, _target: &str) {}
}

impl WithProxyEvents for () {
    fn new() -> Self {
        ()
    }
}

impl<PE1, PE2> WithProxyEvents for (PE1, PE2)
where
    PE1: WithProxyEvents,
    PE2: WithProxyEvents,
{
    fn new() -> Self {
        (PE1::new(), PE2::new())
    }

    fn logging(&self, _session: &mut Session, _e: Option<&pingora::Error>, target: &str) {
        self.0.logging(_session, _e, target);
        self.1.logging(_session, _e, target);
    }
}

impl<PE1, PE2, PE3> WithProxyEvents for (PE1, PE2, PE3)
where
    PE1: WithProxyEvents,
    PE2: WithProxyEvents,
    PE3: WithProxyEvents,
{
    fn new() -> Self {
        (PE1::new(), PE2::new(), PE3::new())
    }

    fn logging(&self, _session: &mut Session, _e: Option<&pingora::Error>, target: &str) {
        self.0.logging(_session, _e, target);
        self.1.logging(_session, _e, target);
        self.2.logging(_session, _e, target);
    }
}

impl<PE1, PE2, PE3, PE4> WithProxyEvents for (PE1, PE2, PE3, PE4)
where
    PE1: WithProxyEvents,
    PE2: WithProxyEvents,
    PE3: WithProxyEvents,
    PE4: WithProxyEvents,
{
    fn new() -> Self {
        (PE1::new(), PE2::new(), PE3::new(), PE4::new())
    }

    fn logging(&self, _session: &mut Session, _e: Option<&pingora::Error>, target: &str) {
        self.0.logging(_session, _e, target);
        self.1.logging(_session, _e, target);
        self.2.logging(_session, _e, target);
        self.3.logging(_session, _e, target);
    }
}
