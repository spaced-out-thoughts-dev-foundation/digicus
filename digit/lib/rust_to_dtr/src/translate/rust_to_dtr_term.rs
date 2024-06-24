use crate::errors::not_translatable_error::NotTranslatableError;

pub fn map_name(rust_name: &str) -> Result<String, NotTranslatableError> {
    if rust_name.contains("BytesN<32>") {
        return Ok("ByteStringSmall".to_string());
    }

    if rust_name.contains("BytesN<64>") {
        return Ok("ByteStringLarge".to_string());
    }

    if rust_name.contains("Vec<") {
        return Ok(replace_vec_with_list(rust_name));
    }

    if rust_name.contains("HashMap<") {
        return Ok(replace_hashmap_with_map(rust_name));
    }

    if rust_name.contains("Map<") {
        return Ok(replace_map_with_map(rust_name));
    }

    if rust_name.contains("HashSet<") {
        return Err(NotTranslatableError::Custom(
            "Unable to translate HashSet type".to_string(),
        ));
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

    // ASSUMPTION: we can return a tuple type here
    if rust_name.contains("Result<") {
        return Ok(replace_result(rust_name));
    }

    match rust_name {
        "Vec" | "HashMap" | "HashSet" => Err(NotTranslatableError::Custom(
            "Unable to translate typeless collection".to_string(),
        )),
        "String" => Ok("String".to_string()),
        "Symbol" => Ok("String".to_string()),
        "Bytes" => Ok("String".to_string()),
        "bool" => Ok("Boolean".to_string()),
        "f32" | "f64" => Ok("Float".to_string()),
        // ASSUMPTION: if we aren't returning anything then don't return anything ¯\_(ツ)_/¯
        "()" => Ok("".to_string()),
        "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" => Ok("Integer".to_string()),
        "u128" | "u256" | "i128" | "i256" => Ok("BigInteger".to_string()),
        "char" | "!" | "Arc" | "Box" | "Cell" | "isize" | "Mutex" | "Option" | "Ref"
        | "RefCell" | "Result" | "usize" => unable_to_translate_type_helper(rust_name),
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
        format!("Dictionary<{}, {}>", key_type.trim(), value_type.trim()) // Format the new string with 'Map'
    } else {
        input.to_string() // If the pattern doesn't match, return the input as is
    }
}

fn replace_map_with_map(input: &str) -> String {
    if input.starts_with("Map<") && input.ends_with('>') {
        let inner_types = &input[4..input.len() - 1]; // Extract the types inside the angle brackets
        let mut types = inner_types.split(","); // Split the types by comma
        let key_type = types.next().unwrap(); // Get the first type as the key type
        let value_type = types.next().unwrap(); // Get the second type as the value type
        format!("Dictionary<{}, {}>", key_type.trim(), value_type.trim()) // Format the new string with 'Map'
    } else {
        input.to_string() // If the pattern doesn't match, return the input as is
    }
}

fn replace_result(input: &str) -> String {
    if input.starts_with("Result<") && input.ends_with('>') {
        let inner_types = &input[7..input.len() - 1]; // Extract the types inside the angle brackets
        let types: Vec<String> = inner_types
            .split(",")
            .map(|s| s.trim().to_string())
            .collect(); // Split the types by comma and trim them

        if types.len() == 0 {
            return "".to_string();
        }

        if types.len() == 1 {
            return types[0].to_string();
        }

        format!("Result<{}>", types.join(", "))
    } else {
        input.to_string() // If the pattern doesn't match, return the input as is
    }
}
