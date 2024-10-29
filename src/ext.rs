use near_sdk::{AccountId, ext_contract};

pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;
type ListId = u64;
type RegistrantId = AccountId;
// Validator interface, for cross-contract calls
#[ext_contract(list_contract)]
trait ListContract {
    fn is_registered(&self, list_id: Option<ListId>, account_id: RegistrantId) -> String;
    // fn set_greeting(&self, greeting: String);
}