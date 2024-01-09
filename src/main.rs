use std::{thread, time};


fn print_state(state: &Vec<Vec<bool>>) {
    state.iter().for_each(|r| {
        r.iter().for_each(|c| {
            if *c {
                print!("{}", "▮");
            } else {
                print!("{}", " ");
            }
        });
        print!("{}", "\n");
    })
}

fn count_alive_neighbors(i: i32, j: i32, w: i32, h: i32, state: &Vec<Vec<bool>>, neighbor_map: &Vec<(i32, i32)>) -> i32 {
    let mut alive_neighbors: i32 = 0;
    neighbor_map.iter().for_each(|nm| {
        let ni: i32 = i+nm.0;
        let nj: i32 = j+nm.1;
        if ni<h && ni>=0 && nj<w && nj>=0 && state[ni as usize][nj as usize] {
            alive_neighbors += 1;
        }
    });
    alive_neighbors
}

fn update_state(state: &Vec<Vec<bool>>, neighbor_map: &Vec<(i32, i32)>,  w: usize, h: usize) -> Vec<Vec<bool>> {
    let mut alive = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            let alive_neighbors = count_alive_neighbors(i as i32, j as i32, w as i32, h as i32, state, neighbor_map);
            if alive_neighbors == 3 || (alive_neighbors == 2 && state[i][j]) {
                alive[i][j] = true;
            }
        }
    }
    alive
}

fn main() {
    let w: usize = 100;
    let h: usize = 50;
    let frame_pause = time::Duration::from_millis(100);
    let neighbor_map: Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let mut state = vec![vec![false; w]; h];
    // Gosper glider gun
    state[5][1] = true; // left cube
    state[5][2] = true;
    state[6][1] = true;
    state[6][2] = true;

    state[3][35] = true; // right cube
    state[3][36] = true;
    state[4][35] = true;
    state[4][36] = true;

    state[3][13] = true; // left ship shape
    state[3][14] = true;
    state[4][12] = true;
    state[4][16] = true;
    state[5][11] = true;
    state[5][17] = true;
    state[6][11] = true;
    state[6][15] = true;
    state[6][17] = true;
    state[6][18] = true;
    state[7][11] = true;
    state[7][17] = true;
    state[8][12] = true;
    state[8][16] = true;    
    state[9][13] = true;
    state[9][14] = true;

    state[1][25] = true; // right ship shape
    state[2][23] = true;
    state[2][25] = true;
    state[3][21] = true;
    state[3][22] = true;
    state[4][21] = true;
    state[4][22] = true;
    state[5][21] = true;
    state[5][22] = true;
    state[6][23] = true;
    state[6][25] = true;
    state[7][25] = true;

    loop {
        thread::sleep(frame_pause);
        print!("\x1B[2J\x1B[1;1H"); // clear the terminal and put the cursor at 1:1
        print_state(&state);
        state = update_state(&state, &neighbor_map, w, h);
    }
}
