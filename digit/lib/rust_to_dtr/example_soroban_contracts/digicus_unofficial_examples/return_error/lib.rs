pub enum SomeErrorTypes {
    NotTheAnswerToLife = 1,
}

fn answer_to_life(some_number: i32) -> Result<i32, String> {
    if some_number != 42 {
        return Err(SomeErrorTypes::NotTheAnswerToLife);
    }

    Ok(some_number)
}
