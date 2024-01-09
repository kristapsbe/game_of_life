fn main() {
    let state = [[0u8; 4]; 6];

    println!("Hello, world!");

    state.iter().for_each(|r| {
        r.iter().for_each(|c| {
            print!("{:#?}", c);
        });
        print!("{}", "\n");
    })
}
