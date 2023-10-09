use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::{Context, Result};
use pgp::{crypto::sym::SymmetricKeyAlgorithm, types::StringToKey, Message};

pub fn gpg_filename(filename: &str) -> String {
    format!("{}.gpg", filename)
}

// GPG Symmetric Encryption with Password
// Please use the following command to decrypt the file
// gpg -o backup.tar.gz -d backup.tar.gz.gpg
pub fn gpg_encrypt(input: &str, output: &str, password: &str) -> Result<()> {
    // 1. read source file to buffer
    let mut in_f = File::open(input).context("open source file to encrypt")?;
    let mut in_buf = Vec::new();
    in_f.read_to_end(&mut in_buf)
        .context("read source file to buffer to encrypt")?;

    // 2. encryt with password
    let mut rng = rand::thread_rng();
    let s2k = StringToKey::new_default(&mut rng);
    let msg = Message::new_literal_bytes(input, &in_buf)
        .compress(pgp::types::CompressionAlgorithm::ZLIB)?;
    let encrypted = msg
        .encrypt_with_password(&mut rng, s2k, SymmetricKeyAlgorithm::AES128, || {
            password.into()
        })
        .context("encrypt with password")?;

    // 3. export the encrypted file
    let armored = encrypted
        .to_armored_bytes(None)
        .context("convert encrypted to armored bytes")?;
    let mut out_f = File::create(output).context("create encrypted output file")?;
    out_f
        .write_all(&armored)
        .context("write encrypted output file")?;

    // 4. return
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use anyhow::Result;
    use pgp::Deserializable;
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn it_encrypts_the_file_with_password() -> Result<()> {
        // 0. create the temp directory
        let tmp_dir = tempdir().context("create the tmp dir")?;
        let input = tmp_dir.path().join("input.txt").display().to_string();
        let output = tmp_dir.path().join("output.txt.gpg").display().to_string();

        // 1. create to_be_encrypted_file
        let mut f = File::create(input.clone())?;
        f.write_all(b"hello world")?;

        // 2. encrypt the file
        gpg_encrypt(&input, &output, "passcode").context("encrypt the file")?;

        // 3. decrypt the file and verify the result
        let mut out_f = File::open(output.clone())?;
        let mut buf = Vec::new();
        out_f.read_to_end(&mut buf)?;
        let msg = Message::from_armor_single(Cursor::new(&buf)).unwrap().0;
        let decrypted = msg
            .decrypt_with_password(|| "passcode".to_string())
            .unwrap()
            .next()
            .unwrap()
            .unwrap();

        let decrypted_bytes = decrypted.get_content()?.unwrap();
        assert_eq!(decrypted_bytes, b"hello world");

        // 4. delete the temp directory
        tmp_dir.close().context("delete the tmp dir")?;

        // return
        Ok(())
    }
}
