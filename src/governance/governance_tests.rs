#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_proposal() {
        let governance = Governance::new();
        let proposal_id = governance.create_proposal("Increase community funding".to_string());

        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.description, "Increase community funding");
        assert_eq!(proposal.votes_for, 0);
        assert_eq!(proposal.votes_against, 0);
        assert!(proposal.is_active);
    }

    #[test]
    fn test_vote_for_proposal() {
        let governance = Governance::new();
        let proposal_id = governance.create_proposal("Increase community funding".to_string());

        governance.vote(proposal_id, true).unwrap();
        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.votes_for, 1);
        assert_eq!(proposal.votes_against, 0);
    }

    #[test]
    fn test_vote_against_proposal() {
        let governance = Governance::new();
        let proposal_id = governance.create_proposal("Increase community funding".to_string());

        governance.vote(proposal_id, false).unwrap();
        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert_eq!(proposal.votes_for, 0);
        assert_eq!(proposal.votes_against, 1);
    }

    #[test]
    fn test_close_proposal() {
        let governance = Governance::new();
        let proposal_id = governance.create_proposal("Increase community funding".to_string());

        governance.close_proposal(proposal_id).unwrap();
        let proposal = governance.get_proposal(proposal_id).unwrap();
        assert!(!proposal.is_active);
    }

    #[test]
    fn test_vote_on_closed_proposal() {
        let governance = Governance::new();
        let proposal_id = governance.create_proposal("Increase community funding".to_string());

        governance.close_proposal(proposal_id).unwrap();
        let result = governance.vote(proposal_id, true);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Proposal is no longer active.");
    }

    #[test]
    fn test_get_active_proposals() {
        let governance = Governance::new();
        let proposal_id1 = governance.create_proposal("Increase community funding".to_string());
        let proposal_id2 = governance.create_proposal("Improve community safety".to_string());

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
}
