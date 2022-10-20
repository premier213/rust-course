pub fn slice_type() {
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..3];
    println!("{:?}", slice);

    let mut colors = ["red", "blue", "green", "black"];
    update_colors(&mut colors[2..=3]);
    println!("{:?}", colors)
}

fn update_colors(colors: &mut [&str]) {
    println!("{:?}", colors);
    colors[0] = "yellow";
    colors[1] = "orange";
}
