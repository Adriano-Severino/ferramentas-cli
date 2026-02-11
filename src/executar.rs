use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};
use path_absolutize::Absolutize;

use crate::toolchain::{listar_prs, localizar_binarios, localizar_raiz};

pub fn run_cmd(caminho: &Path, force: bool, arquivo: Option<&Path>, no_build: bool) -> Result<()> {
    run_unificado(caminho, force, arquivo, no_build)
}

fn run_unificado(
    caminho: &Path,
    force: bool,
    arquivo: Option<&Path>,
    no_build: bool,
) -> Result<()> {
    let raiz = localizar_raiz(caminho);
    let arquivo_path = arquivo.map(|p| p.to_path_buf());

    let somente_pbc = arquivo_path
        .as_ref()
        .map(|p| p.extension() == Some(OsStr::new("pbc")))
        .unwrap_or(false);

    let arquivos_fontes: Vec<PathBuf> = if somente_pbc {
        listar_prs(&raiz)
    } else if let Some(ap) = arquivo_path.as_ref() {
        if ap.extension() == Some(OsStr::new("pr")) {
            match ap.absolutize() {
                Ok(abs) => vec![abs.to_path_buf()],
                Err(_) => vec![ap.clone()],
            }
        } else {
            listar_prs(&raiz)
        }
    } else if caminho.is_file() && caminho.extension() == Some(OsStr::new("pr")) {
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

    let (compilador, interpretador) = localizar_binarios(&raiz);

    if !compilador.exists() {
        bail!(
            "Compilador nao encontrado em {}. Rode `pordosol doctor` e configure PORDOSOL_COMPILADOR_PATH/PORDOSOL_HOME.",
            compilador.display()
        );
    }
    if !interpretador.exists() {
        bail!(
            "Interpretador nao encontrado em {}. Rode `pordosol doctor` e configure PORDOSOL_INTERPRETADOR_PATH/PORDOSOL_HOME.",
            interpretador.display()
        );
    }

    let saida_dir = raiz.join("build");
    fs::create_dir_all(&saida_dir).ok();

    let pbc = if somente_pbc {
        arquivo_path.clone().unwrap()
    } else if let Some(ap) = arquivo_path.as_ref() {
        if ap.extension() == Some(OsStr::new("pr")) {
            let nome = ap
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            saida_dir.join(format!("{}.pbc", nome))
        } else {
            ap.clone()
        }
    } else {
        let nome = arquivos_fontes[0]
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        saida_dir.join(format!("{}.pbc", nome))
    };

    let precisa_compilar = (!somente_pbc)
        && !no_build
        && (force || !pbc.exists() || {
            let pbc_modified = pbc.metadata().ok().and_then(|m| m.modified().ok());
            arquivos_fontes.iter().any(|pr| {
                let pr_modified = pr.metadata().ok().and_then(|m| m.modified().ok());
                match (pbc_modified, pr_modified) {
                    (Some(pbc_time), Some(pr_time)) => pr_time > pbc_time,
                    _ => true,
                }
            })
        });

    if precisa_compilar {
        println!("Compilando...");

        let mut cmd = Command::new(&compilador);
        cmd.current_dir(&saida_dir)
            .arg("--target=bytecode")
            .stdin(Stdio::null());
        for arq in &arquivos_fontes {
            cmd.arg(arq);
        }
        let status = cmd.status().context("Falha ao executar o compilador")?;

        if !status.success() {
            bail!("Compilacao falhou (status {})", status);
        }
        println!("Compilacao concluida.");
    } else if no_build {
        println!("--no-build ativo, pulando compilacao.");
    } else {
        println!("Bytecode esta atualizado, pulando compilacao...");
    }

    if no_build && !pbc.exists() {
        bail!(
            "Bytecode nao encontrado em {}. Rode `pordosol build` ou remova --no-build.",
            pbc.display()
        );
    }

    println!("Executando bytecode {}...", pbc.display());
    let status = Command::new(&interpretador)
        .arg(&pbc)
        .stdin(Stdio::null())
        .status()
        .context("Falha ao executar o interpretador")?;

    if !status.success() {
        bail!("Execucao falhou (status {})", status);
    }
    Ok(())
}
