#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::thread::sleep;

    #[test]
    fn test_create_proposal() {
        let governance = Governance::new();
        let proposal_id = governance.create_proposal("Increase community funding".to_string(), Duration::new(60, 0));

        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.description, "Increase community funding");
        assert_eq!(proposal.votes_for, 0);
        assert_eq!(proposal.votes_against, 0);
        assert!(proposal.is_active);
    }

    #[test]
    fn test_vote_for_proposal() {
        let governance = Governance::new();
        let proposal_id = governance.create_proposal("Increase community funding".to_string(), Duration::new(60, 0));

        governance.vote(proposal_id, true).unwrap();
        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.votes_for, 1);
        assert_eq!(proposal.votes_against, 0);
    }

    #[test]
    fn test_vote_against_proposal() {
        let governance = Governance::new();
        let proposal_id = governance.create_proposal("Increase community funding".to_string(), Duration::new(60, 0));

        governance.vote(proposal_id, false).unwrap();
        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.votes_for, 0);
        assert_eq!(proposal.votes_against, 1);
    }

    #[test]
    fn test_close_proposal() {
        let governance = Governance::new();
        let proposal_id = governance.create_proposal("Increase community funding".to_string(), Duration::new(60, 0));

        governance.close_proposal(proposal_id).unwrap();
        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert!(!proposal.is_active);
    }

    #[test]
    fn test_vote_on_closed_proposal() {
        let governance = Governance::new();
        let proposal_id = governance.create_proposal("Increase community funding".to_string(), Duration::new(60, 0));

        governance.close_proposal(proposal_id).unwrap();
        let result = governance.vote(proposal_id, true);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Proposal is no longer active.");
    }

    #[test]
    fn test_get_active_proposals() {
        let governance = Governance::new();
        let proposal_id1 = governance.create_proposal("Increase community funding".to_string(), Duration::new(60, 0));
        let proposal_id2 = governance.create_proposal("Improve community safety".to_string(), Duration::new(60, 0));

        let active_proposals = governance.get_active_proposals();
        assert_eq!(active_proposals.len(), 2);

        governance.close_proposal(proposal_id1).unwrap();
        let active_proposals_after_closing = governance.get_active_proposals();
        assert_eq!(active_proposals_after_closing.len(), 1);
        assert_eq!(active_proposals_after_closing[0].id, proposal_id2);
    }

    #[test]
    fn test_get_nonexistent_proposal() {
        let governance = Governance::new();
        let proposal = governance.get_proposal(999);
        assert!(proposal.is_none());
    }

    #[test]
    fn test_vote_after_voting_period_expires() {
        let governance = Governance::new();
        let proposal_id = governance.create_proposal("Increase community funding".to_string(), Duration::new(1, 0)); // 1 second voting period

        // Sleep for 2 seconds to ensure the voting period has expired
        sleep(Duration::new(2, 0));

        let result = governance.vote(proposal_id, true);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Voting period has expired.");
    }

    #[test]
    fn test_create_proposal_with_zero_duration() {
        let governance = Governance::new();
        let proposal_id = governance.create_proposal("Instant proposal".to_string(), Duration::new(0, 0)); // 0 seconds voting period

        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert!(!proposal.is_active); // Proposal should be inactive immediately
    }
                        }
