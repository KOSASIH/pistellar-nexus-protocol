use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use log::{info, error};

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u32,
    pub description: String,
    pub votes_for: u32,
    pub votes_against: u32,
    pub is_active: bool,
    pub created_at: Instant,
    pub voting_duration: Duration,
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

    pub fn create_proposal(&self, description: String, voting_duration: Duration) -> u32 {
        let mut proposals = self.proposals.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

        let proposal = Proposal {
            id: *next_id,
            description,
            votes_for: 0,
            votes_against: 0,
            is_active: true,
            created_at: Instant::now(),
            voting_duration,
        };

        proposals.insert(*next_id, proposal);
        *next_id += 1;
        info!("Created proposal: {:?}", proposals.get(&(*next_id - 1)).unwrap());
        *next_id - 1
    }

    pub fn vote(&self, proposal_id: u32, vote_for: bool) -> Result<(), String> {
        let mut proposals = self.proposals.lock().unwrap();

        if let Some(proposal) = proposals.get_mut(&proposal_id) {
            if !proposal.is_active {
                return Err("Proposal is no longer active.".to_string());
            }

            if proposal.created_at.elapsed() > proposal.voting_duration {
                proposal.is_active = false; // Automatically close the proposal if the voting period has expired
                return Err("Voting period has expired.".to_string());
            }

            if vote_for {
                proposal.votes_for += 1;
            } else {
                proposal.votes_against += 1;
            }
            info!("Vote recorded for proposal ID {}: {}", proposal_id, vote_for);
            Ok(())
        } else {
            error!("Proposal not found: ID {}", proposal_id);
            Err("Proposal not found.".to_string())
        }
    }

    pub fn close_proposal(&self, proposal_id: u32) -> Result<(), String> {
        let mut proposals = self.proposals.lock().unwrap();

        if let Some(proposal) = proposals.get_mut(&proposal_id) {
            proposal.is_active = false;
            info!("Closed proposal ID {}: {:?}", proposal_id, proposal);
            Ok(())
        } else {
            error!("Proposal not found: ID {}", proposal_id);
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

// Example usage of logging setup
fn setup_logging() {
    use log::LevelFilter;
    use simplelog::{Config, SimpleLogger};

    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();
}

fn main() {
    setup_logging();
    let governance = Governance::new();
    
    // Example of creating a proposal
    let proposal_id = governance.create_proposal("Increase budget for project X".to_string(), Duration::new(60, 0));
    println!("Created proposal with ID: {}", proposal_id);
    
    // Example of voting
    if let Err(e) = governance.vote(proposal_id, true) {
        println!("Error voting: {}", e);
    }
    
    // Example of closing a proposal
    if let Err(e) = governance.close_proposal(proposal_id) {
        println!("Error closing proposal: {}", e);
    }
            }
