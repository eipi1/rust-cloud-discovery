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

#[derive(Debug, Getters, Clone, Serialize, Deserialize)]
pub struct Port {
    name: Option<String>,
    port: u32,
    protocol: String,
    app_protocol: Option<String>,
}

impl Port {
    pub fn new(
        name: Option<String>,
        port: u32,
        protocol: String,
        app_protocol: Option<String>,
    ) -> Self {
        Self {
            name,
            port,
            protocol,
            app_protocol,
        }
    }

    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }

    pub fn get_port(&self) -> u32 {
        self.port
    }

    pub fn get_protocol(&self) -> &str {
        &self.protocol
    }

    pub fn get_app_protocol(&self) -> &Option<String> {
        &self.app_protocol
    }
}

/// Represents an instance
#[derive(Debug, Getters, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    #[getset(get = "pub")]
    instance_id: Option<String>,
    #[getset(get = "pub")]
    service_id: Option<String>,
    host: Option<String>,
    ports: Option<Vec<Port>>,
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
        ports: Option<Vec<Port>>,
        secure: bool,
        uri: Option<String>,
        metadata: HashMap<String, String>,
        scheme: Option<String>,
    ) -> Self {
        ServiceInstance {
            instance_id,
            service_id,
            host,
            ports,
            secure,
            uri,
            metadata,
            scheme,
        }
    }

    /// get host/IP of the instance
    pub fn host(&self) -> &Option<String> {
        &self.host
    }

    /// get all available ports
    pub fn get_ports(&self) -> &Option<Vec<Port>> {
        &self.ports
    }

    /// Get if the default port uses TLS.
    /// Selection of default port depends on the implementation
    pub fn is_secure(&self) -> bool {
        self.secure
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
