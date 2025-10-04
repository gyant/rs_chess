use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use uuid::Uuid;

use crate::utils::vectors_same_direction;
use crate::Player;

#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub owner: Rc<Player>,
    pub id: Uuid,
    pub has_moved: RefCell<bool>,
}

impl Piece {
    pub fn validate_attack(&self, attack_vec: &(i32, i32)) -> bool {
        match self.piece_type {
            PieceType::Pawn => {
                println!("DO PAWN ATTACK");
                let valid_attack: (i32, i32) = (1, 1);

                // Validate vector matches attack vector in any direction.
                if (attack_vec.0.abs(), attack_vec.1.abs()) != valid_attack {
                    return false;
                }

                // Validate vector matches direction of owner's pawn direction.
                if *attack_vec
                    != (
                        valid_attack.0 * self.owner.pawn_direction,
                        valid_attack.1 * self.owner.pawn_direction,
                    )
                {
                    return false;
                }

                true
            }
            // Every other piece has attack patterns that match move patterns.
            _ => self.validate_move(attack_vec),
        }
    }

    pub fn validate_move(&self, move_vec: &(i32, i32)) -> bool {
        // Check piece capabilities
        match self.piece_type {
            PieceType::Pawn => {
                println!("DO PAWN MOVE");

                if !*self.has_moved.borrow() {
                    if !(move_vec.1.abs() >= 1 && move_vec.1.abs() <= 2) {
                        return false;
                    }

                    if *move_vec != (0, move_vec.1.abs() * self.owner.pawn_direction) {
                        return false;
                    }
                } else {
                    // Validate length of vector matches pawn capabilities
                    if move_vec.1.abs() != 1 {
                        return false;
                    }

                    // Validate the pawn unit vector matches direction of player (pawns can't move
                    // backwards)
                    if *move_vec != (0, self.owner.pawn_direction) {
                        return false;
                    }
                }

                true
            }
            PieceType::Rook => {
                println!("DO ROOK MOVE");

                let valid_vecs: Vec<(i32, i32)> = vec![(0, 1), (1, 0)];
                let mut valid_move: bool = false;

                for valid in valid_vecs {
                    if vectors_same_direction(&valid, &(move_vec.0.abs(), move_vec.1.abs())) {
                        valid_move = true;
                        break;
                    }
                }

                if !valid_move {
                    return false;
                }

                true
            }
            PieceType::Knight => {
                println!("DO KNIGHT MOVE");

                let valid_vecs: Vec<(i32, i32)> = vec![(2, 1), (1, 2)];

                let mut valid_move: bool = false;

                for v in valid_vecs {
                    if (move_vec.0.abs(), move_vec.1.abs()) == v {
                        valid_move = true;
                        break;
                    }
                }

                if !valid_move {
                    return false;
                }

                true
            }
            PieceType::Bishop => {
                println!("DO BISHOP MOVE");

                let valid_vecs: Vec<(i32, i32)> = vec![(1, 1)];

                let mut valid_move: bool = false;

                for valid in valid_vecs {
                    if vectors_same_direction(&valid, &(move_vec.0.abs(), move_vec.1.abs())) {
                        valid_move = true;
                        break;
                    }
                }

                if !valid_move {
                    return false;
                }

                true
            }
            PieceType::Queen => {
                println!("DO QUEEN MOVE");

                let valid_vecs: Vec<(i32, i32)> = vec![(1, 1), (1, 0), (0, 1)];

                let mut valid_move: bool = false;

                for valid in valid_vecs {
                    if vectors_same_direction(&valid, &(move_vec.0.abs(), move_vec.1.abs())) {
                        valid_move = true;
                        break;
                    }
                }

                if !valid_move {
                    return false;
                }

                true
            }
            PieceType::King => {
                println!("DO KING MOVE");

                let valid_vecs: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (1, 1)];

                let mut valid_move: bool = false;

                for v in valid_vecs {
                    if (move_vec.0.abs(), move_vec.1.abs()) == v {
                        valid_move = true;
                        break;
                    }
                }

                if !valid_move {
                    return false;
                }

                true
            }
        }
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Piece")
            .field("piece_type", &self.piece_type)
            .field("owner", &self.owner.name)
            .field("id", &self.id)
            .field("has_moved", &self.has_moved.borrow())
            .finish()
    }
}
