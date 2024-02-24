// 22-07-2024
// created struct : World and Room
// The world is filled with multiple Room objects
// Added function to initialize room and initialize world
// Added functions to generate gold cords, monster cords and pit cords
// Initialized gold, player, monster and pit co-ords in the world


use rand :: Rng; // 0.8.5

// ----------------------------   Room and the functions related to it  ------------------------------------------//

#[derive(Clone, Copy)]
    struct Room {
        has_gold: bool,
        has_monster: bool,
        has_player: bool,
        has_pit: bool,

        has_stench: bool,
        has_breeze: bool,
        has_glitter: bool,
    }


fn default_room() -> Room{
    Room {
        has_gold :false,
        has_monster:false,
        has_player :false,
        has_pit :false,

        has_stench :false,
        has_breeze:false,
        has_glitter:false,
    }
}

// ---------------------------------------  End of Room and it's functions  --------------------------------------------------------------//


// ---------------------------------------  World and it's functions  --------------------------------------------------------------//

struct World{
    grid: [[Room;5] ;5],
    gold_cords: Vec<i8>,
    monster_cords: Vec<i8>,
    pit_cords: Vec<Vec<i8>>,
}

fn initialize_world() -> World{

let gold_cords = generate_gold_cords();
let monster_cords = generate_monster_cords(&gold_cords);
let pit_cords = generate_pit_cords(&gold_cords, &monster_cords);

    let mut main_world = World{
        grid: [[default_room(); 5] ;5],
        gold_cords: gold_cords.clone(),
        monster_cords: monster_cords.clone(),
        pit_cords : pit_cords.clone(),
    };

    // Adding player to the world
    main_world.grid[0][0].has_player = true;

    //Adding gold to the world
    main_world.grid[gold_cords[0] as usize][gold_cords[1] as usize].has_gold = true;

    //Adding monster to the world
    main_world.grid[monster_cords[0] as usize][monster_cords[1] as usize].has_monster = true;

    //Adding pits to the world
    for i in 0..3 {
        main_world.grid[pit_cords[i][0] as usize][pit_cords[i][1] as usize].has_pit = true;
    }

    main_world
}

fn generate_gold_cords() -> Vec<i8>{
    let mut rng = rand::thread_rng();
    let mut cords = Vec::new();

    let mut x: i8 = rng.gen_range(0..5);
    let mut y: i8 = rng.gen_range(0..5);

    while x != 0 && y!= 0 {
        x = rng.gen_range(0..5);
        y = rng.gen_range(0..5);
    }

    cords.push(x);
    cords.push(y);

cords

}

fn generate_monster_cords(gold_cords: &[i8]) -> Vec<i8>{
    let mut rng = rand::thread_rng();
    let mut cords = Vec::new();

    let mut x: i8 = rng.gen_range(0..5);
    let mut y: i8 = rng.gen_range(0..5);

    while x != 0 && y!= 0 && x != gold_cords[0] && y != gold_cords[1]{
        x = rng.gen_range(0..5);
        y = rng.gen_range(0..5);
    }

    cords.push(x);
    cords.push(y);

cords

}

fn generate_pit_cords(gold: &[i8], monster: &[i8]) -> Vec<Vec<i8>> {
    let mut rng = rand::thread_rng();
    let mut pits: Vec<Vec<i8>> = Vec::new();

    while pits.len() < 4 {
        let x: i8 = rng.gen_range(0..5);
        let y: i8 = rng.gen_range(0..5);

        if [gold, monster].iter().all(|&c| c != &[x, y]) {
            pits.push(vec![x, y]);
        }
    }

    pits
}

// ---------------------------------------  End of World and it's functions  --------------------------------------------------------------//


fn main() {
    let world = initialize_world();
    for i in 0 .. 5{
        for j in 0..5{
        println!("{}",world.grid[i][j].has_gold);
        }
    }
}
