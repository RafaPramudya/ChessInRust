mod chess;

fn main() -> Result<(), chess::Error> {
    let board = chess::Board::new("8/8/8/4p1K1/2k1P3/8/8/8 b - - 0 1")?;
    println!("{}", board);

    Ok(())
}