use near_sdk::json_types::U128;
use crate::*;


#[near(serializers=[borsh, json])]
#[derive(Clone, PartialEq)]
pub enum ElectionType {
    GeneralElection,
    Referendum,
    TokenHolderVote,
    CommitteeElection,
    ProjectProposal(AccountId),
    Pot(AccountId),
    Custom(String, Option<AccountId>),
}


#[near(serializers=[borsh, json])]
#[derive(Clone, PartialEq)]
pub enum ApplicationStatus {
    Pending,
    Approved,
    Rejected,
}


#[near(serializers=[borsh, json])]
pub struct Candidate {
    pub account_id: AccountId,
    pub description: String,
    pub status: ApplicationStatus,
    pub votes_received: u64,
    pub application_date: U64,
    pub approval_date: Option<U64>,
}


#[near(serializers=[borsh, json])]
#[derive(Clone, PartialEq)]
pub enum EligibilityType {
    Open,
    ListBased(AccountId, U128), // list contract and id
    TokenBased(AccountId, U128), // Token contract and minimum balance
    Custom(String), // Custom eligibility contract address
}

#[near(serializers=[borsh, json])]
#[derive(Clone, PartialEq)]
pub enum ElectionStatus {
    Pending,
    NominationPeriod,
    VotingPeriod,
    ChallengePeriod,
    Completed,
    Cancelled,
}


#[near(serializers=[borsh, json])]
#[derive(Clone, PartialEq)]
pub struct Election {
    pub id: ElectionId,
    pub title: String,
    pub description: String,
    pub start_date: U64,
    pub end_date: U64,
    pub nomination_start_date: U64,
    pub nomination_end_date: U64,
    pub votes_per_voter: u32,
    pub voting_type: VotingType,
    pub voter_eligibility: EligibilityType,
    pub auto_approval: bool,
    pub owner: AccountId,
    pub status: ElectionStatus,
    pub challenge_period_end: Option<U64>,
    pub winner_ids: Vec<AccountId>,
    pub election_type: ElectionType,
    pub creating_project: AccountId,
}

pub type ElectionId = u64;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn create_election(
        &mut self,
        title: String,
        description: String,
        start_date: U64,
        end_date: U64,
        nomination_start_date: U64,
        nomination_end_date: U64,
        votes_per_voter: u32,
        voter_eligibility: EligibilityType,
        voting_type: VotingType,
        auto_approval: bool,
        election_type: ElectionType,
    ) -> ElectionId {
        self.assert_admin_or_owner();
        assert!(nomination_end_date.0 <= start_date.0, "Nomination must end before voting starts");
        assert!(start_date.0 < end_date.0, "Start date must be before end date");

        let election_id = self.election_counter;
        self.election_counter += 1;

        let creating_project = env::predecessor_account_id();

        let election = Election {
            id: election_id,
            title,
            description,
            start_date,
            end_date,
            nomination_start_date,
            nomination_end_date,
            votes_per_voter,
            voter_eligibility,
            voting_type,
            auto_approval,
            owner: creating_project.clone(),
            status: ElectionStatus::Pending,
            challenge_period_end: None,
            winner_ids: Vec::new(),
            election_type,
            creating_project,
        };

        self.elections.insert(election_id, election);

        let candidates_map: IterableMap<AccountId, Candidate> =
            IterableMap::new(StorageKey::Candidates { election_id });
        self.candidates.insert(election_id, candidates_map);

        let election_votes: IterableMap<AccountId, Vec<Vote>> =
            IterableMap::new(StorageKey::ElectionVotes { election_id });
        self.votes.insert(election_id, election_votes);

        election_id
    }

    pub fn apply(&mut self, election_id: ElectionId, description: String) {
        let election = self.elections.get(&election_id).expect("Election not found");
        assert!(
            env::block_timestamp() >= election.nomination_start_date.0 &&
                env::block_timestamp() <= election.nomination_end_date.0,
            "Nomination period is not active"
        );

        let applicant = env::predecessor_account_id();
        // assert!(self.is_eligible_candidate(&election, &applicant), "Not eligible to be a candidate");

        let candidate = Candidate {
            account_id: applicant.clone(),
            description,
            status: if election.auto_approval {
                ApplicationStatus::Approved
            } else {
                ApplicationStatus::Pending
            },
            votes_received: 0,
            application_date: U64(env::block_timestamp()),
            approval_date: if election.auto_approval {
                Some(U64(env::block_timestamp()))
            } else {
                None
            },
        };

        let candidates_map = self.candidates.get_mut(&election_id).expect("Candidates map not found");
        candidates_map.insert(applicant, candidate);
        // self.candidates.insert(election_id, &candidates_map);
    }

    pub fn review_application(
        &mut self,
        election_id: ElectionId,
        candidate_id: AccountId,
        status: ApplicationStatus,
    ) {
        self.assert_admin_or_owner();
        let candidates_map = self.candidates.get_mut(&election_id).expect("Candidates map not found");
        let candidate = candidates_map.get_mut(&candidate_id).expect("Candidate not found");

        candidate.status = status;
        if matches!(candidate.status, ApplicationStatus::Approved) {
            candidate.approval_date = Some(U64(env::block_timestamp()));
        }
    }
}