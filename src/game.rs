use crate::location::{BoardLocation, LocationCoords, LocationState};
use crate::piece::{Piece, PieceType};
use crate::player::{Color, Player};
use crate::utils::gcd;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Game {
    pub board: Vec<Vec<BoardLocation>>,
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
                            let intermediate_coords = points_along_vector(
                                &source,
                                &move_vec,
                                GatherPointsMode::Exclusive,
                            );

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

    // Generates white_attackable / black_attackable fields for check / victory condition checks.
    pub fn generate_attack_map(&mut self, player: &Player) {
        for piece in player.pieces.borrow().iter() {
            match piece.piece_type {
                PieceType::Pawn => {
                    let attack_unit_vecs: Vec<(i32, i32)> = vec![
                        (1, 1 * player.pawn_direction),
                        (-1, 1 * player.pawn_direction),
                    ];
                    let attack_vecs: Vec<(i32, i32)> =
                        self.get_attack_vecs_in_bounds(&piece, attack_unit_vecs);

                    self.set_attack_flags(&piece, &attack_vecs);
                }
                PieceType::Rook => {
                    let attack_unit_vecs: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
                    let attack_vecs: Vec<(i32, i32)> =
                        self.get_attack_vecs_to_edge(&piece, attack_unit_vecs);
                    self.set_attack_flags(&piece, &attack_vecs);
                }
                PieceType::Knight => {
                    let attack_unit_vecs: Vec<(i32, i32)> = vec![
                        (2, 1),
                        (2, -1),
                        (-2, 1),
                        (-2, -1),
                        (1, 2),
                        (1, -2),
                        (-1, 2),
                        (-1, -2),
                    ];
                    let attack_vecs: Vec<(i32, i32)> =
                        self.get_attack_vecs_in_bounds(&piece, attack_unit_vecs);
                    self.set_attack_flags(&piece, &attack_vecs);
                }
                PieceType::Bishop => {
                    let attack_unit_vecs: Vec<(i32, i32)> =
                        vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
                    let attack_vecs: Vec<(i32, i32)> =
                        self.get_attack_vecs_to_edge(&piece, attack_unit_vecs);
                    self.set_attack_flags(&piece, &attack_vecs);
                }
                PieceType::Queen => {
                    let attack_unit_vecs: Vec<(i32, i32)> = vec![
                        (1, 1),
                        (1, -1),
                        (-1, 1),
                        (-1, -1),
                        (1, 0),
                        (-1, 0),
                        (0, 1),
                        (0, -1),
                    ];
                    let attack_vecs: Vec<(i32, i32)> =
                        self.get_attack_vecs_to_edge(&piece, attack_unit_vecs);
                    self.set_attack_flags(&piece, &attack_vecs);
                }
                PieceType::King => {
                    let attack_unit_vecs: Vec<(i32, i32)> = vec![
                        (1, 1),
                        (1, -1),
                        (-1, 1),
                        (-1, -1),
                        (0, 1),
                        (0, -1),
                        (1, 0),
                        (-1, 0),
                    ];
                    let attack_vecs = self.get_attack_vecs_in_bounds(&piece, attack_unit_vecs);
                    self.set_attack_flags(&piece, &attack_vecs);
                }
            }
        }
    }

    // Used for pawn / knight / king attack map population
    fn get_attack_vecs_in_bounds(
        &self,
        piece: &Piece,
        attack_vecs: Vec<(i32, i32)>,
    ) -> Vec<(i32, i32)> {
        let mut vecs_inbounds: Vec<(i32, i32)> = vec![];

        if let Some(location) = piece.location.borrow().as_ref() {
            for attack_vec in attack_vecs {
                let dx = attack_vec.0;
                let dy = attack_vec.1;

                if dx == 0 && dy == 0 {
                    continue;
                }

                let temp_vec = (location.x as i32 + dx, location.y as i32 + dy);

                if temp_vec.0 >= 0 && temp_vec.0 <= 7 && temp_vec.1 >= 0 && temp_vec.1 <= 7 {
                    vecs_inbounds.push(attack_vec);
                }
            }
        }

        vecs_inbounds
    }

    // Used for Rook / Bishop / Queen attack map population
    fn get_attack_vecs_to_edge(
        &self,
        piece: &Piece,
        attack_vecs: Vec<(i32, i32)>,
    ) -> Vec<(i32, i32)> {
        let mut vecs_to_edge: Vec<(i32, i32)> = vec![];

        if let Some(location) = piece.location.borrow().as_ref() {
            for attack_vec in attack_vecs {
                let dx = attack_vec.0;
                let dy = attack_vec.1;

                if dx == 0 && dy == 0 {
                    continue;
                }

                let mut curr_x = location.x as i32 + dx;
                let mut curr_y = location.y as i32 + dy;
                let mut last_valid: Option<(i32, i32)> = None;

                while curr_x >= 0 && curr_x <= 7 && curr_y >= 0 && curr_y <= 7 {
                    last_valid = Some((curr_x, curr_y));
                    curr_x += dx;
                    curr_y += dy;
                }

                if let Some(point_of_edge) = last_valid {
                    vecs_to_edge.push((
                        point_of_edge.0 - location.x as i32,
                        point_of_edge.1 - location.y as i32,
                    ));
                }
            }
        }

        vecs_to_edge
    }

    fn set_attack_flags(&mut self, piece: &Piece, attack_vecs: &Vec<(i32, i32)>) {
        if let Some(source) = piece.location.borrow().as_ref() {
            for attack in attack_vecs {
                let attack_points = match piece.piece_type {
                    // Knight is special due to mad hops.
                    PieceType::Knight => {
                        vec![LocationCoords {
                            x: <i32 as TryInto<usize>>::try_into(source.x as i32 + attack.0)
                                .unwrap(),
                            y: <i32 as TryInto<usize>>::try_into(source.y as i32 + attack.1)
                                .unwrap(),
                        }]
                    }
                    _ => points_along_vector(&source, &attack, GatherPointsMode::Inclusive),
                };

                for point in attack_points {
                    let location = &mut self.board[point.y][point.x];
                    let mut attackable: bool = false;

                    match location.state {
                        LocationState::Empty => {
                            attackable = true;
                        }
                        LocationState::Occupied => {
                            // if friendly do nothing. If enemy populate attack map.
                            if let Some(other_piece) = &location.piece {
                                if other_piece.owner.color != piece.owner.color {
                                    attackable = true;
                                } else {
                                    // Friendly collision detected. Exit loop.
                                    break;
                                }
                            }
                        }
                    }

                    if attackable {
                        match piece.owner.color {
                            Color::White => {
                                location.white_attackable = true;
                            }
                            Color::Black => {
                                location.black_attackable = true;
                            }
                        }
                    }
                }
            }
        }
    }

    fn clear_attack_map(&mut self, player: &Player) {
        for row in &mut self.board {
            for column in row {
                match player.color {
                    Color::White => {
                        column.white_attackable = false;
                    }
                    Color::Black => {
                        column.black_attackable = false;
                    }
                }
            }
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

// Mode set for regular moves vs attack map (check / victory conditions)
pub enum GatherPointsMode {
    Exclusive,
    Inclusive,
}

pub fn points_along_vector(
    source: &LocationCoords,
    move_vec: &(i32, i32),
    mode: GatherPointsMode,
) -> Vec<LocationCoords> {
    let gcd = gcd(
        move_vec.0.abs().try_into().unwrap(),
        move_vec.1.abs().try_into().unwrap(),
    );

    let step_x: i32 = move_vec.0 / gcd as i32;
    let step_y: i32 = move_vec.1 / gcd as i32;

    let mut points: Vec<LocationCoords> = vec![];

    // Game already checks if target spot contains enemy.
    //
    // Exclusive mode returns all points between source and target to check for collisions on path.
    // Inclusive mode includes target for attack_map population.
    let mode_operation: usize = match mode {
        GatherPointsMode::Exclusive => gcd,
        GatherPointsMode::Inclusive => gcd + 1,
    };

    // Ensure we're getting the locations in between source and dest exclusively.
    for k in 1..mode_operation {
        let x = source.x as i32 + k as i32 * step_x;
        let y = source.y as i32 + k as i32 * step_y;

        // Ensure any point returned is in the bounds of the board
        if (x >= 0 && x <= 7) && (y >= 0 && y <= 7) {
            points.push(LocationCoords {
                x: x as usize,
                y: y as usize,
            });
        }
    }

    points
}
