fn return_some_or_none(x: i32) -> Option<i32> {
    let spend_left: Option<i128> = if let Some(spend_left) = spend_left_per_token {
        Some(spend_left)
    } else if let Some(limit_left) = env.storage() {
        Some(limit_left)
    } else {
        None
    };

    spend_left
}
