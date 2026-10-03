use rmcp::service::RoleClient;
use rmcp::service::RunningService;
use rmcp::transport::TokioChildProcess;
use rmcp::ServiceExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::process::Command;

pub struct McpClient {
    service: Arc<RunningService<RoleClient, ()>>,
}

impl McpClient {
    pub async fn connect(
        command_name: &str,
        args: &[String],
        env: HashMap<String, String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut cmd = Command::new(command_name);
        cmd.args(args);
        for (k, v) in env {
            cmd.env(k, v);
        }

        // TokioChildProcess::new takes &mut Command and returns io::Result
        let transport = TokioChildProcess::new(&mut cmd)?;
        let service = ().serve(transport).await?;

        Ok(Self {
            service: Arc::new(service),
        })
    }

    pub fn service(&self) -> &RunningService<RoleClient, ()> {
        &self.service
    }
}

