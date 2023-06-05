#[cfg(test)]
use crate::Agent;

#[test]
fn test_agent_creation() {
    let _ = Agent::new();
}
#[test]
fn test_actionmap_retrieval() {
    let agent = Agent::new();
    assert!(agent
        .actionmap
        .get("SubredditFeedElements".into())
        .is_some());
}

#[test]
fn test_actionmap_update() {
    let mut agent = Agent::new();
    agent.actionmap.refresh().unwrap();
}

#[test]
fn test_actionmap_not_empty() {
    let agent = Agent::new();
    assert!(!agent.actionmap.len() > 0);
}
