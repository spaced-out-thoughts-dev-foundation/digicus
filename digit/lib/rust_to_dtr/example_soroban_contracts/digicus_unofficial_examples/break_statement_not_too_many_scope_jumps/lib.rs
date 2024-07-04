fn simple_break_statement() {
    for i in 0..10 {
        if i == 5 {
            println!("We got a five!");
            break;
        }

        println!("i: {}", i);
    }

    println!("Done!");
}
