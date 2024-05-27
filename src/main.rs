use aes_gcm_siv::{
    aead::{Aead, KeyInit},
    Aes256GcmSiv, Key, Nonce,
};
use std::io::{self, Read};
use std::process;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

// Функция foo для обработки файлов
fn decrypt(data: &[u8], bytes_of_key: &[u8]) -> Result<Vec<u8>, aes_gcm_siv::aead::Error> {
    println!("bytes {:?}", bytes_of_key);
    let key = Key::<Aes256GcmSiv>::from_slice(&bytes_of_key);
    let cipher = Aes256GcmSiv::new(key);
    let nonce = Nonce::from_slice(b"unique nonce");
    let decrypted_bytes = cipher.decrypt(nonce, &*data);
    decrypted_bytes
}

// Распаковка zip архива и обработка файлов
fn process_zip_file(zip_file: &Path, dest_dir: &Path, key_file: &Path) -> io::Result<()> {
    let file = fs::File::open(zip_file)?;
    let mut archive = zip::ZipArchive::new(file)?;

    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_owned();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        println!("name: {}", file_name);
        let key = fs::read(key_file)?;

        let mut processed_data = Vec::<u8>::new();

        if !file_name.contains(".bin") {
            processed_data = match decrypt(&buffer, &key) {
                Ok(processed_data) => processed_data,
                Err(_) => {
                    println!("Decryption fail");
                    continue;
                }
            };
            let mut dest_path = PathBuf::from(dest_dir);
            dest_path.push(&file_name);
    
            fs::write(dest_path, processed_data)?;
        } else {
            processed_data = buffer;
            
            let mut file = fs::OpenOptions::new().append(true).create(true).open(dest_dir.to_str().unwrap().to_owned() + "/data.bin").unwrap();
            let _ = file.write_all(&processed_data);
        }
        
    }
    
    Ok(())
}
    
fn main() {
    // Получение аргументов командной строки
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <source_dir> <dest_dir> <key_file>", args[0]);
        process::exit(1);
    }

    let source_dir = Path::new(&args[1]);
    let dest_dir = Path::new(&args[2]);
    let key_file = Path::new(&args[3]);

    // Проверка существования директорий и файла с ключом
    if !source_dir.is_dir() || !dest_dir.is_dir() || !key_file.is_file() {
        eprintln!("Invalid directory or key file path.");
        process::exit(1);
    }
    
    
    
    let data_file = fs::File::create(dest_dir.to_str().unwrap().to_owned() + "/data.bin").unwrap();
    // Обработка каждого zip архива в исходной директории
    match fs::read_dir(source_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(extension) = entry.path().extension() {
                        if extension == "zip" {
                            if let Err(err) = process_zip_file(&entry.path(), dest_dir, key_file) {
                                eprintln!("Error processing zip file: {}", err);
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error reading source directory: {}", err);
            process::exit(1);
        }
    }
}
