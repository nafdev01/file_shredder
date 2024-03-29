use std::{
    fs,
    io::{Seek, Write},
};

use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;

pub fn shred_file(path: &String) -> Result<String, Box<dyn std::error::Error>> {
    // open the file for reading and overwriting
    let mut file = match fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .open(path)
    {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };

    // create a buffer to hold the data
    let size = match file.metadata() {
        Ok(metadata) => metadata.len(),
        Err(e) => return Err(Box::new(e)),
    };
    let mut buffer = vec![0; size.min(512) as usize];

    // create a random number generator
    let mut rng = ChaChaRng::from_entropy();

    // loop through the passes
    for _ in 0..5 {
        // seek to the beginning of the file
        if let Err(e) = file.seek(std::io::SeekFrom::Start(0)) {
            return Err(Box::new(e));
        }

        // overwrite the file with random data, 512 bytes at a time until the file is empty
        let mut offset = 0;
        while offset < size {
            // fill the buffer with random data
            rng.fill_bytes(&mut buffer);

            // write the buffer to the file
            if let Err(e) = file.write_all(&buffer) {
                return Err(Box::new(e));
            }

            // increment the offset
            offset += buffer.len() as u64;
        }

        // flush the file
        if let Err(e) = file.flush() {
            return Err(Box::new(e));
        }
    }

    // delete the file
    if let Err(e) = fs::remove_file(path) {
        return Err(Box::new(e));
    }

    Ok("success".to_string())
}
