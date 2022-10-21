pub fn for_type() {
    for i in 1..11 {
        println!("{0} * {0} = {1}", i, i * i);
    }

    let pets = ["dog", "cat", "rabbit", "dankey", "bear"];

    for pet in pets.iter() {
        if pet == &"bear" {
            println!("this is not pet: {}", pet);
            break;
        }

        if pet == &"dankey" {
            println!("are you crazy, your pet is {} ?", pet);
            continue;
        }

        println!("i love my {}", pet);
    }

    for (index, i) in (1..=10).enumerate() {
        println!("{},{}", index, i)
    }
}
