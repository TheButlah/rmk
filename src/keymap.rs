use crate::action::Action;

use alloc::vec::Vec;
use snafu::{ensure, Snafu};

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
pub struct Position {
    row: u8,
    col: u8,
}

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct LayerId(pub u8);

/// Whether
pub enum Orientation {
    /// Consecutive keys/indices are along rows. Example: row-staggered and ortholinear
    RowMajor,
    /// Consecutive keys/indices are along columns. Example: column-staggered
    ColumnMajor,
}

#[derive(Debug, Snafu)]
pub enum Error {
    InvalidPosition { p: Position },
    InvalidLayer { l: LayerId },
}

pub struct Keymap {
    n_layers: u8,
    actions: Vec<Vec<Action>>, // outer, inner, layer (last two dims flattened)
    orientation: Orientation,
}
impl Keymap {
    pub fn n_layers(&self) -> u8 {
        self.n_layers
    }

    pub fn get(&self, pos: Position, layer: LayerId) -> Result<Action, Error> {
        ensure!(layer.0 < self.n_layers(), InvalidLayer { l: layer });
        todo!("Get the action")
    }
}
