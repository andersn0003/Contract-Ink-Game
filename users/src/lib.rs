#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod users {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;

    
    /// Followers for a user.
    #[ink::storage_item]
    #[derive(Debug)]
    pub struct Followers {
        /// Mapping of Followed, to List of Followers.
        list: Mapping<AccountId, Vec<AccountId>>,
        /// Mapping of followed account to number of followers.
        length: Mapping<AccountId, u32>,
    }

    #[ink(storage)]
    pub struct Users {
        /// Mapping of User count to index.
        users: Mapping<AccountId, u32>,
        /// Users count.
        user_count: u32,
        /// Followers.
        followers: Followers
    }

    #[ink(event)]
    pub struct CreateUser {
        id: AccountId,
        index: u32
    }

    #[ink(event)]
    pub struct FollowUser {
        follower: AccountId,
        followed: AccountId,
        follower_count: u32
    }

    #[ink(event)]
    pub struct UnFollowUser {
        follower: AccountId,
        followed: AccountId,
        follower_count: u32
    }

    impl Users {
        /// Creates a new User smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { users: Mapping::default(), user_count:0,  followers: Followers { list: Mapping::default(), length: Mapping::default() } }
        }

        /// Checks if the user account is registered.
        #[ink(message)]
        pub fn verify_user(&self,id:AccountId) {
            assert!(self.users.get(id) != None, "Account not found!");
        }

        /// Creates a user in the contract.
        #[ink(message)]
        pub fn create_user(&mut self) {
            self.users.insert(self.env().caller(), &self.user_count);
            self.followers.length.insert(self.env().caller(), &0);
            self.env().emit_event(CreateUser{ id: self.env().caller(), index: self.user_count});
            self.user_count += 1;
        }

        /// Follows a user.
        #[ink(message)]
        pub fn follow_user(&mut self, id:AccountId) {
            self.verify_user(self.env().caller());
            let mut followers_list: Vec<ink::primitives::AccountId> =self.followers.list.get(&id).expect("Account not found!"); 
            followers_list.push(self.env().caller());
            self.followers.list.insert(&id,&followers_list);
            let length: &u32 = &(self.followers.length.get(id)).expect("Followers not set!");
            self.followers.length.insert(id, &(length +1));
            self.env().emit_event(FollowUser{ follower: self.env().caller(), followed: id, follower_count: length+1 })
        }

        /// Unfollows a user.
        #[ink(message)]
        pub fn unfollow_user(&mut self, id:AccountId) {
            let mut followers_list: Vec<ink::primitives::AccountId> =self.followers.list.get(&id).expect("Account not found!"); 
            followers_list =  followers_list.into_iter().filter(|&x| x != self.env().caller()).collect::<Vec<ink::primitives::AccountId>>();
            self.followers.list.insert(&id, &followers_list);
            let length: &u32 = &(self.followers.length.get(id)).expect("Followers not set!");
            self.followers.length.insert(id, &(length -1));
            self.env().emit_event(UnFollowUser{ follower: self.env().caller(), followed: id, follower_count: length-1 })

        }

        #[ink(message)]
        pub fn get_followers(&self, id:AccountId) -> u32 {
            return self.followers.length.get(id).expect("You have no followers!");
        }

        

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn create_user_test() {
            let mut user: Users = Users::new();
            user.create_user();
            assert_eq!(user.user_count, 1);
        }

        #[ink::test]
        fn verify_user_test() {
            let account = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut user: Users = Users::new();
            user.create_user();
            user.verify_user(account.alice);
        }

    }
   
}