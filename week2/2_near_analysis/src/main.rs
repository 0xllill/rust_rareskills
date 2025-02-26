use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;
use near_sdk::serde::Serialize;
use near_sdk::store::Vector;
use near_sdk::{env, near_bindgen, AccountId, NearToken};

/** SUMMARY OF THE CODE
 * Allow anyone to create an instance of a contract allowing external users to send messages to the contract
 * A message is premium if the attached deposit is greater than or equal to POINT_ONE
 * the contract store all the messages in a vector
 * the contract provide 3 functions:
 * - add_message: allow to add a message to the contract
 * - get_messages: allow to get the messages from the contract
 * - total_messages: allow to get the total number of messages in the contract
 */


//E set one to 100 
const POINT_ONE: NearToken = NearToken::from_millinear(100);

//E @question what is serde ?
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
//E create PostedMessage struct
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
//E create struct contract
pub struct Contract {
    messages: Vector<PostedMessage>,
}

//E implements the Default trait which by default create of vector of messages struct
impl Default for Contract {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"),
        }
    }
}

//E implements the Contract trait
#[near_bindgen]
impl Contract {

    //E set add_message function of contract impl to payable (allow receiving native tokens)
    #[payable]
    pub fn add_message(&mut self, text: String) {

        //E check if the attached deposit (native tokens) is greater than or equal to POINT_ONE
        let premium = env::attached_deposit() >= POINT_ONE;

        //E get the sender of the transaction
        let sender = env::predecessor_account_id();

        //E create a new PostedMessage struct
        let message = PostedMessage {
            premium,
            sender,
            text,
        };

        //E push the new message to the messages vector
        self.messages.push(message);
    }

    //E get_messages function of contract impl
    pub fn get_messages(
        &self, //E contract impl instance
        from_index: Option<U64>, //E from index of the messages vector
        limit: Option<U64> //E limit of the messages vector
    ) 
        -> Vec<&PostedMessage> //E return a vector of posted messages
    {

        //E fetch message from_index (try to unwrap the from_index or set the from_index to 0 by default)
        let from = u64::from(from_index.unwrap_or(U64(0)));

        //E fetch message limit => try to unwrap the limit or set the limit to 10 by default
        let limit = u64::from(limit.unwrap_or(U64(10)));

        //E iterate over the messages vector and return the messages collected
        self.messages
            .iter()
            .skip(from as usize)
            .take(limit as usize)
            .collect()
    }

    //E get_total_messages function of contract impl
    pub fn total_messages(&self) -> u32 {
        self.messages.len()
    }
}


// -- TESTS PART -- // 

#[cfg(test)]
mod tests {
    use super::*;

    //E add_message test
    #[test]
    fn add_message() {
        //E create a new contract instance
        let mut contract = Contract::default();

        //E add a message to the contract
        contract.add_message("A message".to_string());

        //E get the first message from the contract
        let posted_message = &contract.get_messages(None, None)[0];

        //E assert the message is not premium (more than 100 tokens attached to the TX)
        assert_eq!(posted_message.premium, false);

        //E assert the message text is "A message"
        assert_eq!(posted_message.text, "A message".to_string());
    }

    //E iters_messages test
    #[test]
    fn iters_messages() {
        //E create a new contract instance
        let mut contract = Contract::default();

        //E add 3 messages to the contract
        contract.add_message("1st message".to_string());
        contract.add_message("2nd message".to_string());
        contract.add_message("3rd message".to_string());

        //E get the total number of messages
        let total = &contract.total_messages();

        //E assert the total number of messages is 3
        assert!(*total == 3);

        //E get the last details message from the contract 
        let last_message = &contract.get_messages(Some(U64::from(1)), Some(U64::from(2)))[1];

        //E assert the last message is not premium (more than 100 tokens attached to the TX)
        assert_eq!(last_message.premium, false);

        //E assert the last message text is "3rd message"
        assert_eq!(last_message.text, "3rd message".to_string());
    }
}