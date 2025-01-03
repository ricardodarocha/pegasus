use std::fs;
use std::path::Path;
use indicatif::ProgressBar;

use crate::fextractor;
use std::result::Result;

pub fn contar_arquivos(path: &Path, ext: Option<&str>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut count = 0;

    for entry in fs::read_dir(path)? {
        let entry = entry?;  
        let path = entry.path();

        if path.is_dir() {
            count += contar_arquivos(&path, ext)?;
        } else if path.is_file() {
            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if let Some(ext) = ext {
                        if ext == ext_str {
                            count += 1;
                        }
                    }
                    else
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    Ok(count)
}

pub fn processa_pasta(path: &str, count: &mut u32, limit: u32, silent: bool, pb: &mut ProgressBar) -> Result<(), Box<dyn std::error::Error>> {
    
    if *count >= limit {
        return Ok(());
    }
    // Percorre todos os arquivos e subdiretórios
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Chama a função recursivamente para subdiretórios
            processa_subpastas(&path, count, limit, silent, pb)?;
        } else if let Some(extension) = path.extension() {
            
            if extension == "pas" {
                *count += 1;
                // Se for um arquivo .pas, processa
                match fextractor::search(path.to_string_lossy().to_string().as_str()) {
                    Ok(profile) => {
                        if !silent {
                            println!("[{}] Arquivo: {:?}", *count, path.display());
                            println!("{}", profile.filename);
                            println!("Interface Uses: {:?}", profile.interf);
                            println!("Implementation Uses: {:?}", profile.implemen);
                        };

                        pb.inc(1);
                        profile.save();
                    }
                    Err(e) => eprintln!("Erro ao processar arquivo {:?}: {}", path.display(), e),
                }

                if *count >= limit {
                    println!("Limite atingido: {}", count);
                    break;
                }
            }
        }
    }

    Ok(())
}

pub fn processa_subpastas(path: &Path, count: &mut u32, limit: u32, silent: bool, pb: &mut ProgressBar) -> Result<(), Box<dyn std::error::Error>> {
    if *count >= limit {
        return Ok(());
    }
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if *count >= limit {
            break;
        }
        if path.is_dir() {
            processa_subpastas(&path, count, limit, silent, pb)?;
        } else if let Some(extension) = path.extension() {
            if extension == "pas" {
                // Se for um arquivo .pas, processa
                *count += 1;
                match fextractor::search(path.to_string_lossy().to_string().as_str()) {
                    Ok(profile) => {
                        if !silent {
                            println!("..[{}] Arquivo: {:?}", *count, path.display());
                            println!("Interface Uses: {:?}", profile.interf);
                            println!("Implementation Uses: {:?}", profile.implemen);
                        };

                        pb.inc(1);
                        profile.save();
                    }
                    Err(e) => eprintln!("Erro ao processar arquivo {:?}: {}", path.display(), e),
                }

                if *count >= limit {
                    println!("Limite atingido: {}", count);
                    break;
                }
            }
        }
    }

    Ok(())
}