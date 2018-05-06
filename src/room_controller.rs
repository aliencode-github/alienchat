use room::Room;
use uuid::Uuid;

#[derive(Debug)]
pub struct RoomController<'a>{
    public_rooms:Vec<Room<'a>>,
    private_rooms:Vec<Room<'a>>
}

#[allow(dead_code)]
impl<'a> RoomController<'a>{
    pub fn new() -> RoomController<'a>{
        RoomController{public_rooms:Vec::new(),private_rooms:Vec::new()}
    }

    pub fn add_room(&mut self,room:Room<'a>){
        if room.is_private(){
            self.private_rooms.push(room);
        }else{
            self.public_rooms.push(room);
        }
    }

    pub fn remove_room(&mut self, room:&Uuid) -> bool{

        if RoomController::remove_room_from_vec(&mut self.public_rooms,room){
            return true;
        }

        if RoomController::remove_room_from_vec(&mut self.private_rooms,room){
            return true;
        }

        false
    }

    fn remove_room_from_vec(list:&mut Vec<Room>,room:&Uuid) -> bool{

        list.iter()
            .position(|ref n| n.get_id() == room )
            .map(|e| list.remove(e))
            .is_some()

    }

    pub fn contains_room(&self,room:&Room) -> bool{
        for x in &self.public_rooms{
            if x.eq(room){
                return true;
            }
        }

        for y in &self.private_rooms{
            if y.eq(room){
                return true;
            }
        }
        false
    }

    pub fn add_member_to_room(&mut self,room_id:&Uuid,user_id:&'a Uuid){

        match self.find_room_match(room_id){
            Some((counter,room_public)) => {
                if room_public{
                    match self.public_rooms.get_mut(counter){
                        Some(t) => t.add_member(user_id),
                        None => ()
                    }
                }else{
                    match self.private_rooms.get_mut(counter){
                        Some(t) => t.add_member(user_id),
                        None => ()
                    }
                }
            },
            None => ()
        }
    }

    pub fn remove_member_from_room(&mut self,room_id:&Uuid,user_id:&'a Uuid) -> bool{

        match self.find_room_match(room_id){
            Some((counter,room_public)) => {
                if room_public{
                    match self.public_rooms.get_mut(counter){
                        Some(t) => return t.remove_member(user_id),
                        None => false
                    }
                }else{
                    return match self.private_rooms.get_mut(counter){
                        Some(t) => t.remove_member(user_id),
                        None => false
                    };
                }
            },
            None => false
        }

    }

    pub fn add_moderator_to_room(&mut self,room_id:&Uuid,user_id:&'a Uuid){

        match self.find_room_match(room_id){
            Some((counter,room_public)) => {
                if room_public{
                    match self.public_rooms.get_mut(counter){
                        Some(t) =>{
                            t.add_moderator(user_id);
                            if !t.has_member(user_id){
                                t.add_member(user_id);
                            }
                        },
                        None => ()
                    }
                }else{
                    match self.private_rooms.get_mut(counter){
                        Some(t) =>{
                            t.add_moderator(user_id);
                            if !t.has_member(user_id){
                                t.add_member(user_id);
                            }
                        },
                        None => ()
                    }
                }
            },
            None => ()
        };

    }

    pub fn remove_moderator_from_room(&mut self,room_id:&Uuid, user_id:&'a Uuid) -> bool{
        match self.find_room_match(room_id){
            Some((counter,room_public)) => {
                if room_public{
                    match self.public_rooms.get_mut(counter){
                        Some(t) => t.remove_moderator(user_id),
                        None => false
                    }
                }else{
                    match self.private_rooms.get_mut(counter){
                        Some(t) => t.remove_moderator(user_id),
                        None => false
                    }
                }
            },
            None => false
        }
    }

    pub fn ban_member(&mut self,room_id:&Uuid,user_id:&'a Uuid){
        match self.find_room_match(room_id){
            Some((counter,room_public)) => {
                if room_public{
                    match self.public_rooms.get_mut(counter){
                        Some(t) =>{
                            t.bann_member(user_id);
                            t.remove_member(user_id);
                            t.remove_moderator(user_id);
                        },
                        None => ()
                    };
                }else{
                    match self.private_rooms.get_mut(counter){
                        Some(t) => {
                            t.bann_member(user_id);
                            t.remove_member(user_id);
                            t.remove_moderator(user_id);
                        },
                        None => ()
                    };
                }
            },
            None => ()
        }
    }

    pub fn unban_member(&mut self,room_id:&Uuid,user_id:&'a Uuid) -> bool{
        match self.find_room_match(room_id){
            Some((counter,room_public)) => {
                if room_public{
                    match self.public_rooms.get_mut(counter){
                        Some(t) => return t.unbann_member(user_id),
                        None => false
                    }
                }else{
                    return match self.private_rooms.get_mut(counter){
                        Some(t) => t.unbann_member(user_id),
                        None => false
                    };
                }
            },
            None => false
        }
    }

    pub fn mute_member(&mut self, room_id:&Uuid,user_id:&'a Uuid){
        match self.find_room_match(room_id){
            Some((counter,room_public)) => {
                if room_public{
                    match self.public_rooms.get_mut(counter){
                        Some(t) => t.mute_member(user_id),
                        None => ()
                    }
                }else{
                    match self.private_rooms.get_mut(counter){
                        Some(t) => t.mute_member(user_id),
                        None => ()
                    }
                }
            },
            None => ()
        }
    }

    pub fn unmute_member(&mut self, room_id:&Uuid,user_id:&'a Uuid) -> bool{
        match self.find_room_match(room_id){
            Some((counter,room_public)) => {
                if room_public{
                    match self.public_rooms.get_mut(counter){
                        Some(t) => return t.unmute_member(user_id),
                        None => false
                    }
                }else{
                    return match self.private_rooms.get_mut(counter){
                        Some(t) => t.unmute_member(user_id),
                        None => false
                    };
                }
            },
            None => false
        }
    }

    pub fn find_room(&self, id:&Uuid) -> Option<& Room>{

        match self.find_room_match(id){
            Some((counter,public_room)) =>{
                if public_room {
                    return self.public_rooms.get(counter);
                }else{
                    return self.private_rooms.get(counter);
                }
            },
            None => None
        }
    }

    pub fn find_mut_room(&mut self, id:&Uuid) -> Option<&'a mut Room>{

        match self.find_room_match(id){
            Some((counter,public_room)) =>{
                if public_room {
                    return self.public_rooms.get_mut(counter);
                }else{
                    return self.private_rooms.get_mut(counter);
                }
            },
            None => None

        }
    }

    fn find_room_match(&self, id:&Uuid) ->Option<(usize,bool)>{
        let mut counter = 0;
        let mut matched = false;

        for x in &self.public_rooms{
            if x.eq_by_uuid(id){
                matched = true;
                break;
            }
            counter+=1;
        }

        if matched{
            return Some((counter,true));
        }
        counter = 0;

        for y in &self.private_rooms{
            if y.eq_by_uuid(id){
                matched = true;
                break;
            }
            counter +=1;
        }

        if matched{
            return Some((counter,false));
        }

        None
    }
}


#[test]
fn test_room(){
    use user::User;
    let owner = User::new("testinator@example.com".to_string(), "Test Test".to_string(), "testinator".to_string(), "1234567".to_string());
    let room = Room::new("Testroom".to_string(),owner.get_id());
    let mut controller = RoomController::new();

    let id = room.copy_id();

    //the controller consumes the "inserted" room entirely
    controller.add_room(room);
    assert_eq!(id,*controller.find_room(&id).unwrap().get_id());
    controller.remove_room(&id);
    assert_eq!(None,controller.find_room(&id));
}

#[test]
fn test_member(){
    use user::User;
    let owner = User::new("testinator@example.com".to_string(), "Test Test".to_string(), "testinator".to_string(), "1234567".to_string());
    let user = User::new("blubb@example.com".to_string(), "Test Test".to_string(), "blubb".to_string(), "1234567".to_string());
    let room = Room::new("Testroom".to_string(),owner.get_id());
    let mut controller = RoomController::new();
    let id = room.copy_id();

    controller.add_room(room);
    controller.add_member_to_room(&id,user.get_id());
    assert!(controller.find_room(&id).unwrap().has_member(user.get_id()));
    controller.remove_member_from_room(&id,user.get_id());
    assert!(!controller.find_room(&id).unwrap().has_member(user.get_id()));
}

#[test]
fn test_moderator(){
    use user::User;
    let owner = User::new("testinator@example.com".to_string(), "Test Test".to_string(), "testinator".to_string(), "1234567".to_string());
    let user = User::new("blubb@example.com".to_string(), "Test Test".to_string(), "blubb".to_string(), "1234567".to_string());
    let room = Room::new("Testroom".to_string(),owner.get_id());
    let mut controller = RoomController::new();
    let id = room.copy_id();

    controller.add_room(room);
    controller.add_moderator_to_room(&id,user.get_id());
    assert!(controller.find_room(&id).unwrap().has_moderator(user.get_id()));
    controller.remove_moderator_from_room(&id,user.get_id());
    assert!(!controller.find_room(&id).unwrap().has_moderator(user.get_id()));
}

#[test]
fn test_ban(){
    use user::User;
    let owner = User::new("testinator@example.com".to_string(), "Test Test".to_string(), "testinator".to_string(), "1234567".to_string());
    let user = User::new("blubb@example.com".to_string(), "Test Test".to_string(), "blubb".to_string(), "1234567".to_string());
    let room = Room::new("Testroom".to_string(),owner.get_id());
    let mut controller = RoomController::new();
    let id = room.copy_id();

    controller.add_room(room);
    controller.add_member_to_room(&id,user.get_id());
    controller.ban_member(&id,user.get_id());
    assert!(controller.find_room(&id).unwrap().is_member_banned(user.get_id()));
    controller.unban_member(&id,user.get_id());
    assert!(!controller.find_room(&id).unwrap().is_member_banned(user.get_id()));
}

#[test]
fn test_mute(){
    use user::User;
    let owner = User::new("testinator@example.com".to_string(), "Test Test".to_string(), "testinator".to_string(), "1234567".to_string());
    let user = User::new("blubb@example.com".to_string(), "Test Test".to_string(), "blubb".to_string(), "1234567".to_string());
    let room = Room::new("Testroom".to_string(),owner.get_id());
    let mut controller = RoomController::new();
    let id = room.copy_id();

    controller.add_room(room);
    controller.add_member_to_room(&id,user.get_id());
    controller.mute_member(&id,user.get_id());
    assert!(controller.find_room(&id).unwrap().is_member_muted(user.get_id()));
    controller.unmute_member(&id,user.get_id());
    assert!(!controller.find_room(&id).unwrap().is_member_muted(user.get_id()));
}