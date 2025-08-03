use consul::agent::{Agent, Service};
use consul::Client;
use std::collections::HashMap;
use uuid::Uuid;
use crate::error::NodeError;

pub struct ServiceRegistrar {
    consul_client: Client,
    service_id: String,
}

impl ServiceRegistrar {
    pub fn new(consul_addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config = consul::Config {
            address: consul_addr.to_string(),
            ..Default::default()
        };
        
        let client = Client::new(config);
        let service_id = Uuid::new_v4().to_string();
        
        Ok(ServiceRegistrar {
            consul_client: client,
            service_id,
        })
    }

    pub async fn register(&self, name: &str, address: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let agent = Agent::new(&self.consul_client);
        
        let service = Service {
            id: Some(self.service_id.clone()),
            name: name.to_string(),
            tags: Some(vec!["blockchain".to_string(), "node".to_string()]),
            address: Some(address.to_string()),
            port: Some(port),
            check: Some(consul::agent::Check {
                http: Some(format!("http://{}:{}/health", address, port)),
                interval: Some("10s".to_string()),
                timeout: Some("5s".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };
        
        agent.register_service(&service).await?;
        Ok(())
    }

    pub async fn deregister(&self, _name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let agent = Agent::new(&self.consul_client);
        agent.deregister_service(&self.service_id).await?;
        Ok(())
    }

    pub async fn update_health(&self, healthy: bool) -> Result<(), Box<dyn std::error::Error>> {
        let agent = Agent::new(&self.consul_client);
        let check_id = format!("service:{}", self.service_id);
        
        if healthy {
            agent.pass_check(&check_id, None).await?;
        } else {
            agent.fail_check(&check_id, None).await?;
        }
        
        Ok(())
    }
}