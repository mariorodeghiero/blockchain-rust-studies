use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::UnorderedMap;
use near_sdk::{AccountId, near_bindgen, env};
use near_sdk::serde::{Deserialize, Serialize};


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone)]
pub struct Post {
    id: u128,
    title: String,
    description: String,
    tags: Vec<String>,
    media: String,
    users_who_liked: Vec<AccountId>,
    owner_id: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize,)]
pub struct SocialNetworking {
    posts: UnorderedMap<u128, Post>,
    number_of_posts: u128,
    likes_by_user_id: UnorderedMap<AccountId, Vec<Post>>,
    posts_by_tag: UnorderedMap<String, Vec<Post>>,
}

impl Default for SocialNetworking {
    fn default() -> Self {
        Self {
            posts: UnorderedMap::new(b'm'),
            number_of_posts: 0,
            likes_by_user_id: UnorderedMap::new(b"n"),
            posts_by_tag: UnorderedMap::new(b"o"),
        }
    }   
}

#[near_bindgen]
impl SocialNetworking {
    pub fn add_post(&mut self, title: String, description: String, tags: String, media: String) -> Post {

        let tags_iterator = tags.split(",");
        let mut tags = Vec::<String>::new();
        for tag in tags_iterator {
            tags.push(tag.to_string());
        }

        let post = Post {
            id: self.number_of_posts,
            title,
            description,
            tags: tags.clone(),
            media,
            users_who_liked: Vec::<AccountId>::new(),
            owner_id: env::signer_account_id(), // add the signer account id
        };
        
        self.posts.insert(post.id, post.clone());
        self.number_of_posts += 1;
        return post;
    }
} 