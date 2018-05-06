#[derive(PartialEq,Clone,Debug)]
pub struct Role{
    id:u8,
    display_name:String
}

#[allow(dead_code)]
impl Role{

    pub fn new(id:u8,display_name:String)->Option<Role>{
        if id > 10{
            return Some(Role{id,display_name});
        }
        None
    }

    pub fn print_role(&self){
        println!("Id: {} Role: {}",self.id,self.display_name);
    }

    pub fn generate_admin()-> Role{
        Role{id:0,display_name:"admin".to_string()}
    }

    pub fn generate_moderator()-> Role{
        Role{id:1,display_name:"moderator".to_string()}
    }

    pub fn generate_member()-> Role{
        Role{id:2,display_name:"member".to_string()}
    }
}

