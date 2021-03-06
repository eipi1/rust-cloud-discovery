//! Intended to be rust counterpart of Spring Cloud Discovery client.
//!
//! ### Implementations
//! * Kubernetes - https://github.com/eipi1/cloud-discovery-kubernetes
//!

use async_trait::async_trait;
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

/// Represents an instance
#[derive(Debug, Getters, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    #[getset(get = "pub")]
    instance_id: Option<String>,
    #[getset(get = "pub")]
    service_id: Option<String>,
    #[getset(get = "pub")]
    host: Option<String>,
    #[getset(get = "pub")]
    port: Option<usize>,
    #[getset(get = "pub")]
    //org.springframework.cloud.kubernetes.discovery.DefaultIsServicePortSecureResolver#resolve
    secure: bool,
    #[getset(get = "pub")]
    uri: Option<String>,
    #[getset(get = "pub")]
    metadata: HashMap<String, String>,
    #[getset(get = "pub")]
    scheme: Option<String>,
}

impl ServiceInstance {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        instance_id: Option<String>,
        service_id: Option<String>,
        host: Option<String>,
        port: Option<usize>,
        secure: bool,
        uri: Option<String>,
        metadata: HashMap<String, String>,
        scheme: Option<String>,
    ) -> Self {
        ServiceInstance {
            instance_id,
            service_id,
            host,
            port,
            secure,
            uri,
            metadata,
            scheme,
        }
    }
}

/// All discovery service provider must implement the trait. Note that, it's based on [async_trait](https://docs.rs/async-trait)
#[async_trait]
pub trait DiscoveryService {
    /// Returns list of instances
    async fn discover_instances(&self) -> Result<Vec<ServiceInstance>, Box<dyn Error>>;
}

/// Bridge between [DiscoveryService] and their clients.
#[allow(dead_code)]
pub struct DiscoveryClient<T> {
    service: T,
}

#[allow(dead_code)]
impl<T: DiscoveryService> DiscoveryClient<T> {
    pub fn new(ds: T) -> DiscoveryClient<T> {
        DiscoveryClient { service: ds }
    }

    /// Returns a list of discovered instances
    pub async fn get_instances(&self) -> Result<Vec<ServiceInstance>, Box<dyn Error>> {
        self.service.discover_instances().await
    }
}
