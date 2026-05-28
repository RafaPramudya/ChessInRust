use std::fmt::{self, Write};

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn char(&self) -> char {
        match self {
            PieceType::Pawn     => 'p',
            PieceType::Knight   => 'n',
            PieceType::Bishop   => 'b',
            PieceType::Rook     => 'r',
            PieceType::Queen    => 'q',
            PieceType::King     => 'k',
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PieceColor {
    White = 0,
    Black = 8,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    piece: u8
}

impl Piece {
    pub fn new(piece_type: PieceType, color: PieceColor) -> Self {
        Self{ piece: piece_type as u8 + color as u8 }
    }

    pub fn get_type(&self) -> Option<PieceType> {
        let piece_type = self.piece % 8;
        match piece_type {
            0 => Some(PieceType::Pawn),
            1 => Some(PieceType::Knight),
            2 => Some(PieceType::Bishop),
            3 => Some(PieceType::Rook),
            4 => Some(PieceType::Queen),
            5 => Some(PieceType::King),
            _ => None
        }
    }

    pub fn get_color(&self) -> PieceColor {
        if self.piece >= 8  { PieceColor::Black }
        else                { PieceColor::White }
    } 
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let notation = self.
            get_type()
            .unwrap()
            .char();

        let notation = if self.get_color() == PieceColor::Black {
            notation.to_ascii_uppercase()
        } else { notation };

        write!(f, "{}", notation)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Cell(Piece)
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Cell(x) => write!(f, "{}", x)
        }
    }
}

#[derive(Debug)]
pub struct Board {
    cells: [Cell; 64],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        output.reserve(256);
        
        for i in 0..64 {
            let cell = self.cells[i];
            if i % 8 == 0 && i != 0 {
                output.push('\n');
            }

            write!(&mut output, "{} ", cell).unwrap();
        }

        write!(f, "{}", output)
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [Cell::Empty; 64]
        }
    }

    pub fn new()
}