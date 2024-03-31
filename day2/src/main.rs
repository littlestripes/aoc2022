// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢨⠳⢆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠂⠸⣆⡴⠛⣷⠀⢀⣤⠶⣶⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠙⠁⠀⠹⠟⠉⣠⠞⠁⠀⠀
// ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⢤⡀⠀⠀⡼⣦⠀⣠⣤⡀⠀⢀⣴⠋⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⣀⡤⣀⡀⠀⠀⢀⡞⣡⣶⡌⣇⠀⣿⠋⠀⠻⠾⠃⣴⣻⠇⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⢸⢡⣴⣶⡹⣄⣀⡤⣿⠛⠟⠃⠏⠠⢿⣤⣄⣀⠀⢠⡾⠁⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠈⠢⡽⠛⠇⠉⠀⠀⠀⠀⠀⠀⠐⠂⠠⡄⠉⠉⠓⠿⢄⡀⠀⠀⠀⠀⠀⠀
// ⠀⠀⠀⠀⣠⠖⢋⣁⣠⡤⠖⠒⠶⠦⣄⡀⠈⠄⢲⣶⣤⠥⠀⠀⠀⠀⠉⠳⣦⡀⠀⠀⠀
// ⠀⠀⢠⠎⠁⢀⡼⢛⣁⠀⠀⠀⡀⠀⠈⠹⣄⠐⠈⠫⠭⠠⠤⠤⠤⠄⠀⠐⢌⠙⣦⡀⠀
// ⠀⣰⡟⡂⢠⡟⠸⣿⣿⣷⠀⣾⣿⠆⠀⠀⢹⡀⠀⢀⣀⣠⠤⢴⡆⢈⠐⠀⠈⡇⠈⠻⣆
// ⣠⡿⢰⡓⢾⠀⠀⠈⠻⠋⠀⠀⠀⠀⠀⣀⡿⠓⠚⡏⠁⠀⢠⠟⢀⡠⠄⠀⢀⠀⠀⠀⢸
// ⡟⠀⣾⠀⢌⠓⠦⢶⠤⠤⢤⠤⠴⠚⢻⡁⠀⢀⣀⣅⣠⣴⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰
// ⡇⠀⣾⣶⣦⣤⣤⣦⣤⣤⣾⣶⣶⣶⣿⠿⠛⠋⠉⣥⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸
// ⣇⠀⢹⡙⠉⠉⠻⠋⠉⠙⡏⠁⠀⠀⢸⠀⢀⣤⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡜
// ⢹⡄⠀⠙⠦⢄⣀⣀⠀⢀⡇⠀⣀⣠⠼⠒⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡼⠁
// ⠀⠹⣦⡀⠀⠈⠁⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⠟⠁⠀
// ⠀⠀⠈⠛⠦⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⡤⠖⠉⠀⠀⠀⠀
// ⠀⠀⠀⠀⠀⠀⠉⠙⠂⠠⠤⣄⣀⣀⣀⣀⣀⣀⣀⣤⠤⠤⠛⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀

fn main() {
    // read in the input file
    let file_contents = include_str!("static/input").lines();

    // iterate over it
    let mut total_score = 0;
    for line in file_contents {
        // round 1, FIGHT

        // store the round's plays in a vec
        let plays: Vec<&str> = line.split(" ").collect();

        // direct indexing is fine here since we know there will only ever be 2 elements in our vec
        let opp_play = plays[0]; // a, b, or c
        let my_play = plays[1]; // x, y, or z

        // then map out each outcome
        let round_score = match my_play {
            // &str can't be matched exhaustively, so need a wildcard arm
            "X" => match opp_play {
                "A" => 4,
                "B" => 1,
                "C" => 7,
                &_ => 0
            },
            "Y" => match opp_play {
                "A" => 8,
                "B" => 5,
                "C" => 2,
                &_ => 0
            },
            "Z" => match opp_play {
                "A" => 3,
                "B" => 9,
                "C" => 6,
                &_ => 0
            },
            &_ => 0
        };

        // and add this round's score to the total
        total_score += round_score;
    }

    println!("{}", total_score);
}
