use std::fmt::{self, Write};

#[derive(Debug)]
pub enum Error {
    InvalidFEN
}

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
    Black = 0,
    White = 8,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    piece: u8
}

impl Piece {
    pub fn new(piece_type: PieceType, color: PieceColor) -> Self {
        Self{ piece: piece_type as u8 + color as u8 }
    }

    pub fn new_from_char(piece: char) -> Option<Self> {
        match piece {
            'p' => Some(Self::new(PieceType::Pawn, PieceColor::Black)),
            'n' => Some(Self::new(PieceType::Knight, PieceColor::Black)),
            'b' => Some(Self::new(PieceType::Bishop, PieceColor::Black)),
            'r' => Some(Self::new(PieceType::Rook, PieceColor::Black)),
            'q' => Some(Self::new(PieceType::Queen, PieceColor::Black)),
            'k' => Some(Self::new(PieceType::King, PieceColor::Black)),
            'P' => Some(Self::new(PieceType::Pawn, PieceColor::White)),
            'N' => Some(Self::new(PieceType::Knight, PieceColor::White)),
            'B' => Some(Self::new(PieceType::Bishop, PieceColor::White)),
            'R' => Some(Self::new(PieceType::Rook, PieceColor::White)),
            'Q' => Some(Self::new(PieceType::Queen, PieceColor::White)),
            'K' => Some(Self::new(PieceType::King, PieceColor::White)),

            _ => None
        }
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
        if self.piece >= 8  { PieceColor::White }
        else                { PieceColor::Black }
    } 
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let notation = self.
            get_type()
            .unwrap()
            .char();

        let notation = if self.get_color() == PieceColor::White {
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
    pub fn empty() -> Self {
        Self {
            cells: [Cell::Empty; 64]
        }
    }

    pub fn new<T: AsRef<str>>(fen: T) -> Result<Self, Error> {
        let fen = fen.as_ref();
        let mut cells = Vec::new();
        cells.reserve(64);

        let fen = fen.chars();

        for char in fen {
            match char {
                '1'..='8' => {
                    let mut empty = vec![Cell::Empty; char as usize - '0' as usize];
                    cells.append(&mut empty);
                }
                '/' => {
                    if cells.len() % 8 != 0 { return Err(Error::InvalidFEN) }
                }
                'p' | 'n' | 'b' | 'r' | 'q' | 'k' |
                'P' | 'N' | 'B' | 'R' | 'Q' | 'K'
                => {cells.push(Cell::Cell(Piece::new_from_char(char).unwrap()));}
                ' ' => {}
                _ => {}
            }
        }

        Ok(Self {
            cells: cells.try_into().unwrap()
        })
    }
}