use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use path_absolutize::Absolutize;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(name = "pordosol", version, about = "Ferramenta CLI do Por do Sol", long_about=None)]
struct Cli {
    /// Mostra ajuda detalhada
    #[arg(long = "ajuda", action = clap::ArgAction::SetTrue)]
    ajuda: bool,
    /// Mostra versão da CLI e tenta detectar a versão do compilador
    #[arg(long = "versao", action = clap::ArgAction::SetTrue)]
    versao: bool,

    #[command(subcommand)]
    command: Option<CommandEnum>,
}

#[derive(Subcommand, Debug)]
enum CommandEnum {
    /// Cria um projeto base com src/ e programa.pr
    #[command(alias = "novo", visible_alias = "Novo")]
    Novo {
        /// Caminho do diretório do projeto a criar (padrão: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
        /// Não sobrescrever arquivos existentes
        #[arg(long, action = clap::ArgAction::SetTrue)]
        nao_sobrescrever: bool,
    },
    /// Compila arquivos .pr para bytecode (.pbc) por padrão
    #[command(alias = "compilar", visible_alias = "Compilar")]
    Compilar {
        /// Caminho do projeto ou arquivo .pr (padrão: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
        /// Target de compilação (bytecode|llvm-ir|cil-bytecode|console|universal)
        #[arg(long, value_name = "ALVO", default_value = "bytecode")]
        target: String,
        /// Caminho de saída (pasta build/ por padrão)
        #[arg(long)]
        saida: Option<PathBuf>,
    },

    /// Executa um bytecode (.pbc) com o interpretador
    #[command(alias = "rodar", visible_alias = "Rodar")]
    #[command(alias = "executar", visible_alias = "Executar")]
    Rodar {
        /// Caminho do projeto ou arquivo .pbc (padrão: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
        /// Arquivo .pbc específico (se omitido, tenta deduzir pelo nome base)
        #[arg(long)]
        arquivo: Option<PathBuf>,
    },

    /// Compila para produção (LLVM), podendo especificar target
    #[command(alias = "producao", visible_alias = "Producao")]
    Producao {
        /// Caminho do projeto ou arquivo .pr (padrão: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
        /// Target de produção (ex.: llvm-ir)
        #[arg(long, default_value = "llvm-ir")]
        target: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Alias manual para ajuda
    if cli.ajuda {
        let mut cmd = Cli::command();
        cmd.print_long_help().ok();
        println!();
        return Ok(());
    }

    // Versões
    if cli.versao {
        let cwd = std::env::current_dir().unwrap();
        imprimir_versoes(&cwd);
        return Ok(());
    }

    match cli.command {
        Some(CommandEnum::Novo {
            caminho,
            nao_sobrescrever,
        }) => novo_cmd(&caminho, nao_sobrescrever),
        Some(CommandEnum::Compilar {
            caminho,
            target,
            saida,
        }) => compilar_cmd(&caminho, &target, saida.as_deref()),
        Some(CommandEnum::Rodar { caminho, arquivo }) => rodar_cmd(&caminho, arquivo.as_deref()),
        Some(CommandEnum::Producao { caminho, target }) => producao_cmd(&caminho, &target),
        None => {
            let mut cmd = Cli::command();
            cmd.print_long_help().ok();
            println!();
            Ok(())
        }
    }
}

fn localizar_raiz(caminho: &Path) -> PathBuf {
    // Considera a pasta que contém src/ como raiz do projeto
    let mut p = caminho.absolutize().unwrap().to_path_buf();
    if p.is_file() {
        if let Some(parent) = p.parent() {
            p = parent.to_path_buf();
        }
    }
    // Sobe no máximo 5 níveis procurando por src/
    for _ in 0..5 {
        if p.join("src").is_dir() {
            return p;
        }
        if let Some(par) = p.parent() {
            p = par.to_path_buf();
        } else {
            break;
        }
    }
    caminho.absolutize().unwrap().to_path_buf()
}

fn listar_prs(raiz: &Path) -> Vec<PathBuf> {
    let src = raiz.join("src");
    let mut arquivos: Vec<PathBuf> = WalkDir::new(&src)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| p.is_file() && p.extension() == Some(OsStr::new("pr")))
        .collect();

    // Coloca programa.pr no início se existir
    let preferido = src.join("programa.pr");
    if let Some(pos) = arquivos.iter().position(|p| p == &preferido) {
        let pref = arquivos.remove(pos);
        arquivos.insert(0, pref);
    }
    arquivos
}

fn localizar_binarios(raiz: &Path) -> (PathBuf, PathBuf) {
    // Binários são colocados em <raiz>/lib pelo configurar-ambiente.ps1
    let lib = raiz.join("lib");
    let (mut comp, mut interp) = (lib.join("compilador"), lib.join("interpretador"));
    if cfg!(windows) {
        comp.set_extension("exe");
        interp.set_extension("exe");
    }
    (comp, interp)
}

fn imprimir_versoes(cwd: &Path) {
    // Versão da CLI
    let cli_ver = env!("CARGO_PKG_VERSION");
    println!("pordosol CLI v{}", cli_ver);

    // Versão do compilador, se disponível em lib/
    let raiz = localizar_raiz(cwd);
    let (compilador, _) = localizar_binarios(&raiz);
    if compilador.exists() {
        match Command::new(&compilador).output() {
            Ok(out) => {
                let txt = String::from_utf8_lossy(&out.stdout);
                if let Some(pos) = txt.find("(v") {
                    if let Some(end) = txt[pos..].find(')') {
                        let ver = &txt[pos + 1..pos + end];
                        println!("compilador {}", ver);
                        return;
                    }
                }
                println!(
                    "compilador: encontrado em {}, versão não detectada",
                    compilador.display()
                );
            }
            Err(_) => println!(
                "compilador: encontrado em {}, mas falhou ao executar",
                compilador.display()
            ),
        }
    } else {
        println!(
            "compilador: não encontrado (esperei em {})",
            compilador.display()
        );
    }
}

fn novo_cmd(destino: &Path, nao_sobrescrever: bool) -> Result<()> {
    let raiz = destino.absolutize().unwrap().to_path_buf();
    fs::create_dir_all(raiz.join("src"))?;
    fs::create_dir_all(raiz.join("build")).ok();
    // cria exemplo src/programa.pr se não existir
    let prog = raiz.join("src").join("programa.pr");
    if prog.exists() && nao_sobrescrever {
        println!("Projeto já contém src/programa.pr (não sobrescrito).");
    } else if !prog.exists() || !nao_sobrescrever {
        let exemplo = r#"// programa.pr - exemplo inicial
função vazio Principal()
{
    imprima(\"Olá, Por do Sol!\");
}
"#;
        fs::write(&prog, exemplo)?;
        println!("Criado {}", prog.display());
    }
    println!("Projeto base pronto em {}", raiz.display());
    Ok(())
}

fn compilar_cmd(caminho: &Path, target: &str, saida: Option<&Path>) -> Result<()> {
    let raiz = localizar_raiz(caminho);

    // Descobrir lista de arquivos .pr
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
            "Compilador não encontrado em {}. Rode configurar-ambiente.ps1.",
            compilador.display()
        );
    }

    let saida_dir = saida
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| raiz.join("build"));
    fs::create_dir_all(&saida_dir).ok();

    let tnorm = target.trim().to_ascii_lowercase();
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

    let mut cmd = Command::new(&compilador);
    cmd.current_dir(&saida_dir)
        .arg(alvo_flag)
        .stdin(Stdio::null());
    for arq in &arquivos {
        cmd.arg(arq);
    }
    let status = cmd.status().context("Falha ao executar o compilador")?;

    if !status.success() {
        bail!("Compilação falhou (status {})", status);
    }

    println!("Compilado com sucesso. Saída em {}", saida_dir.display());
    Ok(())
}

fn rodar_cmd(caminho: &Path, arquivo: Option<&Path>) -> Result<()> {
    let raiz = localizar_raiz(caminho);
    let (_compilador, interpretador) = localizar_binarios(&raiz);
    if !interpretador.exists() {
        bail!(
            "Interpretador não encontrado em {}. Rode configurar-ambiente.ps1.",
            interpretador.display()
        );
    }

    // Se foi passado um arquivo .pbc, usa; caso contrário, tenta deduzir a partir do .pr principal
    let pbc = if let Some(arq) = arquivo {
        arq.to_path_buf()
    } else {
        let arquivos = listar_prs(&raiz);
        if arquivos.is_empty() {
            bail!("Nenhum arquivo .pr encontrado para deduzir o .pbc");
        }
        let nome = arquivos[0]
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        raiz.join("build").join(format!("{}.pbc", nome))
    };

    if !pbc.exists() {
        bail!(
            "Bytecode não encontrado: {}. Rode pordosol compilar.",
            pbc.display()
        );
    }

    let status = Command::new(&interpretador)
        .arg(&pbc)
        .stdin(Stdio::null())
        .status()
        .context("Falha ao executar o interpretador")?;

    if !status.success() {
        bail!("Execução falhou (status {})", status);
    }
    Ok(())
}

fn producao_cmd(caminho: &Path, target: &str) -> Result<()> {
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
            "Compilador não encontrado em {}. Rode configurar-ambiente.ps1.",
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
                "Target de produção desconhecido: {}. Usando llvm-ir.",
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
        .context("Falha ao executar o compilador (produção)")?;

    if !status.success() {
        bail!("Compilação de produção falhou (status {})", status);
    }

    println!("Produção concluída. Artefatos em {}", saida_dir.display());
    Ok(())
}
