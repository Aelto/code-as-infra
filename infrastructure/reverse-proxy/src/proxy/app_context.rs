use crate::proxy::{HostConfigPlain, app::ProxyApp};

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

    pub fn host<'a, CONTEXT, EVENTS>(
        &self,
        app_context: &'a ProxyApp<CONTEXT, EVENTS>,
    ) -> Option<&'a HostConfigPlain>
    where
        CONTEXT: super::context::WithProxyContext + Send + Sync,
        EVENTS: super::events::WithProxyEvents + Send + Sync,
    {
        app_context.get_host_by_index(self.cached_host_index())
    }
}

pub type InternalContext = AppContextHostCache;

pub struct AppContext<CONTEXT> {
    pub internal: InternalContext,
    pub public: CONTEXT,
}

impl<CONTEXT> AppContext<CONTEXT> {
    pub fn new(public: CONTEXT) -> Self {
        Self {
            internal: AppContextHostCache::new(),
            public,
        }
    }

    pub fn hostname_cache(&mut self) -> &mut AppContextHostCache {
        &mut self.internal
    }
}
