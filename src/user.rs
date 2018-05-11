use chrono::DateTime;
use chrono::Local;
use role::Role;
use serde_json;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: Uuid,
    image: String,
    email: String,
    display_name: String,
    username: String,
    password: String,
    state: State,
    created_at: DateTime<Local>,
    updated_at: Option<DateTime<Local>>,
    last_online: Option<DateTime<Local>>,
    roles: Vec<Role>,
}

#[allow(dead_code)]
impl User {
    pub fn new(email: String, display_name: String, username: String, password: String) -> User {
        User {
            id: Uuid::new_v4(),
            image: "".to_string(),
            email,
            display_name,
            username,
            password,
            state: State::Offline,
            created_at: Local::now(),
            updated_at: None,
            last_online: None,
            roles: Vec::new(),
        }
    }

    pub fn print_user(&self) {
        println!(
            "Image: {:?}\n email: {:?}\n display_name: {:?}\n created at: {:?}",
            self.image, self.email, self.display_name, self.created_at
        );
    }

    pub fn copy_roles(&self) -> Vec<Role> {
        self.roles.clone()
    }

    pub fn grant_role(&mut self, role: &Role) {
        self.roles.push(role.clone());
    }

    pub fn revoke_role(&mut self, role: &Role) {
        self.roles
            .iter()
            .position(|role| role.eq(role))
            .map(|n| self.roles.remove(n));
    }

    pub fn has_role(&mut self, role: &Role) -> bool {
        self.roles.contains(role)
    }

    pub fn update_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn copy_id(&self) -> Uuid {
        self.id.clone()
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum State {
    Online,
    Offline,
    Away,
    Busy,
}

// this test exists simply to print a serialized user object (cargo test -- --nocapture)
#[test]
fn test_user_serialize() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "user1".to_string(),
        "username1".to_string(),
        "password1".to_string(),
    );

    println!("{:?}", serde_json::ser::to_string(&user));

    assert!(true);
}

#[test]
fn test_role() {
    let mut user = User::new(
        "user@example.com".to_string(),
        "user1".to_string(),
        "username1".to_string(),
        "password1".to_string(),
    );
    let role = Role::new(11, "test".to_string()).unwrap();

    user.grant_role(&role);
    assert!(user.has_role(&role));

    user.revoke_role(&role);
    assert!(!user.has_role(&role));
}
