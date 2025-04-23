struct User {
    username: String,
    is_active: bool,
}

fn get_active_user<'a>(users: &'a Vec<User>, username: &str) -> Result<&'a User, String> {
    let mut users_iter = users.iter();

    users_iter
        .find(|user| user.username == username)
        .ok_or("User not found".to_string())
        .and_then(|user| {
            if user.is_active {
                Ok(user)
            } else {
                Err("User not active".to_string())
            }
        })
}

fn main() {
    let users = vec![
        User { username: "Test123".to_string(), is_active: true},
        User { username: "fffaaaddd".to_string(), is_active: false},
    ];

    match get_active_user(&users, "Test123") {
        Ok(user) => println!("Found active user: {}", user.username),
        Err(e) => println!("Error: {}", e),
    }

    match get_active_user(&users, "fffaaaddd") {
        Ok(user) => println!("Found active user: {}", user.username),
        Err(e) => println!("Error: {}", e),
    }

    match get_active_user(&users, "sssssssssss") {
        Ok(user) => println!("Found active user: {}", user.username),
        Err(e) => println!("Error: {}", e),
    }
}