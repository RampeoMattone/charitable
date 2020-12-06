use crate::database::{User, Server, Database};


fn main() {
    let mut database = Database::connect("mysql://anoni:anoni@192.168.1.4:3306/anoni");
    let users = vec!(
        User::new(1, "nilone"),
        User::new(2, "tipone"),
        User::new(3, "slasho"),
        User::new(4, "giovon"),
    );
    database.user_batch_sync(users);
    database.server_sync(Server::new(1,1,1));
    database.server_sync(Server::new(2,2,2));
    database.server_sync(Server::new(3,3,3));
    database.server_sync(Server::new(4,4,4));
    database.server_available_sync(2, false);
    //database.user_server_state_sync(1,5, true);
    //database.user_server_state_sync(2,5, true);
    //database.user_server_state_sync(3,5, (false));
    database.msg_in_add(1,2, "ciao!".to_string());
    database.msg_out_sync(1,1, 5, 1, None, false);
    database.user_last_msg(2);
    let active:Vec<(u64, u64)> = vec!((3,3), (3,2), (2,3));
    println!("{:?}", database.user_server_state_batch_sync(active));
    println!("{:?}", database.user_server_state_get(3));
}
