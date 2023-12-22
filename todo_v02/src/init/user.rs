use uuid::Uuid;
pub struct User{
    pub user_id: String,
    pub user_pw: String,
    pub status: bool,
    pub uuid: String,
}

impl User{
    
    pub fn new(user_id: String, user_pw: String, status: bool) -> Self{
        Self{
            user_id,
            user_pw,
            status,
            uuid: Uuid::new_v4().to_string(),
        }
    }
}

pub mod utils{
    use super::*;

    pub fn login(user_id: String, user_pw: String) -> User{
        // get user from DB
        let user_id_clone= user_id.clone();
        let user_pw_clone= user_pw.clone();
        let mut user= User::new(user_id, user_pw, false);

        // hasing pw
        
        // if hashed == hashing
        if user_id_clone == user.user_id && user_pw_clone == user.user_pw {
            user.status= true;
        }

        user
    }
}