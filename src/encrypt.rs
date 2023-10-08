use anyhow::{Context, Result};
use pgp::{crypto::sym::SymmetricKeyAlgorithm, types::StringToKey, Message};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

pub async fn gpg_encrypt(input: &str, output: &str) -> Result<()> {
    // 1. read source file to buffer
    let mut f = File::open(input)
        .await
        .context("open source file to encrypt")?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)
        .await
        .context("read source file to buffer to encrypt")?;

    // 2. encryt with password
    let mut rng = rand::thread_rng();
    let s2k = StringToKey::new_default(&mut rng);
    let msg =
        Message::new_literal_bytes(input, &buf).compress(pgp::types::CompressionAlgorithm::ZLIB)?;
    let encrypted = msg
        .encrypt_with_password(&mut rng, s2k, SymmetricKeyAlgorithm::AES128, || {
            "password".to_string()
        })
        .context("encrypt with password")?;

    // 3. export the encrypted file
    let armored = encrypted
        .to_armored_bytes(None)
        .context("convert encrypted to armored bytes")?;
    let mut out_f = File::create(output)
        .await
        .context("create encrypted output file")?;
    out_f
        .write_all(&armored)
        .await
        .context("write encrypted output file")?;

    // 4. return
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{Cursor, Read, Write},
    };

    use anyhow::Result;
    use pgp::Deserializable;
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn it_encrypts_the_file_with_password() -> Result<()> {
        // 0. create the temp directory
        let tmp_dir = tempdir().context("create the tmp dir")?;
        let input = tmp_dir.path().join("input.txt").display().to_string();
        let output = tmp_dir.path().join("output.txt.gpg").display().to_string();

        // 1. create to_be_encrypted_file
        let mut f = File::create(input.clone())?;
        f.write_all(b"hello world")?;

        // 2. encrypt the file
        gpg_encrypt(&input, &output)
            .await
            .context("encrypt the file")?;

        // 3. decrypt the file and verify the result
        let mut out_f = File::open(output.clone())?;
        let mut buf = Vec::new();
        out_f.read_to_end(&mut buf)?;
        let msg = Message::from_armor_single(Cursor::new(&buf)).unwrap().0;
        let decrypted = msg
            .decrypt_with_password(|| "password".to_string())
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
