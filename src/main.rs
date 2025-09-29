use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
struct Game {
    board: Vec<Vec<BoardLocation>>,
    player1: Rc<Player>,
    player2: Rc<Player>,
    current_player: Rc<Player>,
}

impl Game {
    fn new(player1: Rc<Player>, player2: Rc<Player>) -> Game {
        let mut board: Vec<Vec<BoardLocation>> = Vec::with_capacity(8);

        Player::populate_pieces(&player1);
        Player::populate_pieces(&player2);

        for i in 0..8 {
            board.push(Vec::with_capacity(8));

            for j in 0..8 {
                if i < 2 {
                    board[i].push(BoardLocation {
                        coords: LocationCoords { x: j, y: i },
                        state: LocationState::Occupied,
                        piece: Some(Rc::clone(&player2.pieces.borrow()[i * 7 + 1])),
                    })
                } else if i >= 2 && i < 6 {
                    board[i].push(BoardLocation {
                        coords: LocationCoords { x: j, y: i },
                        state: LocationState::Empty,
                        piece: None,
                    });
                } else {
                    board[i].push(BoardLocation {
                        coords: LocationCoords { x: j, y: i },
                        state: LocationState::Occupied,
                        piece: Some(Rc::clone(&player1.pieces.borrow()[(i - 6) * 7 + 1])),
                    })
                }
            }
        }

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
    //fn run(&mut self) {}

    fn move_piece(&mut self, source: LocationCoords, dest: LocationCoords) {
        let mut successful_move = false;
        let mut piece_clone: Option<Rc<Piece>> = None;

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

                    match self.get_loc_cartesian(&dest).state {
                        LocationState::Occupied => {
                            if let Some(o) = &self.get_loc_cartesian(&dest).piece {
                                if o.owner.name == piece.owner.name {
                                    println!("FRIENDLY FIRE!");
                                    return;
                                }
                            } else {
                                // Reconcile attack / move
                            }
                        }
                        _ => {
                            // Validate piece move
                            if !piece.validate_move(&source, &dest) {
                                println!("NOT A VALID MOVE FOR {:?}", piece);
                                return;
                            }
                        }
                    }

                    // Mark success
                    successful_move = true;
                    piece_clone = Some(Rc::clone(&piece));
                }
            }
            _ => {
                println!("EMPTY NOTHING TO DO");
                return;
            }
        }

        if successful_move {
            {
                let source_board = &mut self.board[source.y][source.x];

                source_board.piece = None;
                source_board.state = LocationState::Empty;
            }

            {
                let dest_board = &mut self.board[dest.y][dest.x];

                dest_board.piece = piece_clone;
                dest_board.state = LocationState::Occupied;
            }

            self.switch_turns();
        }
    }

    fn switch_turns(&mut self) {
        if self.current_player.name == self.player1.name {
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
                    LocationState::Empty => text.push_str("_"),
                    _ => {
                        if let Some(p) = &column.piece {
                            text.push(p.owner.piece_char);
                        }
                    }
                };
            }

            if index < self.board.len() - 1 {
                text.push_str("\n");
            }
        }

        write!(f, "{}", text)
    }
}

#[derive(Debug)]
struct BoardLocation {
    coords: LocationCoords,
    state: LocationState,
    piece: Option<Rc<Piece>>,
}

#[derive(Debug)]
struct LocationCoords {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum LocationState {
    Empty,
    Occupied,
}

#[derive(Debug)]
struct Player {
    name: String,
    pieces: RefCell<Vec<Rc<Piece>>>,
    dead_pieces: RefCell<Vec<Rc<Piece>>>,
    color: Color,
    pawn_direction: i64,
    piece_char: char,
}

impl Player {
    fn new(name: &str, color: Color) -> Self {
        let pieces: Vec<Rc<Piece>> = vec![];
        let dead_pieces: Vec<Rc<Piece>> = vec![];
        let pawn_direction: i64;
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

        let player = Player {
            name: name.to_string(),
            pieces: RefCell::new(pieces),
            dead_pieces: RefCell::new(dead_pieces),
            color,
            pawn_direction,
            piece_char,
        };

        player
    }

    fn with_rc(name: &str, color: Color) -> Rc<Self> {
        let player = Self::new(name, color);

        Rc::new(player)
    }

    fn populate_pieces(player: &Rc<Self>) {
        let mut pieces = player.pieces.borrow_mut();
        for _ in 0..16 {
            pieces.push(Rc::new(Piece {
                piece_type: PieceType::Pawn,
                owner: Rc::clone(player),
            }));
        }
    }
}

struct Piece {
    piece_type: PieceType,
    owner: Rc<Player>,
}

impl Piece {
    fn validate_move(&self, source: &LocationCoords, dest: &LocationCoords) -> bool {
        match self.piece_type {
            PieceType::Pawn => {
                println!("DO PAWN MOVE");
                // Get movement vector between both points
                let move_vec: (i64, i64) = (
                    dest.x as i64 - source.x as i64,
                    dest.y as i64 - source.y as i64,
                );

                // Validate length of vector matches pawn capabilities
                if move_vec.1.abs() != 1 {
                    return false;
                }

                // Validate the pawn unit vector matches direction of player (pawns can't move
                // backwards)
                if move_vec != (0, 1 * self.owner.pawn_direction) {
                    return false;
                }

                true
            }
            PieceType::Rook => {
                println!("DO ROOK MOVE");
                false
            }
            PieceType::Knight => {
                println!("DO KNIGHT MOVE");
                false
            }
            PieceType::Bishop => {
                println!("DO BISHOP MOVE");
                false
            }
            PieceType::Queen => {
                println!("DO QUEEN MOVE");
                false
            }
            PieceType::King => {
                println!("DO KING MOVE");
                false
            }
        }
    }
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Piece")
            .field("piece_type", &self.piece_type)
            .field("owner", &self.owner.name)
            .finish()
    }
}

#[derive(Debug)]
enum Color {
    Black,
    White,
}

#[derive(Debug)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

fn main() {
    let player1 = Player::with_rc("bob", Color::White);
    let player2 = Player::with_rc("alice", Color::Black);

    let mut game = Game::new(player1, player2);
    println!("{:#?}", &game);

    println!("{}", &game);

    game.move_piece(LocationCoords { x: 7, y: 6 }, LocationCoords { x: 7, y: 5 });
    game.move_piece(LocationCoords { x: 0, y: 1 }, LocationCoords { x: 0, y: 2 });

    game.move_piece(LocationCoords { x: 7, y: 5 }, LocationCoords { x: 7, y: 6 });
    game.move_piece(LocationCoords { x: 7, y: 5 }, LocationCoords { x: 7, y: 4 });

    println!("{}", &game);
}
