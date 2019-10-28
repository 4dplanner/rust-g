use noise::{Perlin, Seedable, NoiseFn};
use std::io::{BufWriter, Write};
use std::fs::File;

use crate::error::Result;

byond_fn! {perlin_noise_2d(filename, seed, scaling) {
    make_noise_file(filename, seed, scaling).err()
} }


//returns filename
fn make_noise_file(filename: &str, seed_as_str: &str, scaling_as_str: &str) -> Result<()> {
    let seed = seed_as_str.parse::<u32>()?;
    let scaling = scaling_as_str.parse::<f64>()?;
    let mut file = BufWriter::new(File::create(filename)?);
    let noise = Perlin::new().set_seed(seed);
    for y in 0..255{
        let row_string = (0u32..255u32).map(|x|noise.get([x as f64*scaling,y as f64*scaling]))
              .map(|noise|noise.to_string())
              .collect::<Vec<String>>()
              .join(",");

        write!(&mut file, "{}", row_string)?;
        file.write(b"\n")?;
    }
    file.flush()?;
    Ok(())
}
