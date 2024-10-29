# voting-contract

A flexible NEAR blockchain smart contract implementing various voting and election mechanisms, suitable for DAOs, governance systems, and other decentralized voting needs. To be used for PotV3 as way to catalog on chain votes for voting mechanisms. 

For more information on Pot v2 Contracts https://github.com/PotLock/core/tree/feat/pot-v2/contracts
Potlock Docs https://docs.potlock.io

## Features

### Election Types
- **General Election**: Standard election format
- **Referendum**: For yes/no decisions
- **Token Holder Vote**: Specific to token holders
- **Committee Election**: For committee member selection
- **Project Proposal**: Linked to specific project accounts
- **Pot**: Related to fund allocation
- **Custom**: Customizable with optional account association

### Voting Mechanisms
1. **Simple Voting**
   - Fixed number of votes per voter
   - One vote per candidate
   - Total votes limited by `votes_per_voter`

2. **Weighted Voting**
   - Voters distribute a maximum weight among candidates
   - Flexible allocation (e.g., 60/40 split between candidates)
   - Maximum weight configurable

3. **Point-Based Voting**
   - Total point allocation per voter
   - Points can be distributed among candidates
   - Total points configurable

### Voter Eligibility
- **Open**: Anyone can vote
- **List Based**: Must be on a specific list (verified via cross-contract calls)
- **Token Based**: Must hold minimum token balance
- **Custom**: Customizable eligibility rules

### Election Lifecycle
1. Pending
2. Nomination Period
3. Voting Period
4. Challenge Period
5. Completed
6. Cancelled

### Security Features
- Admin/owner controls for sensitive operations
- Cross-contract calls for eligibility verification
- Timestamp-based period enforcement
- Vote weight and point limitations
- Challenge period support

## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near build
```

## How to Test Locally?

```bash
cargo test
```

## How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.
To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near deploy <account-id>
```

## Contract Usage

### Creating an Election
```rust
create_election(
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
)
```

### Applying as a Candidate
```rust
apply(
    election_id: ElectionId,
    description: String
)
```

### Voting
```rust
vote(
    election_id: ElectionId,
    votes: Vec<(AccountId, u32)>
)
```

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
