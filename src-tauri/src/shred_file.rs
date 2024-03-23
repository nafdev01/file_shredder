use tokio::{
    fs,
    io::{AsyncSeekExt, AsyncWriteExt},
};
use tokio::fs::OpenOptions;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use crate::initialize_app::CustomError;

pub async fn shred_file(path: &String) -> Result<String, CustomError> {
    // open the file for reading and overwriting
    let mut file = match OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .open(path)
        .await
    {
        Ok(file) => file,
        Err(e) => return Err(CustomError::ShreddingError(e.to_string())),
    };

    // create a buffer to hold the data
    let size = match file.metadata().await {
        Ok(metadata) => metadata.len(),
        Err(e) => return Err(CustomError::ShreddingError(e.to_string())),
    };
    let mut buffer = vec![0; size.min(512) as usize];

    // create a random number generator
    let mut rng = ChaChaRng::from_entropy();

    // loop through the passes
    for _ in 0..5 {
        // seek to the beginning of the file
        if let Err(e) = file.seek(std::io::SeekFrom::Start(0)).await {
            return Err(CustomError::ShreddingError(e.to_string()));
        }

        // overwrite the file with random data, 512 bytes at a time until the file is empty
        let mut offset = 0;
        while offset < size {
            // fill the buffer with random data
            rng.fill_bytes(&mut buffer);

            // write the buffer to the file
            if let Err(e) = file.write_all(&buffer).await {
                return Err(CustomError::ShreddingError(e.to_string()));
            }

            // increment the offset
            offset += buffer.len() as u64;
        }

        // flush the file
        if let Err(e) = file.flush().await {
            return Err(CustomError::ShreddingError(e.to_string()));
        }
    }

    // delete the file
    if let Err(e) = fs::remove_file(path).await {
        return Err(CustomError::ShreddingError(e.to_string()));
    }

    Ok("success".to_string())
}
