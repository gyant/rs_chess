use crate::location::{BoardLocation, LocationCoords, LocationState};
use crate::piece::{Piece, PieceType};
use crate::player::{Color, Player};
use crate::utils::gcd;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Game {
    board: Vec<Vec<BoardLocation>>,
    pub player1: Rc<Player>,
    pub player2: Rc<Player>,
    pub current_player: Rc<Player>,
}

impl Game {
    pub fn new(player1: Rc<Player>, player2: Rc<Player>) -> Game {
        let mut board: Vec<Vec<BoardLocation>> = Vec::with_capacity(8);

        Player::populate_pieces(&player1);
        Player::populate_pieces(&player2);

        // Reverse board for player 2 board location population
        let mut player2_board = player2.pieces.borrow_mut();
        player2_board.reverse();

        // Puts queen and king in correct spots
        player2_board.swap(3, 4);

        // Get read-only ref to player1_board
        let player1_board = player1.pieces.borrow_mut();

        for i in 0..8 {
            board.push(Vec::with_capacity(8));

            for j in 0..8 {
                let coords = Rc::new(LocationCoords { x: j, y: i });

                if i < 2 {
                    let mut piece_location = player2_board[i * 8 + j].location.borrow_mut();
                    *piece_location = Some(Rc::clone(&coords));

                    let board_location = BoardLocation {
                        coords,
                        state: LocationState::Occupied,
                        piece: Some(Rc::clone(&player2_board[i * 8 + j])),
                        white_attackable: false,
                        black_attackable: false,
                    };

                    board[i].push(board_location);
                } else if (2..6).contains(&i) {
                    board[i].push(BoardLocation {
                        coords,
                        state: LocationState::Empty,
                        piece: None,
                        white_attackable: false,
                        black_attackable: false,
                    });
                } else {
                    let mut piece_location = player1_board[(i - 6) * 8 + j].location.borrow_mut();
                    *piece_location = Some(Rc::clone(&coords));

                    board[i].push(BoardLocation {
                        coords,
                        state: LocationState::Occupied,
                        piece: Some(Rc::clone(&player1_board[(i - 6) * 8 + j])),
                        white_attackable: false,
                        black_attackable: false,
                    });
                }
            }
        }

        // Re-reverse for consistency
        player2_board.swap(3, 4);
        player2_board.reverse();
        drop(player2_board);
        drop(player1_board);

        let current_player = Rc::clone(&player1);

        Game {
            board,
            player1,
            player2,
            current_player,
        }
    }

    fn get_loc_cartesian(&self, location: &LocationCoords) -> &BoardLocation {
        &self.board[location.y][location.x]
    }

    pub fn move_piece(&mut self, source: LocationCoords, dest: LocationCoords) {
        let mut successful_move = false;
        let mut piece_clone: Option<Rc<Piece>> = None;

        // Check bounds
        if dest.x > 7 || dest.y > 7 {
            println!("Destination out of bounds");
            return;
        }

        match self.board[source.y][source.x].state {
            LocationState::Occupied => {
                if let Some(piece) = &self.board[source.y][source.x].piece {
                    println!("FOUND THE PIECE");
                    println!("{:?}", piece);

                    // Verify ownership
                    if piece.owner.name != self.current_player.name {
                        println!("YOU DO NOT OWN THIS PIECE");
                        return;
                    }

                    let move_vec: (i32, i32) = get_move_vector(&source, &dest);

                    // Check intermediate collisions
                    match piece.piece_type {
                        PieceType::Knight => (),
                        _ => {
                            // Collisions apply to all other pieces
                            let intermediate_coords = points_along_vector(&source, &move_vec);

                            for coord in intermediate_coords {
                                if let LocationState::Occupied = self.board[coord.y][coord.x].state
                                {
                                    println!("COLLISION DETECTED. NOT VALID MOVE");
                                    return;
                                }
                            }
                        }
                    }

                    match self.get_loc_cartesian(&dest).state {
                        LocationState::Occupied => {
                            // Validate piece attack
                            if !piece.validate_attack(&move_vec) {
                                println!("NOT A VALID ATTACK FOR {:?}", piece);
                            }

                            // Check for friendly fire / path open
                            if let Some(o) = &self.get_loc_cartesian(&dest).piece {
                                if o.owner.name == piece.owner.name {
                                    println!("FRIENDLY FIRE!");
                                    return;
                                } else {
                                    // Reconcile attack / move
                                    let id = o.id;

                                    let mut owner_board = o.owner.pieces.borrow_mut();
                                    let mut dead_board = o.owner.dead_pieces.borrow_mut();
                                    let mut destroyed: i32 = -1;
                                    let mut found_destroyed: bool = false;

                                    for (index, piece) in owner_board.iter().enumerate() {
                                        if id == piece.id {
                                            println!("FOUND ATTACKED PIECE: {:?}", piece);
                                            found_destroyed = true;
                                            destroyed = index as i32;
                                        }
                                    }

                                    if found_destroyed {
                                        let mut dead_piece_location = o.location.borrow_mut();
                                        *dead_piece_location = None;

                                        dead_board
                                            .push(owner_board.swap_remove(destroyed as usize));
                                    } else {
                                        return;
                                    }
                                }
                            }
                        }
                        _ => {
                            // Validate piece move
                            if !piece.validate_move(&move_vec) {
                                println!("NOT A VALID MOVE FOR {:?}", piece);
                                return;
                            }
                        }
                    }

                    // Mark success
                    successful_move = true;
                    piece_clone = Some(Rc::clone(piece));
                }
            }
            _ => {
                println!("EMPTY NOTHING TO DO");
                return;
            }
        }

        if successful_move {
            let dest_loc = &mut self.board[dest.y][dest.x];

            // Set has_moved to true if this is the first time a piece has moved.
            if let Some(p) = piece_clone.clone() {
                let mut has_moved = p.has_moved.borrow_mut();
                if !*has_moved {
                    *has_moved = true;
                }

                let mut piece_location = p.location.borrow_mut();
                *piece_location = Some(Rc::clone(&dest_loc.coords));
            }

            // Set dest loc to moved piece.
            dest_loc.piece = piece_clone;
            dest_loc.state = LocationState::Occupied;

            // Set source board location's piece to None.
            let source_loc = &mut self.board[source.y][source.x];
            source_loc.piece = None;
            source_loc.state = LocationState::Empty;

            self.switch_turns();
        }
    }

    fn switch_turns(&mut self) {
        if self.current_player.id == self.player1.id {
            self.current_player = Rc::clone(&self.player2);
        } else {
            self.current_player = Rc::clone(&self.player1);
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut text = "".to_string();

        for (index, row) in self.board.iter().enumerate() {
            for column in row {
                match column.state {
                    LocationState::Empty => text.push_str(" __ "),
                    _ => {
                        if let Some(p) = &column.piece {
                            text.push(' ');
                            text.push(p.owner.piece_char);
                            match p.piece_type {
                                PieceType::Pawn => {
                                    text.push('P');
                                }
                                PieceType::Rook => {
                                    text.push('R');
                                }
                                PieceType::Knight => {
                                    text.push('N');
                                }
                                PieceType::Bishop => {
                                    text.push('B');
                                }
                                PieceType::Queen => {
                                    text.push('Q');
                                }
                                PieceType::King => {
                                    text.push('K');
                                }
                            }
                            text.push(' ');
                        }
                    }
                };
            }

            if index < self.board.len() - 1 {
                text.push('\n');
            }
        }

        write!(f, "{}", text)
    }
}

fn get_move_vector(source: &LocationCoords, dest: &LocationCoords) -> (i32, i32) {
    (
        dest.x as i32 - source.x as i32,
        dest.y as i32 - source.y as i32,
    )
}

pub fn points_along_vector(source: &LocationCoords, move_vec: &(i32, i32)) -> Vec<LocationCoords> {
    let gcd = gcd(
        move_vec.0.abs().try_into().unwrap(),
        move_vec.1.abs().try_into().unwrap(),
    );

    let step_x: i32 = move_vec.0 / gcd as i32;
    let step_y: i32 = move_vec.1 / gcd as i32;

    let mut points: Vec<LocationCoords> = vec![];

    // Ensure we're getting the locations in between source and dest exclusively.
    for k in 1..gcd {
        let x = source.x as i32 + k as i32 * step_x;
        let y = source.y as i32 + k as i32 * step_y;
        points.push(LocationCoords {
            x: x as usize,
            y: y as usize,
        });
    }

    points
}
