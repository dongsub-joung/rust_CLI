pub mod hashing{
    extern crate crypto;
    use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
    

    pub fn encrypt_string(key: &[u8], iv: &[u8], plaintext: &str) -> Result<Vec<u8>, crypto::symmetriccipher::SymmetricCipherError> {
        let mut encryptor = crypto::aes::cbc_encryptor(
            crypto::aes::KeySize::KeySize128,
            key,
            iv,
            crypto::blockmodes::PkcsPadding,
        );
    
        let mut final_result = Vec::<u8>::new();
        let mut read_buffer = crypto::buffer::RefReadBuffer::new(plaintext.as_bytes());
        let mut buffer = [0; 10000];
        let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);
    
        loop {
            let result = encryptor.encrypt(
                &mut read_buffer
                , &mut write_buffer
                , true
            )?;
    
            final_result.extend(
                write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter().copied()
            );
    
            match result {
                BufferResult::BufferUnderflow => break,
                BufferResult::BufferOverflow => {},
            }
        }
    
        Ok(final_result)
    }
}