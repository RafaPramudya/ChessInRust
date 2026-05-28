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

#[derive(Debug)]
pub enum Square {
    Invalid,
    Square(i8)
}

impl Square {
    pub fn new(file: i8, rank: i8) -> Self {
        let index = rank * 8 + file;
        if index > 63 || index < 0  { Self::Invalid }
        else                        { Self::Square(index) }
    }

    pub fn from(file: char, rank: i8) -> Self {
        let file = file as i8 - 'a' as i8;

        Self::new(file, rank)
    }

    pub fn from_notation(expr: &str) -> Self {
        let mut expr = expr.chars();

        let file = expr.next().unwrap() as i8 - 'a' as i8;
        let rank = expr.next().unwrap() as i8 - '0' as i8;

        Self::new(file, rank)
    }

    pub fn get_file(&self) -> Option<i8> {
        match self {
            Square::Invalid => None,
            Square::Square(x) => Some(x % 8)
        }
    }

    pub fn get_rank(&self) -> Option<i8> {
        match self {
            Square::Invalid => None,
            Square::Square(x) => Some(x / 8)
        }
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
pub struct Castle {
    white_king: bool,
    white_queen: bool,
    black_king: bool,
    black_queen: bool,
}

impl Castle {
    pub fn all_false() -> Self {
        Self {
            white_king: false,
            white_queen: false,
            black_king: false,
            black_queen: false,
        }
    }

    pub fn all_true() -> Self {
        Self {
            white_king: true,
            white_queen: true,
            black_king: true,
            black_queen: true,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    cells: [Cell; 64],
    white_to_play: bool,
    castle_available: Castle,
    enpassant: Square,

    halfmove: u8,
    fullmove: u8
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        output.reserve(256);
        
        for rank in (0..8).rev() {
            for file in 0..8 {
                let index = rank * 8 + file;
                let cell = self.cells[index];

                write!(&mut output, "{} ", cell).unwrap();
            }
            output.push('\n');
        }

        output.push('\n');
        writeln!(&mut output, "WhiteToPlay: {}", self.white_to_play).unwrap();
        writeln!(&mut output, "Caslte: {:?}", self.castle_available).unwrap();
        writeln!(&mut output, "EnPassant: {:?}", self.enpassant).unwrap();
        writeln!(&mut output, "HalfMove: {}", self.halfmove).unwrap();
        writeln!(&mut output, "FullMove: {}", self.fullmove).unwrap();

        write!(f, "{}", output)
    }
}

impl Board {
    pub fn empty() -> Self {
        Self {
            cells: [Cell::Empty; 64],
            castle_available: Castle::all_false(),
            white_to_play: true,
            enpassant: Square::Invalid,
            halfmove: 0,
            fullmove: 0
        }
    }

    fn parse_pieces(token: &str) -> Result<[Cell; 64], Error> {
        let mut cells = Vec::new();
        // let token = token.chars();
        let ranks = token.split('/').into_iter();
        for rank in ranks.rev() {
            let rank = rank.chars();

            for char in rank {
                match char {
                    '1'..='8' => {
                        let mut empty = vec![Cell::Empty; char as usize - '0' as usize];
                        cells.append(&mut empty);
                    }
                    'p' | 'n' | 'b' | 'r' | 'q' | 'k' |
                    'P' | 'N' | 'B' | 'R' | 'Q' | 'K'
                    => {cells.push(Cell::Cell(Piece::new_from_char(char).unwrap()));}
                    _ => {return Err(Error::InvalidFEN)}
                }
            }
        }

        // println!("{}", cells.len());
        Ok(cells.try_into().expect("Cell dalam fen seharusnya berjumlah 64"))
    }

    fn parse_white_to_play(token: &str) -> Option<bool> {
        match token {
            "w" => Some(true),
            "b" => Some(false),
            _ => None
        }
    }

    fn parse_castle(token: &str) -> Result<Castle, Error> {
        let mut castle = Castle::all_false();

        let token = token.chars();
        for char in token {
            match char {
                'k' => castle.black_king = true,
                'q' => castle.black_queen = true,
                'K' => castle.white_king = true,
                'Q' => castle.white_queen = true,
                '-' => {}
                _ => return Err(Error::InvalidFEN)
            }
        }

        Ok(castle)
    }

    fn parse_enpassant(token: &str) -> Square {
        if token == "-" { return Square::Invalid }
        Square::from_notation(token)
    }

    pub fn new<T: AsRef<str>>(fen: T) -> Result<Self, Error> {
        let mut fen = fen.as_ref().split_whitespace();

        let cells = Self::parse_pieces(fen.next().unwrap())?;
        let white_to_play = Self::parse_white_to_play(fen.next().unwrap()).unwrap();
        let castle = Self::parse_castle(fen.next().unwrap())?;
        let enpassant = Self::parse_enpassant(fen.next().unwrap());
        let halfmove: u8 = fen.next().unwrap().trim().parse().unwrap();
        let fullmove: u8 = fen.next().unwrap().trim().parse().unwrap();

        Ok(Self{
            cells,
            white_to_play,
            castle_available: castle,
            enpassant,
            halfmove,
            fullmove
        })
    }
}