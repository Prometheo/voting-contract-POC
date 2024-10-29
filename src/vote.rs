use near_sdk::{Gas, PromiseError, PromiseOrValue, log};
use crate::*;
use crate::ext::{list_contract, XCC_SUCCESS};


#[near(serializers=[borsh, json])]
pub struct Vote {
    pub voter: AccountId,
    pub candidate_id: AccountId,
    pub weight: u32,  // Used for both weighted and point-based voting
    pub timestamp: U64,
}


#[near_bindgen]
impl Contract {
    pub fn vote(&mut self, election_id: ElectionId, votes: Vec<(AccountId, u32)>) -> PromiseOrValue<bool> {
        let election = self.elections.get(&election_id).cloned().expect("Election not found");
        assert!(
            env::block_timestamp() >= election.start_date.0 &&
                env::block_timestamp() <= election.end_date.0,
            "Voting period is not active"
        );

        let voter = env::predecessor_account_id();
        return  self.assert_voter_eligible(&election, &voter, votes)
    }

    fn record_vote(&mut self, voter: &AccountId, candidate_id: &AccountId, weight: u32, election_id: &ElectionId) {
        let vote = Vote {
            voter: voter.clone(),
            candidate_id: candidate_id.clone(),
            weight,
            timestamp: U64(env::block_timestamp()),
        };
        let election_votes = self.votes.get_mut(&election_id).expect("Votes not found for election");

        let voter_votes = election_votes.entry(voter.clone()).or_insert_with(Vec::new);
        voter_votes.push(vote);

        let candidates_map = self.candidates.get_mut(&election_id).expect("Candidates map not found");
        let candidate = candidates_map.get_mut(candidate_id).expect("Candidate not found");
        candidate.votes_received += weight as u64;

    }


    pub(crate) fn assert_voter_eligible(&mut self, election: &Election, voter: &AccountId, votes: Vec<(AccountId, u32)>) -> PromiseOrValue<bool> {
        match &election.voter_eligibility {
            EligibilityType::Open => {
                PromiseOrValue::Value(
                    self.handle_voting(voter, election, votes)
                )
            },
            EligibilityType::ListBased(contract_id, list_id) => {
                let promise = list_contract::ext(contract_id.clone())
                    .with_static_gas(Gas::from_tgas(5))
                    .is_registered(Some(list_id.0 as u64), voter.clone());
                return PromiseOrValue::Promise(promise.then(
                    Self::ext(env::current_account_id())
                        .with_static_gas(Gas::from_tgas(XCC_SUCCESS))
                        .eligible_voting_callback(voter, election, votes)
                ))
            },
            EligibilityType::TokenBased(_token_contract, _min_balance) => {
                unimplemented!()
                // env::log_str("Token-based eligibility check not fully implemented");
                // PromiseOrValue::Value(true)
            },
            EligibilityType::Custom(_contract_addr) => {
                unimplemented!()
                // env::log_str("Custom eligibility check not fully implemented");
                // PromiseOrValue::Value(true)
            },
        }
    }

    pub fn handle_voting(
        &mut self,
        voter: &AccountId,
        election: &Election,
        votes: Vec<(AccountId, u32)>
    ) -> bool {
        

        match election.voting_type {
            VotingType::Simple => {
                let election_votes = self.votes.get(&election.id).expect("Votes not found for election");
                let current_votes = election_votes.get(voter);
                if let Some(cvt) = current_votes {
                    assert!(
                        cvt.len() as u32 + votes.len() as u32 <= election.votes_per_voter,
                        "Exceeds allowed number of votes"
                    );
                }
                
                for (candidate_id, _) in votes {
                    self.record_vote(&voter, &candidate_id, 1, &election.id);
                }
            },
            VotingType::Weighted(max_weight) => {
                let total_weight: u32 = votes.iter().map(|(_, weight)| *weight).sum();
                assert!(total_weight <= max_weight, "Exceeds maximum allowed weight");
                for (candidate_id, weight) in votes {
                    self.record_vote(&voter, &candidate_id, weight, &election.id);
                }
            },
            VotingType::PointBased(total_points) => {
                let total_used_points: u32 = votes.iter().map(|(_, points)| *points).sum();
                assert!(total_used_points <= total_points, "Exceeds total allowed points");
                for (candidate_id, points) in votes {
                    self.record_vote(&voter, &candidate_id, points, &election.id);
                }
            },
        }
        false
    }

    #[private]
    pub fn eligible_voting_callback(
        &mut self,
        voter: &AccountId,
        election: &Election,
        votes: Vec<(AccountId, u32)>,
        #[callback_result] call_result: Result<bool, PromiseError>,
    ) -> bool {
        if call_result.is_err() {
            log!("There was an error checking eligibility");
            return false;
        }
        // if call_result.unwrap().
        self.handle_voting(voter, election, votes)

    }
}