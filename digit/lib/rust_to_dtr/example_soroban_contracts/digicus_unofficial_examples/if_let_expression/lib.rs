fn if_let_example(letter: Option<char>) {
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    if let Ok(foobar) = ok_foobar {
        return foobar;
    }

    panic!("This is a panic!");
}
