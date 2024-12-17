use std::fs::File;
    use std::path::PathBuf;
    use std::io::{self, Read};
    use charset_normalizer_rs::from_path;
    
    pub fn detect_and_convert_to_utf8(file_path: &str) -> io::Result<String> {
          // Detecta o encoding do arquivo
    let result = from_path(&PathBuf::from(file_path), None).unwrap();
    let best_guess = result.get_best();

    // Obtém o encoding detectado
    if let Some(_encoding) = best_guess.and_then(|e| Some(e.encoding()) ) {
        // Abre o arquivo com o encoding detectado
        let mut file = File::open(file_path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        Ok(String::from_utf8_lossy(&bytes).to_string())
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Encoding não detectado"))
    }
}