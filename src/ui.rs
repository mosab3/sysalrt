use std::hash::{Hasher, Hash};
use std::env;
use std::time::SystemTime;

fn get_os() -> &'static str {
    return env::consts::OS;
}

fn get_user() -> String{

    let env_name: String = match get_os() {
        "windows" => "UserName".to_string(),
        "linux" => "USER".to_string(),
        "macos" => "USER".to_string(),
        _ => panic!("Match is not supported.")
    };
    
       let user = match env::var(env_name) {
        Ok(r) => r,
        Err(e) => panic!("{}",e)
       };

    return String::from(user)

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

#[test]
fn test() {
    println!("{:?}", UiElements::values());
}