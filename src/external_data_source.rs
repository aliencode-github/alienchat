use room::Room;
use user::User;
use uuid::Uuid;

pub trait UserDataInterface {
    fn provide_user_data(&mut self) -> Vec<User>;

    fn provide_user_id_list(&mut self) -> Vec<Uuid>;

    fn provide_user(&mut self, user_id: &Uuid) -> Option<User>;

    fn store_user(&mut self, user: User);

    fn update_user(&mut self, user: User);
}

pub trait RoomDataInterface {
    fn provide_room_data(&mut self) -> Vec<Room>;

    fn provide_room(&mut self, room_id: &Uuid) -> Option<Room>;

    fn store_room(&mut self, room: Room);

    fn update_room(&mut self, room: Room);
}
