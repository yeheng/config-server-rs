use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::RaftConfig;

#[derive(Debug, Clone, PartialEq)]
pub enum NodeState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone)]
pub struct RaftNode {
    config: Arc<RaftConfig>,
    state: Arc<RwLock<NodeState>>,
    current_term: Arc<RwLock<u64>>,
    voted_for: Arc<RwLock<Option<String>>>,
}

impl RaftNode {
    pub async fn new(config: RaftConfig) -> Result<Self> {
        Ok(Self {
            config: Arc::new(config),
            state: Arc::new(RwLock::new(NodeState::Follower)),
            current_term: Arc::new(RwLock::new(0)),
            voted_for: Arc::new(RwLock::new(None)),
        })
    }

    pub async fn start(&self) -> Result<()> {
        // Start election timer
        self.start_election_timer().await?;
        
        // Start heartbeat timer if leader
        self.start_heartbeat_timer().await?;

        Ok(())
    }

    async fn start_election_timer(&self) -> Result<()> {
        let config = self.config.clone();
        let state = self.state.clone();
        let current_term = self.current_term.clone();
        let voted_for = self.voted_for.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                std::time::Duration::from_millis(config.election_timeout)
            );

            loop {
                interval.tick().await;
                let mut state_guard = state.write().await;
                let mut term_guard = current_term.write().await;
                let mut voted_guard = voted_for.write().await;

                match *state_guard {
                    NodeState::Follower => {
                        // Start election
                        *state_guard = NodeState::Candidate;
                        *term_guard += 1;
                        *voted_guard = Some(config.node_id.clone());
                    }
                    NodeState::Candidate => {
                        // Request votes
                        // TODO: Implement vote request logic
                    }
                    NodeState::Leader => {
                        // Reset election timer
                    }
                }
            }
        });

        Ok(())
    }

    async fn start_heartbeat_timer(&self) -> Result<()> {
        let config = self.config.clone();
        let state = self.state.clone();
        let current_term = self.current_term.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                std::time::Duration::from_millis(config.heartbeat_interval)
            );

            loop {
                interval.tick().await;
                let state_guard = state.read().await;
                let term_guard = current_term.read().await;

                if *state_guard == NodeState::Leader {
                    // Send heartbeat to all peers
                    // TODO: Implement heartbeat logic
                }
            }
        });

        Ok(())
    }

    pub async fn get_state(&self) -> NodeState {
        self.state.read().await.clone()
    }

    pub async fn get_term(&self) -> u64 {
        *self.current_term.read().await
    }

    pub async fn get_voted_for(&self) -> Option<String> {
        self.voted_for.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_raft_node_creation() {
        let config = RaftConfig {
            node_id: "node1".to_string(),
            data_dir: std::path::PathBuf::from("/tmp/raft"),
            peers: vec!["node2".to_string(), "node3".to_string()],
            heartbeat_interval: 100,
            election_timeout: 1000,
        };

        let node = RaftNode::new(config).await;
        assert!(node.is_ok());
    }

    #[tokio::test]
    async fn test_raft_state_transition() {
        let config = RaftConfig {
            node_id: "node1".to_string(),
            data_dir: std::path::PathBuf::from("/tmp/raft"),
            peers: vec!["node2".to_string(), "node3".to_string()],
            heartbeat_interval: 100,
            election_timeout: 1000,
        };

        let node = RaftNode::new(config).await.unwrap();
        assert_eq!(node.get_state().await, NodeState::Follower);
    }
}
