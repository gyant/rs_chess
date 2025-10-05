use std::rc::Rc;

use crate::piece::Piece;

#[derive(Debug)]
pub struct BoardLocation {
    pub coords: Rc<LocationCoords>,
    pub state: LocationState,
    pub piece: Option<Rc<Piece>>,
    pub white_attackable: bool,
    pub black_attackable: bool,
}

#[derive(Debug)]
pub struct LocationCoords {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub enum LocationState {
    Empty,
    Occupied,
}
