use std::hash::{Hasher, Hash};
use std::env;
use std::time::SystemTime;
use users::{get_user_by_uid, get_current_uid};

fn get_os() -> &'static str {
    return env::consts::OS;
}

// #[test]
fn get_user() -> String{
    let user = get_user_by_uid(get_current_uid()).unwrap();

    return user.name().to_string_lossy().to_string();
}

fn get_session_id() -> u64{

    let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(e) => e,
        Err(e) => panic!("{}", e)
    };

    let mut hasher = std::collections::hash_map::DefaultHasher::new();

    get_os().hash(&mut hasher);
    timestamp.hash(&mut hasher);

    return hasher.finish()


}

#[derive(Debug)]
pub struct UiElements {
    pub os: &'static str,
    pub session_id: u64,
    pub user: String

}

impl UiElements {
    pub fn values() -> UiElements {
        UiElements {os: get_os(), session_id: get_session_id(), user: get_user()}
    }
}

// pub fn get_ui_elements() -> Elements{

//     return Elements{
//         os: get_os(),
//         session_id: get_session_id()
//     };
// }

#[test]
fn test() {
    println!("{:?}", UiElements::values());
}