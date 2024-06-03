use crate::errors::not_translatable_error::NotTranslatableError;

pub fn map_name(rust_name: &str) -> Result<String, NotTranslatableError> {
    if rust_name.contains("Vec<") {
        return Ok(replace_vec_with_list(rust_name));
    }

    if rust_name.contains("HashMap<") {
        return Ok(replace_hashmap_with_map(rust_name));
    }

    if rust_name.contains("HashSet<") {
        return Ok(replace_hashset_with_set(rust_name));
    }

    if rust_name.contains("Arc<") {
        return Err(NotTranslatableError::Custom(
            "Unable to translate Arc type".to_string(),
        ));
    }

    if rust_name.contains("Box<") {
        return Err(NotTranslatableError::Custom(
            "Unable to translate Box type".to_string(),
        ));
    }

    if rust_name.contains("Cell<") {
        return Err(NotTranslatableError::Custom(
            "Unable to translate Cell type".to_string(),
        ));
    }

    if rust_name.contains("Mutex<") {
        return Err(NotTranslatableError::Custom(
            "Unable to translate Mutex type".to_string(),
        ));
    }

    if rust_name.contains("Option<") {
        return Err(NotTranslatableError::Custom(
            "Unable to translate Option type".to_string(),
        ));
    }

    if rust_name.contains("Ref<") {
        return Err(NotTranslatableError::Custom(
            "Unable to translate Ref type".to_string(),
        ));
    }

    if rust_name.contains("RefCell<") {
        return Err(NotTranslatableError::Custom(
            "Unable to translate RefCell type".to_string(),
        ));
    }

    // TODO: is this correct?
    if rust_name.contains("Result<") {
        return Ok(rust_name.to_string());
        // return Err(NotTranslatableError::Custom(
        //     "Unable to translate Result type for result".to_string(),
        // ));
    }

    match rust_name {
        "Vec" | "HashMap" | "HashSet" => Err(NotTranslatableError::Custom(
            "Unable to translate typeless collection".to_string(),
        )),
        "String" => Ok("String".to_string()),
        "Symbol" => Ok("String".to_string()),
        "bool" => Ok("Boolean".to_string()),
        "char" => Ok("Character".to_string()),
        "f32" | "f64" => Ok("Float".to_string()),
        "i8" | "i16" | "i32" | "i64 " => Ok(rust_name.to_string()),
        "u8" | "u16" | "u32" | "u64 " => Ok(rust_name.to_string()),
        "()" | "!" | "Arc" | "Box" | "Cell" | "isize" | "Mutex" | "Option" | "Ref" | "RefCell"
        | "Result" | "usize" => unable_to_translate_type_helper(rust_name),
        // ASSUMPTION: these are custom type names
        _ => Ok(rust_name.to_string()),
    }
}

fn unable_to_translate_type_helper(rust_name: &str) -> Result<String, NotTranslatableError> {
    Err(NotTranslatableError::Custom(format!(
        "Unable to translate {}",
        rust_name
    )))
}

fn replace_vec_with_list(input: &str) -> String {
    if input.starts_with("Vec<") && input.ends_with('>') {
        let inner_type = &input[4..input.len() - 1]; // Extract the type inside the angle brackets
        format!("List<{}>", inner_type.trim()) // Format the new string with 'List'
    } else {
        input.to_string() // If the pattern doesn't match, return the input as is
    }
}

fn replace_hashmap_with_map(input: &str) -> String {
    if input.starts_with("HashMap<") && input.ends_with('>') {
        let inner_types = &input[8..input.len() - 1]; // Extract the types inside the angle brackets
        let mut types = inner_types.split(","); // Split the types by comma
        let key_type = types.next().unwrap(); // Get the first type as the key type
        let value_type = types.next().unwrap(); // Get the second type as the value type
        format!("Map<{}, {}>", key_type.trim(), value_type.trim()) // Format the new string with 'Map'
    } else {
        input.to_string() // If the pattern doesn't match, return the input as is
    }
}

fn replace_hashset_with_set(input: &str) -> String {
    if input.starts_with("HashSet<") && input.ends_with('>') {
        let inner_type = &input[9..input.len() - 1]; // Extract the type inside the angle brackets
        format!("Set<{}>", inner_type.trim()) // Format the new string with 'Set'
    } else {
        input.to_string() // If the pattern doesn't match, return the input as is
    }
}
