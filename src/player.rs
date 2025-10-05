use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::piece::{Piece, PieceType};

#[derive(Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub id: Uuid,
    pub pieces: RefCell<Vec<Rc<Piece>>>,
    pub dead_pieces: RefCell<Vec<Rc<Piece>>>,
    pub color: Color,
    pub pawn_direction: i32,
    pub piece_char: char,
    pub king: RefCell<Option<Rc<Piece>>>,
}

impl Player {
    pub fn new(name: &str, color: Color) -> Self {
        let pieces: Vec<Rc<Piece>> = vec![];
        let dead_pieces: Vec<Rc<Piece>> = vec![];
        let pawn_direction: i32;
        let piece_char: char;

        match color {
            Color::White => {
                pawn_direction = -1;
                piece_char = 'O';
            }
            Color::Black => {
                pawn_direction = 1;
                piece_char = 'X';
            }
        }

        Player {
            name: name.to_string(),
            id: Uuid::new_v4(),
            pieces: RefCell::new(pieces),
            dead_pieces: RefCell::new(dead_pieces),
            color,
            pawn_direction,
            piece_char,
            king: RefCell::new(None),
        }
    }

    pub fn with_rc(name: &str, color: Color) -> Rc<Self> {
        let player = Self::new(name, color);

        Rc::new(player)
    }

    pub fn populate_pieces(player: &Rc<Self>) {
        let mut pieces = player.pieces.borrow_mut();
        for i in 0..16 {
            if i < 8 {
                pieces.push(Rc::new(Piece {
                    piece_type: PieceType::Pawn,
                    owner: Rc::clone(player),
                    id: Uuid::new_v4(),
                    has_moved: RefCell::new(false),
                    location: RefCell::new(None),
                }));
            } else if i == 8 || i == 15 {
                pieces.push(Rc::new(Piece {
                    piece_type: PieceType::Rook,
                    owner: Rc::clone(player),
                    id: Uuid::new_v4(),
                    has_moved: RefCell::new(false),
                    location: RefCell::new(None),
                }));
            } else if i == 9 || i == 14 {
                pieces.push(Rc::new(Piece {
                    piece_type: PieceType::Knight,
                    owner: Rc::clone(player),
                    id: Uuid::new_v4(),
                    has_moved: RefCell::new(false),
                    location: RefCell::new(None),
                }));
            } else if i == 10 || i == 13 {
                pieces.push(Rc::new(Piece {
                    piece_type: PieceType::Bishop,
                    owner: Rc::clone(player),
                    id: Uuid::new_v4(),
                    has_moved: RefCell::new(false),
                    location: RefCell::new(None),
                }));
            } else if i == 11 {
                pieces.push(Rc::new(Piece {
                    piece_type: PieceType::Queen,
                    owner: Rc::clone(player),
                    id: Uuid::new_v4(),
                    has_moved: RefCell::new(false),
                    location: RefCell::new(None),
                }));
            } else if i == 12 {
                let king = Rc::new(Piece {
                    piece_type: PieceType::King,
                    owner: Rc::clone(player),
                    id: Uuid::new_v4(),
                    has_moved: RefCell::new(false),
                    location: RefCell::new(None),
                });

                // Populate king field in player now that it exists.
                let mut player_king_ref = player.king.borrow_mut();
                *player_king_ref = Some(Rc::clone(&king));

                // Add king to pieces vec
                pieces.push(king);
            }
        }
    }
}
