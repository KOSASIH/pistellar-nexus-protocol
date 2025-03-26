use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub description: String,
    pub votes_for: u32,
    pub votes_against: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {
    pub voter: String,
    pub support: bool,
}

pub struct Governance {
    proposals: Arc<Mutex<HashMap<String, Proposal>>>,
    votes: Arc<Mutex<HashMap<String, HashMap<String, Vote>>>>,
}

impl Governance {
    pub fn new() -> Self {
        Governance {
            proposals: Arc::new(Mutex::new(HashMap::new())),
            votes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_proposal(&self, id: String, description: String) {
        let mut proposals = self.proposals.lock().unwrap();
        proposals.insert(id.clone(), Proposal {
            id,
            description,
            votes_for: 0,
            votes_against: 0,
        });
    }

    pub fn vote(&self, proposal_id: String, voter: String, support: bool) {
        let mut proposals = self.proposals.lock().unwrap();
        let mut votes = self.votes.lock().unwrap();

        if let Some(proposal) = proposals.get_mut(&proposal_id) {
            if support {
                proposal.votes_for += 1;
            } else {
                proposal.votes_against += 1;
            }

            votes.entry(proposal_id).or_insert_with(HashMap::new).insert(voter, Vote {
                voter,
                support,
            });
        }
    }

    pub fn get_results(&self, proposal_id: &str) -> Option<Proposal> {
        let proposals = self.proposals.lock().unwrap();
        proposals.get(proposal_id).cloned()
    }

    pub fn get_votes(&self, proposal_id: &str) -> Option<Vec<Vote>> {
        let votes = self.votes.lock().unwrap();
        votes.get(proposal_id).map(|votes| votes.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_proposal() {
        let governance = Governance::new();
        governance.create_proposal("1".to_string(), "Test proposal".to_string());

        let proposal = governance.get_results("1").unwrap();
        assert_eq!(proposal.description, "Test proposal");
        assert_eq!(proposal.votes_for, 0);
        assert_eq!(proposal.votes_against, 0);
    }

    #[test]
    fn test_vote() {
        let governance = Governance::new();
        governance.create_proposal("1".to_string(), "Test proposal".to_string());
        governance.vote("1".to_string(), "voter1".to_string(), true);

        let proposal = governance.get_results("1").unwrap();
        assert_eq!(proposal.votes_for, 1);
        assert_eq!(proposal.votes_against, 0);

        let votes = governance.get_votes("1").unwrap();
        assert_eq!(votes.len(), 1);
        assert_eq!(votes[0].voter, "voter1");
        assert!(votes[0].support);
    }
            }
