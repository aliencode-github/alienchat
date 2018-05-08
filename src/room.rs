use uuid::Uuid;
use chrono::Local;
use chrono::DateTime;


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Room{
    id:Uuid,
    name:String,
    owner:Uuid,
    members:Vec<Uuid>,
    topic:String,
    private: bool,
    hidden: bool,
    moderators:Vec<Uuid>,
    created_at:DateTime<Local>,
    updated_at:Option<DateTime<Local>>,
    last_message_at:Option<DateTime<Local>>,
    messages:Vec<String>,
    banned_users:Vec<Uuid>,
    muted_users:Vec<Uuid>
}

#[allow(dead_code)]
impl Room{
    pub fn new(name:String,owner:Uuid)->Room{
        Room {
            id: Uuid::new_v4(),
            name,
            owner,
            members: vec![owner],
            topic: "".to_string(),
            private: true,
            hidden: false,
            moderators: Vec::new(),
            created_at: Local::now(),
            updated_at: None,
            last_message_at: None,
            messages: Vec::new(),
            banned_users: Vec::new(),
            muted_users: Vec::new()
        }
    }

    pub fn is_private(&self) -> bool{
        self.private
    }

    pub fn is_hidden(&self) -> bool{
        self.hidden
    }

    pub fn generate_time_tupel(&self)->(DateTime<Local>,Option<DateTime<Local>>,Option<DateTime<Local>>){
        (self.created_at,self.updated_at,self.last_message_at)
    }

    pub fn add_member(&mut self,member_id:Uuid){
        self.members.push(member_id);
    }

    pub fn remove_member(&mut self,member_id:&Uuid) -> bool{
        remove_ref(&mut self.members, member_id)
    }

    pub fn add_moderator(&mut self,member_id:Uuid){
        self.moderators.push(member_id);
    }

    pub fn remove_moderator(&mut self,member_id:&Uuid) -> bool{
        remove_ref(&mut self.moderators, &member_id)
    }

    pub fn mute_member(&mut self,member_id:Uuid){
        self.muted_users.push(member_id);
    }

    pub fn unmute_member(&mut self,member_id:&Uuid) -> bool{
        remove_ref(&mut self.muted_users, member_id)
    }

    pub fn bann_member(&mut self,member_id:Uuid){
        self.banned_users.push(member_id);
        self.remove_member(&member_id);
        self.remove_moderator(&member_id);
    }

    pub fn unbann_member(&mut self,member_id:Uuid) -> bool{
        if remove_ref(&mut self.banned_users, &member_id){
            self.add_member(member_id);
            return true;
        }
        false
    }

    pub fn provide_messages(&mut self) -> &mut Vec<String>{
        &mut self.messages
    }

    pub fn has_member(&self, user:&Uuid) -> bool{
        self.members.contains(&user)
    }

    pub fn has_moderator(&self, user:&Uuid) -> bool{
        self.moderators.contains(&user)
    }

    pub fn is_member_muted(&self, user:&Uuid) -> bool{
        self.muted_users.contains(&user)
    }

    pub fn is_member_banned(&self, user:&Uuid) -> bool{
        self.banned_users.contains(&user)
    }

    pub fn count_member(&self)-> usize{
        self.members.len()
    }

    pub fn eq_by_uuid(&self,uuid:&Uuid) -> bool{
        self.id == *uuid
    }

    pub fn copy_id(&self) -> Uuid{
        self.id.clone()
    }

    pub fn get_id(&self) -> &Uuid{
        &self.id
    }
}

fn remove_ref(list:&mut Vec<Uuid>, element:&Uuid) -> bool{
    list.iter()
        .position(|&n| n == *element )
        .map(|e| list.remove(e))
        .is_some()
}

impl PartialEq for Room{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[test]
fn test_room_serialize(){
    use user::User;
    use serde_json;
    let user = User::new("user@example.com".to_string(),"user1".to_string(),"username1".to_string(),"password1".to_string());
    let mut room = Room::new("Testroom".to_string(),user.copy_id());
    println!("{:?}",serde_json::ser::to_string(&room));

    assert!(true);
}

/*
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct RoomDTO{
    pub id:Uuid,
    pub name:String,
    pub owner:Uuid,
    pub members:Vec<Uuid>,
    pub topic:String,
    pub private: bool,
    pub hidden: bool,
    pub moderators:Vec<Uuid>,
    pub created_at:DateTime<Local>,
    pub updated_at:Option<DateTime<Local>>,
    pub last_message_at:Option<DateTime<Local>>,
    pub messages:Vec<String>,
    pub banned_users:Vec<Uuid>,
    pub muted_users:Vec<Uuid>
}

impl RoomDTO{
    pub fn new(room:&mut Room) -> Self{
        RoomDTO{
            id:room.copy_id(),
            name:room.name.clone(),
            owner:room.owner.clone(),
            members:dereference_vec(&mut room.members),
            topic:room.topic.clone(),
            private:room.private,
            hidden:room.hidden,
            moderators:dereference_vec(&mut room.moderators),
            created_at: room.created_at,
            updated_at: room.updated_at,
            last_message_at: room.last_message_at,
            messages:room.messages.clone(),
            banned_users:dereference_vec(&mut room.banned_users),
            muted_users:dereference_vec(&mut room.muted_users)
        }
    }
}

fn dereference_vec(list:&mut Vec<&Uuid>) -> Vec<Uuid>{
    list.iter().map(|s| **s).collect()
}

*/