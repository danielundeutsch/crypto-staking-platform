use crate::error::ApiError;
use consul::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ServiceRegistry {
    consul_client: Client,
    cache: Arc<RwLock<HashMap<String, String>>>,
}

impl ServiceRegistry {
    pub async fn new(_consul_addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = consul::Config::new()?;
        
        let client = Client::new(config);
        
        Ok(ServiceRegistry {
            consul_client: client,
            cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn list_services(&self) -> Result<Vec<String>, ApiError> {
        // For now, return a static list of services
        // In production, this would query Consul's catalog
        let mut services = vec!["api-gateway".to_string()];
        
        // Check cache for registered services
        let cache = self.cache.read().await;
        for service_name in cache.keys() {
            if !services.contains(service_name) {
                services.push(service_name.clone());
            }
        }
        
        Ok(services)
    }

    pub async fn get_service(&self, service_name: &str) -> Result<Option<String>, ApiError> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(url) = cache.get(service_name) {
                return Ok(Some(url.clone()));
            }
        }

        // For demonstration, use hardcoded service URLs
        // In production, this would query Consul
        let url = match service_name {
            "ethereum-node" => Some("http://localhost:8081".to_string()),
            "bitcoin-node" => Some("http://localhost:8082".to_string()),
            "solana-node" => Some("http://localhost:8083".to_string()),
            _ => None,
        };

        if let Some(service_url) = url {
            // Update cache
            let mut cache = self.cache.write().await;
            cache.insert(service_name.to_string(), service_url.clone());
            Ok(Some(service_url))
        } else {
            Ok(None)
        }
    }

    pub async fn register_service(
        &self,
        _name: &str,
        _address: &str,
        _port: u16,
    ) -> Result<(), ApiError> {
        // Service registration would be implemented here
        // For now, we'll rely on services self-registering with Consul
        Ok(())
    }
}