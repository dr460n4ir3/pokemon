// create a pokemon class
#[derive(Debug)]
struct Pokemon {
    name: String,
    level: u8,
    hp: u8,
    attack: u8,
    defense: u8,
    speed: u8,
    moves: Vec<Move>,
}

// create a move class
#[derive(Debug)]
struct Move {
    name: String,
    power: u8,
    accuracy: u8,
    move_type: String,
}

// create a player class
#[derive(Debug)]
struct Player {
    name: String,
    pokemon: Vec<Pokemon>,
}

// create a battle class
#[derive(Debug)]
struct Battle {
    player: Player,
    opponent: Pokemon,
}

// create a battle state
#[derive(Debug)]
struct BattleState {
    battle: Battle,
}

// create a turn-based battle system
fn battle_system(
    mut commands: Commands,
    mut state: ResMut<State<BattleState>>,
    mut battle: ResMut<BattleState>,
) {
    // check if the battle is over
    if battle.battle.player.pokemon[0].hp == 0 {
        println!("You lost!");
        state.set(BattleState {
            battle: Battle {
                player: Player {
                    name: "Player".to_string(),
                    pokemon: vec![Pokemon {
                        name: "Pikachu".to_string(),
                        level: 5,
                        hp: 0,
                        attack: 55,
                        defense: 40,
                        speed: 90,
                        moves: vec![Move {
                            name: "Thunder Shock".to_string(),
                            power: 40,
                            accuracy: 100,
                            move_type: "Electric".to_string(),
                        }],
                    }],
                },
                opponent: Pokemon {
                    name: "Bulbasaur".to_string(),
                    level: 5,
                    hp: 45,
                    attack: 49,
                    defense: 49,
                    speed: 45,
                    moves: vec![Move {
                        name: "Tackle".to_string(),
                        power: 40,
                        accuracy: 100,
                        move_type: "Normal".to_string(),
                    }],
                },
            },
        });
    } else if battle.battle.opponent.hp == 0 {
        println!("You won!");
        state.set(BattleState {
            battle: Battle {
                player: Player {
                    name: "Player".to_string(),
                    pokemon: vec![Pokemon {
                        name: "Pikachu".to_string(),
                        level: 5,
                        hp: 35,
                        attack: 55,
                        defense: 40,
                        speed: 90,
                        moves: vec![Move {
                            name: "Thunder Shock".to_string(),
                            power: 40,
                            accuracy: 100,
                            move_type: "Electric".to_string(),
                        }],
                    }],
                },
                opponent: Pokemon {
                    name: "Bulbasaur".to_string(),
                    level: 5,
                    hp: 45,
                    attack: 49,
                    defense: 49,
                    speed: 45,
                    moves: vec![Move {
                        name: "Tackle".to_string(),
                        power: 40,
                        accuracy: 100,
                        move_type: "Normal".
[...]                   

//   // check if the player's pokemon is faster than the opponent's pokemon
if battle.battle.player.pokemon[0].speed > battle.battle.opponent.speed { 
    // if the player's pokemon is faster, the player's pokemon attacks first
    println!("{} used {}!", battle.battle.player.pokemon[0].name, battle.battle.player.pokemon[0].moves[0].name);
    battle.battle.opponent.hp -= battle.battle.player.pokemon[0].moves[0].power;
    println!("{}'s HP: {}", battle.battle.opponent.name, battle.battle.opponent.hp);
    println!("{} used {}!", battle.battle.opponent.name, battle.battle.opponent.moves[0].name);
    battle.battle.player.pokemon[0].hp -= battle.battle.opponent.moves[0].power;
    println!("{}'s HP: {}", battle.battle.player.pokemon[0].name, battle.battle.player.pokemon[0].hp);
} else {
    // if the opponent's pokemon is faster, the opponent's pokemon attacks first
    println!("{} used {}!", battle.battle.opponent.name, battle.battle.opponent.moves[0].name);
    battle.battle.player.pokemon[0].hp -= battle.battle.opponent.moves[0].power;
    println!("{}'s HP: {}", battle.battle.player.pokemon[0].name, battle.battle.player.pokemon[0].hp);
    println!("{} used {}!", battle.battle.player.pokemon[0].name, battle.battle.player.pokemon[0].moves[0].name);
    battle.battle.opponent.hp -= battle.battle.player.pokemon[0].moves[0].power;
    println!("{}'s HP: {}", battle.battle.opponent.name, battle.battle.opponent.hp);
}
}