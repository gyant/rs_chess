mod game;
mod location;
mod piece;
mod player;
mod utils;

use game::Game;
use location::LocationCoords;
use player::{Color, Player};

fn main() {
    let player1 = Player::with_rc("bob", Color::White);
    let player2 = Player::with_rc("alice", Color::Black);

    let mut game = Game::new(player1, player2);

    println!("{}", &game);

    game.move_piece(LocationCoords { x: 7, y: 6 }, LocationCoords { x: 7, y: 5 });
    game.move_piece(LocationCoords { x: 6, y: 1 }, LocationCoords { x: 6, y: 2 });

    game.move_piece(LocationCoords { x: 7, y: 5 }, LocationCoords { x: 7, y: 4 });
    game.move_piece(LocationCoords { x: 6, y: 2 }, LocationCoords { x: 6, y: 3 });

    println!("{}", &game);

    game.move_piece(LocationCoords { x: 7, y: 4 }, LocationCoords { x: 6, y: 3 });

    println!("{}", &game);

    println!(
        "Player {:?}\nActive pieces: {:?}\nDead pieces: {:?}",
        &game.player2.name, &game.player2.pieces, &game.player2.dead_pieces
    );

    let pieces = game.player2.pieces.borrow();
    let dead = game.player2.dead_pieces.borrow();

    println!(
        "Alive count: {}\nDead count: {}",
        &pieces.len(),
        &dead.len()
    );

    drop(pieces);
    drop(dead);

    game.move_piece(LocationCoords { x: 6, y: 0 }, LocationCoords { x: 5, y: 2 });
    println!("{}", &game);

    game.move_piece(LocationCoords { x: 4, y: 6 }, LocationCoords { x: 4, y: 5 });
    game.move_piece(LocationCoords { x: 5, y: 2 }, LocationCoords { x: 3, y: 3 });
    game.move_piece(LocationCoords { x: 4, y: 7 }, LocationCoords { x: 4, y: 6 });
    println!("{}", &game);

    game.move_piece(LocationCoords { x: 1, y: 1 }, LocationCoords { x: 1, y: 2 });
    game.move_piece(LocationCoords { x: 7, y: 7 }, LocationCoords { x: 7, y: 1 });
    game.move_piece(LocationCoords { x: 7, y: 0 }, LocationCoords { x: 7, y: 1 });

    // Move pawn to make room for bishop test
    game.move_piece(LocationCoords { x: 6, y: 6 }, LocationCoords { x: 6, y: 5 });

    // Throwaway move for turn
    game.move_piece(LocationCoords { x: 7, y: 1 }, LocationCoords { x: 7, y: 2 });

    // Bishop test
    game.move_piece(LocationCoords { x: 5, y: 7 }, LocationCoords { x: 7, y: 5 });

    // throwaway
    game.move_piece(LocationCoords { x: 7, y: 2 }, LocationCoords { x: 5, y: 2 });

    // Bishop reverse test
    game.move_piece(LocationCoords { x: 7, y: 5 }, LocationCoords { x: 5, y: 7 });

    // Pawn double step test
    game.move_piece(LocationCoords { x: 0, y: 1 }, LocationCoords { x: 0, y: 3 });

    // Invalid double move for pawn
    game.move_piece(LocationCoords { x: 4, y: 5 }, LocationCoords { x: 4, y: 3 });
    // Single move to validate after failed move
    game.move_piece(LocationCoords { x: 4, y: 5 }, LocationCoords { x: 4, y: 4 });

    // Test failed collision
    game.move_piece(LocationCoords { x: 0, y: 0 }, LocationCoords { x: 0, y: 6 });

    println!("{}", &game);
}
