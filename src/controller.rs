use external_data_source::RoomDataInterface;
use external_data_source::UserDataInterface;
use role::Role;
use room::Room;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use user::State;
use user::User;
use uuid::Uuid;
use chrono::Local;
use chrono::DateTime;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::hash::Hash;

#[derive(Debug)]
pub struct Controller<U, R>
where
    U: UserDataInterface,
    U: Debug,
    R: RoomDataInterface,
    R: Debug,
{
    user_list: Vec<Uuid>,
    public_rooms: Vec<Room>,
    private_rooms: Vec<Room>,
    user_data_interface: U,
    room_data_interface: R,
}

#[allow(dead_code)]
impl<U, R> Controller<U, R>
where
    U: UserDataInterface,
    U: Debug,
    R: RoomDataInterface,
    R: Debug,
{
    pub fn new(user_interface: U, room_interface: R) -> Controller<U, R> {
        let mut controller = Controller {
            user_list: Vec::new(),
            public_rooms: Vec::new(),
            private_rooms: Vec::new(),
            user_data_interface: user_interface,
            room_data_interface: room_interface,
        };
        controller.fetch_user_data();
        controller.fetch_room_data();
        controller
    }

    fn fetch_user_data(&mut self) {
        self.user_list = self.user_data_interface.provide_user_id_list();
    }

    fn fetch_room_data(&mut self) {
        let mut room_data = self.room_data_interface.provide_room_data();

        for room in room_data {
            if room.is_private() {
                self.private_rooms.push(room);
            } else {
                self.public_rooms.push(room);
            }
        }
    }

    fn find_user_position(&self, user_id: &Uuid) -> Option<usize> {
        self.user_list.iter().position(|id| id.eq(user_id))
    }

    //user based methods
    pub fn add_user(&mut self, user: User) {
        self.user_list.push(user.copy_id());
        self.user_data_interface.store_user(user);
    }

    //TODO: remove user from the database
    pub fn remove_user(&mut self, user_id: &Uuid) -> bool {
        if Controller::<U, R>::remove_uuid_from_vec(&mut self.user_list, user_id) {
            return true;
        }
        false
    }

    pub fn is_user(&self, user_id: &Uuid) -> bool {
        self.user_list.contains(&user_id)
    }

    pub fn find_user(&mut self, user_id: &Uuid) -> Option<User> {
        self.user_data_interface.provide_user(user_id)
    }

    pub fn grant_role(&mut self, user_id: &Uuid, role: &Role){
        match self.find_user(user_id) {
            Some(mut user) => {
                user.grant_role(role);
                self.user_data_interface.update_user(user);
            },
            None => (),
        }
    }

    pub fn revoke_role(&mut self, user_id: &Uuid, role: &Role) {
        match self.find_user(user_id) {
            Some(mut user) => {
                user.revoke_role(role);
                self.user_data_interface.update_user(user);
            },
            None => (),
        }
    }

    pub fn update_state(&mut self, user_id: &Uuid, state: State) {
        match self.find_user(user_id) {
            Some(mut user) =>{
                user.update_state(state);
                self.user_data_interface.update_user(user);
        },
            None => (),
        }
    }

    //room based methods

    pub fn generate_room(&mut self, name: String, owner: Uuid) {
        let mut room = Room::new(name, owner);
        self.add_room(room);
    }

    pub fn add_room(&mut self, room: Room) {

        self.room_data_interface.store_room(room.clone());

        if room.is_private() {
            self.private_rooms.push(room);
        } else {
            self.public_rooms.push(room);
        }
    }

    pub fn remove_room(&mut self, room: &Uuid) -> bool {
        if Controller::<U, R>::remove_room_from_vec(&mut self.public_rooms, room) {
            return true;
        }

        if Controller::<U, R>::remove_room_from_vec(&mut self.private_rooms, room) {
            return true;
        }

        false
    }

    pub fn contains_room(&self, room: &Room) -> bool {
        for x in &self.public_rooms {
            if x.eq(room) {
                return true;
            }
        }

        for y in &self.private_rooms {
            if y.eq(room) {
                return true;
            }
        }
        false
    }

    pub fn add_member_to_room(&mut self, room_id: &Uuid, user_id: Uuid) {
        match self.find_room_match(room_id) {
            Some((counter, room_public)) => {
                if room_public {
                    match self.public_rooms.get_mut(counter) {
                        Some(t) => t.add_member(user_id),
                        None => (),
                    }
                } else {
                    match self.private_rooms.get_mut(counter) {
                        Some(t) => t.add_member(user_id),
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    pub fn remove_member_from_room(&mut self, room_id: &Uuid, user_id: &Uuid) -> bool {
        match self.find_room_match(room_id) {
            Some((counter, room_public)) => {
                if room_public {
                    match self.public_rooms.get_mut(counter) {
                        Some(t) => return t.remove_member(user_id),
                        None => false,
                    }
                } else {
                    return match self.private_rooms.get_mut(counter) {
                        Some(t) => t.remove_member(user_id),
                        None => false,
                    };
                }
            }
            None => false,
        }
    }

    pub fn add_moderator_to_room(&mut self, room_id: &Uuid, user_id: Uuid) {
        match self.find_room_match(room_id) {
            Some((counter, room_public)) => {
                if room_public {
                    match self.public_rooms.get_mut(counter) {
                        Some(t) => {
                            t.add_moderator(user_id);
                            if !t.has_member(&user_id) {
                                t.add_member(user_id);
                            }
                        }
                        None => (),
                    }
                } else {
                    match self.private_rooms.get_mut(counter) {
                        Some(t) => {
                            t.add_moderator(user_id);
                            if !t.has_member(&user_id) {
                                t.add_member(user_id);
                            }
                        }
                        None => (),
                    }
                }
            }
            None => (),
        };
    }

    pub fn remove_moderator_from_room(&mut self, room_id: &Uuid, user_id: &Uuid) -> bool {
        match self.find_room_match(room_id) {
            Some((counter, room_public)) => {
                if room_public {
                    match self.public_rooms.get_mut(counter) {
                        Some(t) => t.remove_moderator(user_id),
                        None => false,
                    }
                } else {
                    match self.private_rooms.get_mut(counter) {
                        Some(t) => t.remove_moderator(user_id),
                        None => false,
                    }
                }
            }
            None => false,
        }
    }

    pub fn ban_member(&mut self, room_id: &Uuid, user_id: Uuid) {
        match self.find_room_match(room_id) {
            Some((counter, room_public)) => {
                if room_public {
                    match self.public_rooms.get_mut(counter) {
                        Some(t) => {
                            t.bann_member(user_id);
                            t.remove_member(&user_id);
                            t.remove_moderator(&user_id);
                        }
                        None => (),
                    };
                } else {
                    match self.private_rooms.get_mut(counter) {
                        Some(t) => {
                            t.bann_member(user_id);
                            t.remove_member(&user_id);
                            t.remove_moderator(&user_id);
                        }
                        None => (),
                    };
                }
            }
            None => (),
        }
    }

    pub fn unban_member(&mut self, room_id: &Uuid, user_id: Uuid) -> bool {
        match self.find_room_match(room_id) {
            Some((counter, room_public)) => {
                if room_public {
                    match self.public_rooms.get_mut(counter) {
                        Some(t) => return t.unbann_member(user_id),
                        None => false,
                    }
                } else {
                    return match self.private_rooms.get_mut(counter) {
                        Some(t) => t.unbann_member(user_id),
                        None => false,
                    };
                }
            }
            None => false,
        }
    }

    pub fn mute_member(&mut self, room_id: &Uuid, user_id: Uuid) {
        match self.find_room_match(room_id) {
            Some((counter, room_public)) => {
                if room_public {
                    match self.public_rooms.get_mut(counter) {
                        Some(t) => t.mute_member(user_id),
                        None => (),
                    }
                } else {
                    match self.private_rooms.get_mut(counter) {
                        Some(t) => t.mute_member(user_id),
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    pub fn unmute_member(&mut self, room_id: &Uuid, user_id: &Uuid) -> bool {
        match self.find_room_match(room_id) {
            Some((counter, room_public)) => {
                if room_public {
                    match self.public_rooms.get_mut(counter) {
                        Some(t) => return t.unmute_member(user_id),
                        None => false,
                    }
                } else {
                    return match self.private_rooms.get_mut(counter) {
                        Some(t) => t.unmute_member(user_id),
                        None => false,
                    };
                }
            }
            None => false,
        }
    }

    pub fn find_room(&self, id: &Uuid) -> Option<&Room> {
        match self.find_room_match(id) {
            Some((counter, public_room)) => {
                if public_room {
                    return self.public_rooms.get(counter);
                } else {
                    return self.private_rooms.get(counter);
                }
            }
            None => None,
        }
    }

    pub fn find_mut_room(&mut self, id: &Uuid) -> Option<&mut Room>{
        match self.find_room_match(id) {
            Some((counter, public_room)) => {
                if public_room {
                    return self.public_rooms.get_mut(counter);
                } else {
                    return self.private_rooms.get_mut(counter);
                }
            }
            None => None,
        }
    }

    fn find_room_match(&self, id: &Uuid) -> Option<(usize, bool)> {
        let mut counter = 0;
        let mut matched = false;

        for x in &self.public_rooms {
            if x.eq_by_uuid(id) {
                matched = true;
                break;
            }
            counter += 1;
        }

        if matched {
            return Some((counter, true));
        }
        counter = 0;

        for y in &self.private_rooms {
            if y.eq_by_uuid(id) {
                matched = true;
                break;
            }
            counter += 1;
        }

        if matched {
            return Some((counter, false));
        }

        None
    }

    pub fn join_room(&mut self,room_id:&Uuid,user_id:&Uuid) -> bool{

        let admin_flag = self.verify_admin(user_id);

        match self.find_mut_room(room_id){

            Some(mut room) => {

                if room.is_private(){
                    if room.has_member(user_id){
                        room.add_online_member(user_id);
                        return true;
                    }else if admin_flag{
                        room.add_online_member(user_id);
                        return true;
                    }
                }else{
                    room.add_online_member(user_id);
                    return true;
                }

                return false;
            },
            None => false
        }
    }

    pub fn leave_room(&mut self, room_id:&Uuid,user_id:&Uuid){
        match self.find_mut_room(room_id){
            Some(mut room) => room.remove_online_member(user_id),
            None => ()
        }
    }

    //additional methods

    fn remove_room_from_vec(list: &mut Vec<Room>, reference: &Uuid) -> bool {
        list.iter()
            .position(|ref n| n.get_id() == reference)
            .map(|e| list.remove(e))
            .is_some()
    }

    fn remove_uuid_from_vec(list: &mut Vec<Uuid>, reference: &Uuid) -> bool {
        list.iter()
            .position(|ref n| n == &reference)
            .map(|e| list.remove(e))
            .is_some()
    }

    //admin related methods

    //TODO: change experimental stage! => save token in database
    pub fn generate_invite_token(&mut self, admin_id: &Uuid) -> Option<u64>{

        if self.verify_admin(admin_id){

            let mut hasher = DefaultHasher::new();
            let token = (admin_id.clone().to_string() + &DateTime::from(Local::now()).to_string()).hash(&mut hasher);
            return Some(hasher.finish());
        }
        None
    }

    pub fn verify_admin(&mut self, admin_id :&Uuid) -> bool{

        match self.find_user(admin_id){
            Some(mut value) => value.has_role(&Role::generate_admin()),
            None => return false
        }

    }
}

#[test]
fn test_interface() {
    use mock_data::*;
    let mut user_data_interface = MockUserDataImpl::new();
    let user_data = user_data_interface.provide_user_data();
    let mut room_data_interface = MockRoomDataImpl::new(&user_data);
    let mut controller = Controller::new(user_data_interface, room_data_interface);

    assert_eq!(
        *user_data.get(0).unwrap(),
        controller
            .find_user(user_data.get(0).unwrap().get_id())
            .unwrap()
    );

    assert!(true);
}

#[test]
fn test_room() {
    use mock_data::*;
    use user::User;

    let owner = User::new(
        "testinator@example.com".to_string(),
        "Test Test".to_string(),
        "testinator".to_string(),
        "1234567".to_string(),
    );
    let room = Room::new("Testroom".to_string(), owner.copy_id());

    let mut user_data_interface = MockUserDataImpl::new();
    let user_data = user_data_interface.provide_user_data();
    let mut room_data_interface = MockRoomDataImpl::new(&user_data);
    let mut controller = Controller::new(user_data_interface, room_data_interface);

    let id = room.copy_id();

    //the controller consumes the "inserted" room entirely
    controller.add_room(room);
    assert_eq!(id, *controller.find_room(&id).unwrap().get_id());
    controller.remove_room(&id);
    assert_eq!(None, controller.find_room(&id));
}

#[test]
fn test_member() {
    use mock_data::*;
    use user::User;

    let owner = User::new(
        "testinator@example.com".to_string(),
        "Test Test".to_string(),
        "testinator".to_string(),
        "1234567".to_string(),
    );
    let user = User::new(
        "blubb@example.com".to_string(),
        "Test Test".to_string(),
        "blubb".to_string(),
        "1234567".to_string(),
    );
    let room = Room::new("Testroom".to_string(), owner.copy_id());
    let id = room.copy_id();

    let mut user_data_interface = MockUserDataImpl::new();
    let user_data = user_data_interface.provide_user_data();
    let mut room_data_interface = MockRoomDataImpl::new(&user_data);
    let mut controller = Controller::new(user_data_interface, room_data_interface);

    controller.add_room(room);
    controller.add_member_to_room(&id, user.copy_id());
    assert!(controller.find_room(&id).unwrap().has_member(user.get_id()));
    controller.remove_member_from_room(&id, user.get_id());
    assert!(!controller.find_room(&id).unwrap().has_member(user.get_id()));
}

#[test]
fn test_moderator() {
    use mock_data::*;
    use user::User;

    let owner = User::new(
        "testinator@example.com".to_string(),
        "Test Test".to_string(),
        "testinator".to_string(),
        "1234567".to_string(),
    );
    let user = User::new(
        "blubb@example.com".to_string(),
        "Test Test".to_string(),
        "blubb".to_string(),
        "1234567".to_string(),
    );
    let room = Room::new("Testroom".to_string(), owner.copy_id());
    let id = room.copy_id();

    let mut user_data_interface = MockUserDataImpl::new();
    let user_data = user_data_interface.provide_user_data();
    let mut room_data_interface = MockRoomDataImpl::new(&user_data);
    let mut controller = Controller::new(user_data_interface, room_data_interface);

    controller.add_room(room);
    controller.add_moderator_to_room(&id, user.copy_id());
    assert!(
        controller
            .find_room(&id)
            .unwrap()
            .has_moderator(user.get_id())
    );
    controller.remove_moderator_from_room(&id, user.get_id());
    assert!(!controller
        .find_room(&id)
        .unwrap()
        .has_moderator(user.get_id()));
}

#[test]
fn test_ban() {
    use mock_data::*;
    use user::User;

    let owner = User::new(
        "testinator@example.com".to_string(),
        "Test Test".to_string(),
        "testinator".to_string(),
        "1234567".to_string(),
    );
    let user = User::new(
        "blubb@example.com".to_string(),
        "Test Test".to_string(),
        "blubb".to_string(),
        "1234567".to_string(),
    );
    let room = Room::new("Testroom".to_string(), owner.copy_id());
    let id = room.copy_id();

    let mut user_data_interface = MockUserDataImpl::new();
    let user_data = user_data_interface.provide_user_data();
    let mut room_data_interface = MockRoomDataImpl::new(&user_data);
    let mut controller = Controller::new(user_data_interface, room_data_interface);

    controller.add_room(room);
    controller.add_member_to_room(&id, user.copy_id());
    controller.ban_member(&id, user.copy_id());
    assert!(
        controller
            .find_room(&id)
            .unwrap()
            .is_member_banned(user.get_id())
    );
    controller.unban_member(&id, user.copy_id());
    assert!(!controller
        .find_room(&id)
        .unwrap()
        .is_member_banned(user.get_id()));
}

#[test]
fn test_mute() {
    use mock_data::*;
    use user::User;

    let owner = User::new(
        "testinator@example.com".to_string(),
        "Test Test".to_string(),
        "testinator".to_string(),
        "1234567".to_string(),
    );
    let user = User::new(
        "blubb@example.com".to_string(),
        "Test Test".to_string(),
        "blubb".to_string(),
        "1234567".to_string(),
    );
    let room = Room::new("Testroom".to_string(), owner.copy_id());
    let id = room.copy_id();

    let mut user_data_interface = MockUserDataImpl::new();
    let user_data = user_data_interface.provide_user_data();
    let mut room_data_interface = MockRoomDataImpl::new(&user_data);
    let mut controller = Controller::new(user_data_interface, room_data_interface);

    controller.add_room(room);
    controller.add_member_to_room(&id, user.copy_id());
    controller.mute_member(&id, user.copy_id());
    assert!(
        controller
            .find_room(&id)
            .unwrap()
            .is_member_muted(user.get_id())
    );
    controller.unmute_member(&id, user.get_id());
    assert!(!controller
        .find_room(&id)
        .unwrap()
        .is_member_muted(user.get_id()));
}

#[test]
fn test_admin(){
    use mock_data::*;
    let mut user_data_interface = MockUserDataImpl::new();
    let user_data = user_data_interface.provide_user_data();
    let admin_id = user_data.get(0).unwrap().copy_id();
    let mut room_data_interface = MockRoomDataImpl::new(&user_data);
    let mut controller = Controller::new(user_data_interface, room_data_interface);
    let admin_role = Role::generate_admin();

    controller.grant_role(&admin_id,&admin_role);

    assert!(controller.find_user(&admin_id).unwrap().has_role(&admin_role));

    println!("{:?}",controller.generate_invite_token(&admin_id).unwrap());
}

#[test]
fn test_join(){
    use mock_data::*;
    let mut user_data_interface = MockUserDataImpl::new();
    let user_data = user_data_interface.provide_user_data();
    let admin_id = user_data.get(0).unwrap().copy_id();
    let mut room_data_interface = MockRoomDataImpl::new(&user_data);
    let mut controller = Controller::new(user_data_interface, room_data_interface);
    let admin_role = Role::generate_admin();

    let user = User::new(
        "blubb@example.com".to_string(),
        "Test Test".to_string(),
        "blubb".to_string(),
        "1234567".to_string(),
    );

    let mut room = Room::new(String::from("test room "),user.copy_id());
    //room.set_private(false);
    let room_id = room.copy_id();

    controller.grant_role(&admin_id,&admin_role);
    controller.add_room(room);
    controller.join_room(&room_id,&admin_id);
    assert!(controller.find_room(&room_id).unwrap().get_online_members().contains(&admin_id));

}
