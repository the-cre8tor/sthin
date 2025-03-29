use prometheus::{Encoder, Gauge, IntCounter, Registry, TextEncoder};

pub struct ApplicationMetrics {
    registry: Registry,
    url_creation_counter: IntCounter,
    url_access_counter: IntCounter,
    active_urls_gauge: Gauge,
}

impl ApplicationMetrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let url_creation_counter =
            IntCounter::new("url_creation_total", "Total number of URLs created")
                .expect("Failed to create metric");

        let url_access_counter =
            IntCounter::new("url_access_total", "Total number of URL accesses")
                .expect("Failed to create metric");

        let active_urls_gauge = Gauge::new("active_urls", "Number of active short URLs")
            .expect("Failed to create metric");

        registry
            .register(Box::new(url_creation_counter.clone()))
            .unwrap();
        registry
            .register(Box::new(url_access_counter.clone()))
            .unwrap();
        registry
            .register(Box::new(active_urls_gauge.clone()))
            .unwrap();

        Self {
            registry,
            url_creation_counter,
            url_access_counter,
            active_urls_gauge,
        }
    }

    pub fn increment_url_creation(&self) {
        self.url_creation_counter.inc();
    }

    pub fn increment_url_access(&self) {
        self.url_access_counter.inc();
    }

    pub fn set_active_urls(&self, count: f64) {
        self.active_urls_gauge.set(count);
    }

    pub fn export_metrics(&self) {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();

        encoder.encode(&metric_families, &mut buffer).unwrap();
        String::from_utf8(buffer).unwrap();
    }
}
