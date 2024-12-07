// src/governance.rs
use std::collections::HashMap;

pub struct Governance {
    proposals: HashMap ```rust
<String, Proposal>,
    votes: HashMap<String, HashMap<String, bool>>,
}

pub struct Proposal {
    pub description: String,
    pub votes_for: u32,
    pub votes_against: u32,
}

impl Governance {
    pub fn new() -> Self {
        Governance {
            proposals: HashMap::new(),
            votes: HashMap::new(),
        }
    }

    pub fn create_proposal(&mut self, id: String, description: String) {
        self.proposals.insert(id.clone(), Proposal {
            description,
            votes_for: 0,
            votes_against: 0,
        });
    }

    pub fn vote(&mut self, proposal_id: String, voter: String, support: bool) {
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            if support {
                proposal.votes_for += 1;
            } else {
                proposal.votes_against += 1;
            }
            self.votes.entry(proposal_id).or_insert_with(HashMap::new).insert(voter, support);
        }
    }

    pub fn get_results(&self, proposal_id: &str) -> Option<&Proposal> {
        self.proposals.get(proposal_id)
    }
}
