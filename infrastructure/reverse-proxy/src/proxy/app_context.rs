use crate::{
    WithServerService,
    proxy::{HostConfigPlain, app::ProxyApp, context::WithProxyContext},
};

pub struct AppContextHostCache {
    index: usize,
}

impl AppContextHostCache {
    pub fn new() -> Self {
        Self { index: 0 }
    }
    pub fn cache_host(&mut self, host_index: usize) {
        self.index = host_index;
    }

    pub fn cached_host_index(&self) -> usize {
        self.index
    }

    pub fn host<'a, EVENTS, SERVICE>(
        &self,
        app_context: &'a ProxyApp<EVENTS, SERVICE>,
    ) -> Option<&'a HostConfigPlain>
    where
        EVENTS: super::events::WithProxyEvents + Send + Sync,
        SERVICE: WithServerService + Send + Sync + 'static,
    {
        app_context.get_host_by_index(self.cached_host_index())
    }
}

pub type InternalContext = AppContextHostCache;

pub struct AppContext {
    pub internal: InternalContext,
    pub public: Box<dyn WithProxyContext>,
}

impl AppContext {
    pub fn new(public: impl WithProxyContext + 'static) -> Self {
        Self {
            internal: AppContextHostCache::new(),
            public: Box::new(public),
        }
    }

    pub fn hostname_cache(&mut self) -> &mut AppContextHostCache {
        &mut self.internal
    }
}
