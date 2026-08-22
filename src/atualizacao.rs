use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{bail, Result};
use tempfile::Builder;

#[cfg(target_os = "windows")]
pub fn is_msi_installation(pordosol_home: &Path) -> bool {
    let home_str = pordosol_home.to_string_lossy().to_lowercase();
    home_str.contains("program files")
}

#[cfg(not(target_os = "windows"))]
pub fn is_msi_installation(_pordosol_home: &Path) -> bool {
    false
}

pub fn baixar_arquivo(url: &str, destino: &Path) -> Result<()> {
    println!("Fazendo o download de {}...", url);
    let mut response = reqwest::blocking::get(url)?;
    if !response.status().is_success() {
        bail!("Falha no download: HTTP {}", response.status());
    }
    let mut arquivo = fs::File::create(destino)?;
    response.copy_to(&mut arquivo)?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn executar_atualizacao_msi(msi_path: &Path) -> Result<()> {
    println!("Iniciando atualização silenciosa via MSI...");
    Command::new("msiexec.exe")
        .args(&[
            "/i",
            msi_path.to_str().unwrap(),
            "/qn",
            "/norestart"
        ])
        .spawn()?;
    
    println!("O instalador foi iniciado em segundo plano.");
    println!("Aguarde alguns segundos e a CLI será atualizada. O terminal será fechado.");
    std::process::exit(0);
}

#[cfg(not(target_os = "windows"))]
pub fn executar_atualizacao_msi(_msi_path: &Path) -> Result<()> {
    bail!("MSI suportado apenas no Windows.")
}

pub fn atualizar_via_arquivo(arquivo_path: &Path, pordosol_home: &Path) -> Result<()> {
    let temp_dir = Builder::new().prefix("pordosol-update").tempdir()?;
    let extract_path = temp_dir.path();

    println!("Extraindo arquivos...");
    if arquivo_path.extension().and_then(|s| s.to_str()) == Some("zip") {
        extrair_zip(arquivo_path, extract_path)?;
    } else {
        extrair_targz(arquivo_path, extract_path)?;
    }

    let root = encontrar_raiz_sdk(extract_path)?;

    println!("Substituindo arquivos em {}...", pordosol_home.display());
    substituir_diretorio(&root, pordosol_home)?;

    println!("Limpeza dos temporários...");
    Ok(()) // temp_dir is automatically deleted
}

fn extrair_zip(arquivo_path: &Path, destino: &Path) -> Result<()> {
    let file = fs::File::open(arquivo_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(destino)?;
    Ok(())
}

fn extrair_targz(arquivo_path: &Path, destino: &Path) -> Result<()> {
    let file = fs::File::open(arquivo_path)?;
    let tar = flate2::read::GzDecoder::new(file);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(destino)?;
    Ok(())
}

fn encontrar_raiz_sdk(base: &Path) -> Result<PathBuf> {
    if base.join("bin").exists() && base.join("tools").exists() {
        return Ok(base.to_path_buf());
    }
    for entry in fs::read_dir(base)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path.join("bin").exists() && path.join("tools").exists() {
                return Ok(path);
            }
        }
    }
    bail!("Não foi possível encontrar a estrutura do SDK (bin/ e tools/) no pacote.")
}

fn substituir_diretorio(origem: &Path, destino: &Path) -> Result<()> {
    renomear_executaveis_em_uso(destino)?;

    let entries = walkdir::WalkDir::new(origem).into_iter().filter_map(|e| e.ok());
    for entry in entries {
        let path = entry.path();
        let relative = path.strip_prefix(origem)?;
        let dest_path = destino.join(relative);

        if path.is_dir() {
            if !dest_path.exists() {
                fs::create_dir_all(&dest_path)?;
            }
        } else if path.is_file() {
            if let Some(parent) = dest_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
            let _ = fs::copy(path, &dest_path);
        }
    }

    Ok(())
}

fn renomear_executaveis_em_uso(destino: &Path) -> Result<()> {
    // Remove backups antigos primeiro
    for entry in walkdir::WalkDir::new(destino).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "old_exe" {
                    let _ = fs::remove_file(path);
                }
            }
        }
    }

    // Renomeia executáveis atuais para .old_exe
    for subdir in &["bin", "tools"] {
        let dir = destino.join(subdir);
        if dir.exists() {
            for entry in fs::read_dir(&dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "exe" || ext == "" {
                            let old_path = path.with_extension("old_exe");
                            let _ = fs::rename(&path, &old_path);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
