pub fn main() {
    println!(
        "{}",
        include_str!("static/input")
        // bring in the text file as a string
            .split("\n\n")
            // split the literal into an iterator
            .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
            // parse each line of each elf's data slice and sum it
            .max()
            // find the maximum
            .unwrap(),
            // return the Ok value (the sum as a u32)
    );
}
