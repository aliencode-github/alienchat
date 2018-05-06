use std::path::Path;
use role::Role;
use chrono::DateTime;
use chrono::Local;
use uuid::Uuid;

#[derive(Debug)]
pub struct User{
    id:Uuid,
    image:Option<Box<Path>>,
    email:String,
    display_name:String,
    username:String,
    password:String,
    state:u8,
    created_at:DateTime<Local>,
    updated_at:Option<DateTime<Local>>,
    last_online:Option<DateTime<Local>>,
    roles: Vec<Role>
}

#[allow(dead_code)]
impl User{
    pub fn new(email:String,display_name:String,username:String,password:String) -> User{
        User{
            id:Uuid::new_v4(),
            image:None,
            email,
            display_name,
            username,
            password,
            state: 0,
            created_at: Local::now(),
            updated_at: None,
            last_online: None,
            roles: Vec::new()
        }
    }

    pub fn print_user(&self){
        println!("Image: {:?}\n email: {:?}\n display_name: {:?}\n created at: {:?}",self.image,self.email,self.display_name,self.created_at);
    }

    pub fn copy_roles(&self) -> Vec<Role>{
        self.roles.clone()
    }

    pub fn grant_role(&mut self,role:Role){
        self.roles.push(role);
    }

    pub fn revoke_role(&mut self,role:&Role){

        let mut counter:usize = 0;
        let mut match_flag = false;

        for x in &self.roles{

            if x.eq(role){
                match_flag = true;
                break;
            }
            counter +=1;
        }

        if match_flag {
            self.roles.remove(counter);
        }

    }

    //states are defiend as 0 => offline 1 => away 2 => busy 3= offline
    pub fn update_state(&mut self,state:u8)-> bool{
        if state < 4 {
            self.state = state;
            return true;
        }

        false
    }

    pub fn get_id(&self)->&Uuid{
        &self.id
    }

    pub fn copy_id(&self) -> Uuid{
        self.id.clone()
    }

}

impl PartialEq for User{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}