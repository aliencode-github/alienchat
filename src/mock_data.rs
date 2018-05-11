use external_data_source::RoomDataInterface;
use external_data_source::UserDataInterface;
use room::Room;
use user::User;
use uuid::Uuid;

#[derive(Debug)]
pub struct MockUserDataImpl {
    user_data: Vec<User>,
}

impl MockUserDataImpl {
    pub fn new() -> MockUserDataImpl {
        MockUserDataImpl {
            user_data: vec![
                User::new(
                    "user3@example.com".to_string(),
                    "User 3".to_string(),
                    "user3".to_string(),
                    "password1".to_string(),
                ),
                User::new(
                    "user4@example.com".to_string(),
                    "User 4".to_string(),
                    "user4".to_string(),
                    "password1".to_string(),
                ),
                User::new(
                    "user5@example.com".to_string(),
                    "User 5".to_string(),
                    "user5".to_string(),
                    "password1".to_string(),
                ),
                User::new(
                    "user6@example.com".to_string(),
                    "User 6".to_string(),
                    "user6".to_string(),
                    "password1".to_string(),
                ),
                User::new(
                    "user7@example.com".to_string(),
                    "User 7".to_string(),
                    "user7".to_string(),
                    "password1".to_string(),
                ),
                User::new(
                    "user8@example.com".to_string(),
                    "User 8".to_string(),
                    "user8".to_string(),
                    "password1".to_string(),
                ),
                User::new(
                    "user9@example.com".to_string(),
                    "User 9".to_string(),
                    "user9".to_string(),
                    "password1".to_string(),
                ),
            ],
        }
    }
}

impl UserDataInterface for MockUserDataImpl {
    fn provide_user_data(&mut self) -> Vec<User> {
        self.user_data.clone()
    }

    fn provide_user_id_list(&mut self) -> Vec<Uuid> {
        self.user_data.iter().map(|user| user.copy_id()).collect()
    }

    fn provide_user(&mut self, user_id: &Uuid) -> Option<User> {
        for user in &self.user_data {
            if user.get_id().eq(user_id) {
                return Some(user.clone());
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct MockRoomDataImpl {
    room_data: Vec<Room>,
}

impl MockRoomDataImpl {
    pub fn new(user_list: &Vec<User>) -> Self {
        MockRoomDataImpl {
            room_data: vec![
                Room::new(
                    "testroom one".to_string(),
                    user_list.get(0).unwrap().copy_id(),
                ),
                Room::new(
                    "testroom two".to_string(),
                    user_list.get(1).unwrap().copy_id(),
                ),
            ],
        }
    }
}

impl RoomDataInterface for MockRoomDataImpl {
    fn provide_room_data(&mut self) -> Vec<Room> {
        self.room_data.clone()
    }

    fn provide_room(&mut self, room_id: &Uuid) -> Option<Room> {
        for room in &self.room_data {
            if room.get_id().eq(room_id) {
                return Some(room.clone());
            }
        }
        None
    }
}
