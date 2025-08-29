use std::{
    fs::File,
    io::{BufReader, Read},
};

fn read_nnue_file(path: &str) -> std::io::Result<()> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut header = [0u8; 32];
    reader.read_exact(&mut header)?;
    println!("Header: {:?}", header);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::read_nnue_file;

    #[test]
    fn test_read() {
        read_nnue_file("assets/nnue-probe/nn-04cf2b4ed1da.nnue").unwrap();
    }
}
