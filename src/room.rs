use uuid::Uuid;
use chrono::Local;
use chrono::DateTime;
use utils::utils::*;


#[derive(Debug)]
pub struct Room<'a>{
    id:Uuid,
    name:String,
    owner:&'a Uuid,
    members:Vec<&'a Uuid>,
    topic:String,
    private: bool,
    hidden: bool,
    moderators:Vec<&'a Uuid>,
    created_at:DateTime<Local>,
    updated_at:Option<DateTime<Local>>,
    last_message_at:Option<DateTime<Local>>,
    messages:Vec<String>,
    banned_users:Vec<&'a Uuid>,
    muted_users:Vec<&'a Uuid>
}

#[allow(dead_code)]
impl<'a> Room<'a>{
    pub fn new(name:String,owner:&'a Uuid)->Room{
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

    pub fn add_member(&mut self,member_id:&'a Uuid){
        self.members.push(member_id);
    }

    pub fn remove_member(&mut self,member_id:&'a Uuid) -> bool{
        remove_ref_from_ref_vec(&mut self.members,&member_id)
    }

    pub fn add_moderator(&mut self,member_id:&'a Uuid){
        self.moderators.push(member_id);
    }

    pub fn remove_moderator(&mut self,member_id:&'a Uuid) -> bool{
        remove_ref_from_ref_vec(&mut self.moderators,member_id)
    }

    pub fn mute_member(&mut self,member_id:&'a Uuid){
        self.muted_users.push(member_id);
    }

    pub fn unmute_member(&mut self,member_id:&'a Uuid) -> bool{
        remove_ref_from_ref_vec(&mut self.muted_users,member_id)
    }

    pub fn bann_member(&mut self,member_id:&'a Uuid){
        self.banned_users.push(member_id);
        self.remove_member(member_id);
        self.remove_moderator(member_id);
    }

    pub fn unbann_member(&mut self,member_id:&'a Uuid) -> bool{
        if remove_ref_from_ref_vec(&mut self.banned_users,member_id){
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

    pub fn get_id(&self) -> &Uuid{
        &self.id
    }

    pub fn copy_id(&self) -> Uuid{
        self.id.clone()
    }

}

impl<'a> PartialEq for Room<'a>{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}