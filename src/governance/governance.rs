use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u32,
    pub description: String,
    pub votes_for: u32,
    pub votes_against: u32,
    pub is_active: bool,
}

pub struct Governance {
    proposals: Arc<Mutex<HashMap<u32, Proposal>>>,
    next_id: Arc<Mutex<u32>>,
}

impl Governance {
    pub fn new() -> Self {
        Governance {
            proposals: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    pub fn create_proposal(&self, description: String) -> u32 {
        let mut proposals = self.proposals.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

        let proposal = Proposal {
            id: *next_id,
            description,
            votes_for: 0,
            votes_against: 0,
            is_active: true,
        };

        proposals.insert(*next_id, proposal);
        *next_id += 1;
        *next_id - 1
    }

    pub fn vote(&self, proposal_id: u32, vote_for: bool) -> Result<(), String> {
        let mut proposals = self.proposals.lock().unwrap();

        if let Some(proposal) = proposals.get_mut(&proposal_id) {
            if !proposal.is_active {
                return Err("Proposal is no longer active.".to_string());
            }

            if vote_for {
                proposal.votes_for += 1;
            } else {
                proposal.votes_against += 1;
            }
            Ok(())
        } else {
            Err("Proposal not found.".to_string())
        }
    }

    pub fn close_proposal(&self, proposal_id: u32) -> Result<(), String> {
        let mut proposals = self.proposals.lock().unwrap();

        if let Some(proposal) = proposals.get_mut(&proposal_id) {
            proposal.is_active = false;
            Ok(())
        } else {
            Err("Proposal not found.".to_string())
        }
    }

    pub fn get_proposal(&self, proposal_id: u32) -> Option<Proposal> {
        let proposals = self.proposals.lock().unwrap();
        proposals.get(&proposal_id).cloned()
    }

    pub fn get_active_proposals(&self) -> Vec<Proposal> {
        let proposals = self.proposals.lock().unwrap();
        proposals.values().filter(|p| p.is_active).cloned().collect()
    }
}
