mod chess;

fn main() -> Result<(), chess::Error> {
    let board = chess::Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")?;
    println!("{}", board);

    Ok(())
}