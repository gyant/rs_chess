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
                        x: i as u8,
                        y: j as u8,
                        state: LocationState::Occupied,
                        piece: Some(Rc::clone(&player1.pieces.borrow()[i * 7 + 1])),
                    })
                } else if i >= 2 && i < 6 {
                    board[i].push(BoardLocation {
                        x: i as u8,
                        y: j as u8,
                        state: LocationState::Empty,
                        piece: None,
                    });
                } else {
                    board[i].push(BoardLocation {
                        x: i as u8,
                        y: j as u8,
                        state: LocationState::Occupied,
                        piece: Some(Rc::clone(&player2.pieces.borrow()[(i - 6) * 7 + 1])),
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

    fn run(&mut self) {}

    fn move_piece(&mut self, source: (usize, usize), dest: (usize, usize)) {
        let mut successful_move = false;

        match self.board[source.0][source.1].state {
            LocationState::Occupied => {
                if let Some(piece) = &self.board[source.0][source.1].piece {
                    println!("FOUND THE PIECE");
                    println!("{:?}", piece);
                    match piece.piece_type {
                        PieceType::Pawn => {
                            // Verify ownership
                            if piece.owner.name != self.current_player.name {
                                println!("YOU DO NOT OWN THIS PIECE");
                                return;
                            }
                            // Validate piece move

                            // Reconcile attack / move
                            println!("MOVE THE PAWN");
                            successful_move = true;
                        }
                        _ => {
                            println!("NOT YET IMPLEMENTED");
                        }
                    }
                }
            }
            _ => {
                println!("EMPTY NOTHING TO DO");
            }
        }

        if successful_move {
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
                    _ => text.push_str("X"),
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
    x: u8,
    y: u8,
    state: LocationState,
    piece: Option<Rc<Piece>>,
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
}

impl Player {
    fn new(name: &str, color: Color) -> Self {
        let pieces: Vec<Rc<Piece>> = vec![];
        let dead_pieces: Vec<Rc<Piece>> = vec![];

        let player = Player {
            name: name.to_string(),
            pieces: RefCell::new(pieces),
            dead_pieces: RefCell::new(dead_pieces),
            color,
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

    game.move_piece((6, 1), (5, 1));
    game.move_piece((1, 1), (2, 1));
    game.move_piece((6, 1), (5, 1));
    game.move_piece((6, 1), (5, 1));
    game.move_piece((1, 1), (2, 1));
}
