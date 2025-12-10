pub struct AppContextHostnameCache {
    inner: String,
}

impl AppContextHostnameCache {
    pub fn new() -> Self {
        Self {
            inner: String::default(),
        }
    }
    pub fn cache_hostname(&mut self, hostname: String) {
        self.inner = hostname;
    }

    pub fn cached_hostname(&self) -> &str {
        &self.inner
    }
}

pub type InternalContext = AppContextHostnameCache;

pub struct AppContext<CONTEXT> {
    pub internal: InternalContext,
    pub public: CONTEXT,
}

impl<CONTEXT> AppContext<CONTEXT> {
    pub fn new(public: CONTEXT) -> Self {
        Self {
            internal: AppContextHostnameCache::new(),
            public,
        }
    }

    pub fn hostname_cache(&mut self) -> &mut AppContextHostnameCache {
        &mut self.internal
    }
}
