use std::{thread, time};


fn print_state(state: Vec<Vec<bool>>) {
    state.iter().for_each(|r| {
        r.iter().for_each(|c| {
            if *c {
                print!("{}", "#");
            } else {
                print!("{}", "_");
            }
        });
        print!("{}", "\n");
    })
}

fn main() {
    let w = 10;
    let h = 10;
    let one_second = time::Duration::from_millis(1000);

    for i in 0..h {
        for j in 0..w {
            let mut state = vec![vec![false; w]; h];
            state[i][j] = true;

            thread::sleep(one_second);
            print!("\x1B[2J\x1B[1;1H"); // clear the terminal and put the cursor at 1:1
            print_state(state);
        }
    }
}
