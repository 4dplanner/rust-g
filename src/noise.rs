use noise::{Perlin};
use std::io::{BufWriter, Write};
use std::fs::File;
use std::io;


byond_fn! {perlin_noise_2d(filename, seed) {
    make_noise_file(seed).err()
} }


//returns filename
fn make_noise_file(filename: &str, seed_as_str: &str) -> io::Result<()> {
    let seed = seed_as_str.parse::<u32>()
        .map_err(|e|io::Error::new(io::ErrorKind::Other, "could not parse seed as string!"))?;
    let mut file = BufWriter::new(File::create(filename)?);
    let mut noise = Perlin::new().seed(seed);
    let ret = Vec::new();
    for y in 0..255{
        let row_string = (0u32..255u32).map(|x|noise.get([x as f64,y as f64]))
              .map(|noise|noise.to_string())
              .collect::<Vec<String>>()
              .join(",");

        write!(&mut file, "{}", row_string)?;
        file.write(b"\n");
    }
    file.flush()?;
    Ok(())
}
