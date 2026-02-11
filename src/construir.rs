use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};
use path_absolutize::Absolutize;

use crate::toolchain::{
    carregar_configuracao_projeto, listar_prs, localizar_binarios, localizar_raiz,
};

pub fn compilar_cmd(caminho: &Path, target: &str, saida: Option<&Path>) -> Result<()> {
    let raiz = localizar_raiz(caminho);
    let config = carregar_configuracao_projeto(&raiz);

    let target_final = if target == "bytecode" && config.is_some() {
        config
            .as_ref()
            .and_then(|c| c.get("configuracao"))
            .and_then(|c| c.get("target_padrao"))
            .and_then(|t| t.as_str())
            .unwrap_or(target)
    } else {
        target
    };

    let arquivos: Vec<PathBuf> =
        if caminho.is_file() && caminho.extension() == Some(OsStr::new("pr")) {
            match caminho.absolutize() {
                Ok(abs) => vec![abs.to_path_buf()],
                Err(_) => vec![caminho.to_path_buf()],
            }
        } else {
            let list = listar_prs(&raiz);
            if list.is_empty() {
                bail!("Nenhum arquivo .pr encontrado em {}/src", raiz.display());
            }
            list
        };

    let (compilador, _interp) = localizar_binarios(&raiz);
    if !compilador.exists() {
        bail!(
            "Compilador nao encontrado em {}. Rode `pordosol doctor` e configure PORDOSOL_COMPILADOR_PATH/PORDOSOL_HOME.",
            compilador.display()
        );
    }

    let saida_dir = saida
        .map(Path::to_path_buf)
        .unwrap_or_else(|| raiz.join("build"));
    fs::create_dir_all(&saida_dir).ok();

    let tnorm = target_final.trim().to_ascii_lowercase();
    let alvo_flag = match tnorm.as_str() {
        "bytecode" | "bc" => "--target=bytecode",
        "llvm" | "llvm-ir" => "--target=llvm-ir",
        "cil-bytecode" => "--target=cil-bytecode",
        "console" => "--target=console",
        "universal" => "--target=universal",
        other => {
            eprintln!("Alvo desconhecido: {}. Usando bytecode.", other);
            "--target=bytecode"
        }
    };

    println!(
        "Compilando para {} com {} arquivo(s)...",
        target_final,
        arquivos.len()
    );

    let mut cmd = Command::new(&compilador);
    cmd.current_dir(&saida_dir)
        .arg(alvo_flag)
        .stdin(Stdio::null());
    for arq in &arquivos {
        cmd.arg(arq);
    }

    let status = cmd.status().context("Falha ao executar o compilador")?;
    if !status.success() {
        bail!("Compilacao falhou (status {})", status);
    }

    println!("Compilado com sucesso. Saida em {}", saida_dir.display());

    if let Ok(entries) = fs::read_dir(&saida_dir) {
        let arquivos_build: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .collect();

        if !arquivos_build.is_empty() {
            println!("Arquivos gerados:");
            for entry in arquivos_build {
                let path = entry.path();
                let rel_path = path.strip_prefix(&saida_dir).unwrap_or(&path);
                if let Ok(metadata) = entry.metadata() {
                    println!("  {} ({} bytes)", rel_path.display(), metadata.len());
                } else {
                    println!("  {}", rel_path.display());
                }
            }
        }
    }

    Ok(())
}

pub fn producao_cmd(caminho: &Path, target: &str) -> Result<()> {
    let raiz = localizar_raiz(caminho);
    let arquivos: Vec<PathBuf> =
        if caminho.is_file() && caminho.extension() == Some(OsStr::new("pr")) {
            vec![caminho.to_path_buf()]
        } else {
            let list = listar_prs(&raiz);
            if list.is_empty() {
                bail!("Nenhum arquivo .pr encontrado em {}/src", raiz.display());
            }
            list
        };

    let (compilador, _interp) = localizar_binarios(&raiz);
    if !compilador.exists() {
        bail!(
            "Compilador nao encontrado em {}. Rode `pordosol doctor` e configure PORDOSOL_COMPILADOR_PATH/PORDOSOL_HOME.",
            compilador.display()
        );
    }

    let saida_dir = raiz.join("build");
    fs::create_dir_all(&saida_dir).ok();

    let tnorm = target.trim().to_ascii_lowercase();
    let alvo_flag = match tnorm.as_str() {
        "llvm" | "llvm-ir" => "--target=llvm-ir",
        other => {
            eprintln!(
                "Target de producao desconhecido: {}. Usando llvm-ir.",
                other
            );
            "--target=llvm-ir"
        }
    };

    let mut cmd = Command::new(&compilador);
    cmd.current_dir(&saida_dir)
        .arg(alvo_flag)
        .stdin(Stdio::null());
    for arq in &arquivos {
        cmd.arg(arq);
    }

    let status = cmd
        .status()
        .context("Falha ao executar o compilador (producao)")?;
    if !status.success() {
        bail!("Compilacao de producao falhou (status {})", status);
    }

    println!("Producao concluida. Artefatos em {}", saida_dir.display());
    Ok(())
}
