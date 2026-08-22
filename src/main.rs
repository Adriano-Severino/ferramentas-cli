use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use clap::{CommandFactory, Parser, Subcommand};

mod construir;
mod executar;
mod novo;
mod toolchain;
mod atualizacao;

#[derive(Parser, Debug)]
#[command(name = "pordosol", version, about = "Ferramenta CLI do Por do Sol", long_about = None)]
struct Cli {
    /// Mostra ajuda detalhada
    #[arg(long = "ajuda", action = clap::ArgAction::SetTrue)]
    ajuda: bool,
    /// Mostra versao da CLI e tenta detectar a versao do compilador
    #[arg(long = "versao", action = clap::ArgAction::SetTrue)]
    versao: bool,

    #[command(subcommand)]
    command: Option<CommandEnum>,
}

#[derive(Subcommand, Debug)]
enum CommandEnum {
    /// Cria um projeto (estilo dotnet new), mantendo compatibilidade com `novo <caminho>`
    #[command(name = "new", alias = "novo", visible_aliases = ["Novo", "criar", "Criar"])]
    New {
        /// Tipo do projeto (console|web|biblioteca|classe), `list` ou caminho legado
        #[arg(value_name = "TIPO")]
        tipo: Option<String>,
        /// Nome do projeto (cria no diretorio atual se -o nao for fornecido)
        #[arg(short = 'n', long = "name", value_name = "NOME")]
        nome: Option<String>,
        /// Pasta base de saida
        #[arg(short = 'o', long = "output", value_name = "PASTA")]
        output: Option<PathBuf>,
        /// Nao sobrescrever arquivos existentes
        #[arg(long, action = clap::ArgAction::SetTrue)]
        nao_sobrescrever: bool,
    },

    /// Compila arquivos .pr para bytecode (.pbc) por padrao
    #[command(name = "build", alias = "compilar", visible_aliases = ["Build", "Compilar"])]
    Build {
        /// Caminho do projeto ou arquivo .pr (compatibilidade legada)
        #[arg(value_name = "CAMINHO")]
        caminho: Option<PathBuf>,
        /// Caminho do projeto ou arquivo .pr
        #[arg(long = "project", alias = "projeto", value_name = "CAMINHO")]
        project: Option<PathBuf>,
        /// Target de compilacao (bytecode|llvm-ir|cil-bytecode|console|universal)
        #[arg(long, value_name = "ALVO", default_value = "bytecode")]
        target: String,
        /// Caminho de saida (pasta build/ por padrao)
        #[arg(long, alias = "output")]
        saida: Option<PathBuf>,
    },

    /// Compila e executa o programa (equivalente a dotnet run)
    #[command(name = "run", alias = "rodar", visible_aliases = ["Rodar"])]
    Run {
        /// Caminho do projeto ou arquivo .pr (compatibilidade legada)
        #[arg(value_name = "CAMINHO")]
        caminho: Option<PathBuf>,
        /// Caminho do projeto ou arquivo .pr
        #[arg(long = "project", alias = "projeto", value_name = "CAMINHO")]
        project: Option<PathBuf>,
        /// Nao compilar antes de executar
        #[arg(long = "no-build", action = clap::ArgAction::SetTrue)]
        no_build: bool,
        /// Forca recompilacao mesmo se bytecode estiver atualizado
        #[arg(long, action = clap::ArgAction::SetTrue)]
        force: bool,
        /// Arquivo .pbc especifico para executar (pula deducao)
        #[arg(long)]
        arquivo: Option<PathBuf>,
    },

    /// Compila para producao (LLVM), podendo especificar target
    #[command(name = "producao", alias = "release", visible_aliases = ["Release", "Producao"])]
    ReleaseInterno {
        /// Caminho do projeto ou arquivo .pr (padrao: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
        /// Target de producao (ex.: llvm-ir)
        #[arg(long, default_value = "llvm-ir")]
        target: String,
    },

    /// Limpa os artefatos de build (pasta build/)
    #[command(alias = "limpar", visible_alias = "Limpar")]
    Clean {
        /// Caminho do projeto (padrao: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
    },

    /// Mostra informacoes sobre o projeto
    #[command(visible_alias = "Info")]
    Info {
        /// Caminho do projeto (padrao: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
    },

    /// Diagnostica toolchain global (compilador, interpretador e stdlib)
    #[command(name = "doctor", alias = "diagnostico", visible_aliases = ["Doctor", "Diagnostico"])]
    Doctor {
        /// Caminho de referencia para detectar fallback local
        #[arg(default_value = ".")]
        caminho: PathBuf,
    },

    /// Lista os arquivos .pr do projeto
    #[command(visible_alias = "Listar")]
    Listar {
        /// Caminho do projeto (padrao: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
        /// Mostrar apenas arquivos modificados recentemente
        #[arg(long, action = clap::ArgAction::SetTrue)]
        recentes: bool,
    },

    /// Gerencia dependencias do projeto (add, remove, list)
    #[command(visible_alias = "Dep")]
    Dep {
        /// Acao: add|remove|list
        #[arg(value_name = "ACAO", default_value = "list")]
        acao: String,
        /// Nome da dependencia (para add/remove)
        #[arg(value_name = "NOME")]
        nome: Option<String>,
        /// Versao (apenas para add)
        #[arg(long, value_name = "VERSAO")]
        versao: Option<String>,
        /// Caminho local (substitui versao se fornecido)
        #[arg(long, value_name = "CAMINHO")]
        caminho: Option<PathBuf>,
        /// Caminho do projeto (padrao: cwd)
        #[arg(long, default_value = ".")]
        caminho_projeto: PathBuf,
    },

    /// Verifica atualizacoes disponiveis do SDK
    #[command(visible_alias = "Atualizar")]
    Update {
        /// Nao instalar atualizacao automaticamente
        #[arg(long, action = clap::ArgAction::SetTrue)]
        check_only: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.ajuda {
        let mut cmd = Cli::command();
        cmd.print_long_help().ok();
        println!();
        return Ok(());
    }

    if cli.versao {
        let cwd = std::env::current_dir().unwrap();
        imprimir_versoes(&cwd);
        return Ok(());
    }

    match cli.command {
        Some(CommandEnum::New {
            tipo,
            nome,
            output,
            nao_sobrescrever,
        }) => {
            if tipo.as_deref().map(normalizar_tipo).as_deref() == Some("list") {
                return novo::listar_templates_cmd();
            }
            let (destino, template_final) = resolver_new_params_modern(
                tipo.as_deref(),
                nome.as_deref(),
                output.as_deref(),
            )?;
            novo::novo_cmd(&destino, nao_sobrescrever, &template_final)
        }
        Some(CommandEnum::Build {
            caminho,
            project,
            target,
            saida,
        }) => {
            let caminho_final = resolver_project_path(project.as_deref(), caminho.as_deref());
            construir::compilar_cmd(&caminho_final, &target, saida.as_deref())
        }
        Some(CommandEnum::Run {
            caminho,
            project,
            no_build,
            force,
            arquivo,
        }) => {
            let caminho_final = resolver_project_path(project.as_deref(), caminho.as_deref());
            executar::run_cmd(&caminho_final, force, arquivo.as_deref(), no_build)
        }
        Some(CommandEnum::ReleaseInterno { caminho, target }) => {
            construir::producao_cmd(&caminho, &target)
        }
        Some(CommandEnum::Clean { caminho }) => clean_cmd(&caminho),
        Some(CommandEnum::Info { caminho }) => info_cmd(&caminho),
        Some(CommandEnum::Doctor { caminho }) => doctor_cmd(&caminho),
        Some(CommandEnum::Listar { caminho, recentes }) => listar_cmd(&caminho, recentes),
        Some(CommandEnum::Dep {
            acao,
            nome,
            versao,
            caminho: caminho_local,
            caminho_projeto,
        }) => dep_cmd(
            &acao,
            nome.as_deref(),
            versao.as_deref(),
            caminho_local.as_deref(),
            &caminho_projeto,
        ),
        Some(CommandEnum::Update { check_only }) => update_cmd(check_only),
        None => {
            let mut cmd = Cli::command();
            cmd.print_long_help().ok();
            println!();
            Ok(())
        }
    }
}

fn normalizar_tipo(valor: &str) -> String {
    valor.trim().to_ascii_lowercase()
}

fn resolver_project_path(project: Option<&Path>, caminho_legacy: Option<&Path>) -> PathBuf {
    project
        .map(Path::to_path_buf)
        .or_else(|| caminho_legacy.map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn resolver_new_params_modern(
    tipo: Option<&str>,
    nome: Option<&str>,
    output: Option<&Path>,
) -> Result<(PathBuf, String)> {
    let cwd = std::env::current_dir().context("Falha ao obter diretorio atual")?;
    let tipo_str = tipo.unwrap_or("");
    let is_known_template = ["console", "web", "biblioteca", "classe", "list"].contains(&tipo_str.to_ascii_lowercase().as_str());

    let (destino, template_final) = if !is_known_template && nome.is_none() && output.is_none() && !tipo_str.is_empty() {
        // Legacy mode: pordosol novo <caminho>
        let dest = PathBuf::from(tipo_str);
        let absolute_dest = if dest.is_absolute() { dest } else { cwd.join(dest) };
        (absolute_dest, "console".to_string())
    } else {
        let template_final = if tipo_str.is_empty() {
            "console".to_string()
        } else {
            normalizar_tipo(tipo_str)
        };

        let destino = match (output, nome) {
            (Some(out), Some(n)) => out.join(n),
            (Some(out), None) => out.to_path_buf(),
            (None, Some(n)) => cwd.join(n),
            (None, None) => cwd.clone(),
        };
        (destino, template_final)
    };

    Ok((destino, template_final))
}

fn imprimir_versoes(cwd: &Path) {
    let cli_ver = env!("CARGO_PKG_VERSION");
    println!("pordosol CLI v{}", cli_ver);

    let raiz = toolchain::localizar_raiz(cwd);
    let diag = toolchain::diagnosticar_toolchain(&raiz);
    imprimir_versao_ferramenta(&diag.compilador);
    imprimir_versao_ferramenta(&diag.interpretador);
}

fn imprimir_versao_ferramenta(item: &toolchain::DiagnosticoFerramenta) {
    if !item.encontrado {
        println!(
            "{}: nao encontrado (origem {}, caminho {})",
            item.nome,
            item.origem,
            item.caminho.display()
        );
        return;
    }

    if let Some(ver) = toolchain::detectar_versao_binario(&item.caminho) {
        println!("{} {} ({})", item.nome, ver, item.caminho.display());
    } else {
        println!(
            "{}: encontrado em {}, versao nao detectada",
            item.nome,
            item.caminho.display()
        );
    }
}

fn info_cmd(caminho: &Path) -> Result<()> {
    let raiz = toolchain::localizar_raiz(caminho);

    println!("=== Informacoes do Projeto ===");
    println!("Raiz do projeto: {}", raiz.display());

    if let Some(config) = toolchain::carregar_configuracao_projeto(&raiz) {
        if let Some(nome) = config.get("nome").and_then(|v| v.as_str()) {
            println!("Nome: {}", nome);
        }
        if let Some(tipo) = config.get("tipo").and_then(|v| v.as_str()) {
            println!("Tipo: {}", tipo);
        }
        if let Some(versao) = config.get("versao").and_then(|v| v.as_str()) {
            println!("Versao: {}", versao);
        }
        if let Some(descricao) = config.get("descricao").and_then(|v| v.as_str()) {
            println!("Descricao: {}", descricao);
        }
    } else {
        println!("Arquivo de projeto (pordosol.proj) nao encontrado.");
    }

    let arquivos = toolchain::listar_prs(&raiz);
    println!("\nArquivos .pr encontrados: {}", arquivos.len());
    for arq in &arquivos {
        let rel_path = arq.strip_prefix(&raiz).unwrap_or(arq);
        println!("  - {}", rel_path.display());
    }

    let diag = toolchain::diagnosticar_toolchain(&raiz);
    println!("\n=== Ferramentas ===");
    println!(
        "Compilador: {} ({}) [{}]",
        diag.compilador.caminho.display(),
        if diag.compilador.encontrado {
            "ok"
        } else {
            "nao encontrado"
        },
        diag.compilador.origem
    );
    println!(
        "Interpretador: {} ({}) [{}]",
        diag.interpretador.caminho.display(),
        if diag.interpretador.encontrado {
            "ok"
        } else {
            "nao encontrado"
        },
        diag.interpretador.origem
    );
    println!(
        "Biblioteca padrao: {} ({}) [{}]",
        diag.stdlib.caminho.display(),
        if diag.stdlib.encontrado {
            "ok"
        } else {
            "nao encontrada"
        },
        diag.stdlib.origem
    );

    let build_dir = raiz.join("build");
    if build_dir.exists() {
        let entries: Vec<_> = fs::read_dir(&build_dir)
            .unwrap_or_else(|_| fs::read_dir(".").unwrap())
            .filter_map(|e| e.ok())
            .collect();
        println!("\nPasta build/: {} arquivo(s)", entries.len());
    } else {
        println!("\nPasta build/: nao existe");
    }

    Ok(())
}

fn listar_cmd(caminho: &Path, recentes: bool) -> Result<()> {
    let raiz = toolchain::localizar_raiz(caminho);
    let arquivos = toolchain::listar_prs(&raiz);

    if arquivos.is_empty() {
        println!("Nenhum arquivo .pr encontrado em {}/src", raiz.display());
        return Ok(());
    }

    println!("Arquivos .pr no projeto:");

    for arq in &arquivos {
        let rel_path = arq.strip_prefix(&raiz).unwrap_or(arq);

        if recentes {
            if let Ok(metadata) = arq.metadata() {
                if let Ok(modified) = metadata.modified() {
                    let duration = std::time::SystemTime::now()
                        .duration_since(modified)
                        .unwrap_or_default();

                    if duration.as_secs() > 86400 {
                        continue;
                    }

                    let size = metadata.len();
                    println!(
                        "  {} ({} bytes, modificado ha {}s)",
                        rel_path.display(),
                        size,
                        duration.as_secs()
                    );
                } else {
                    println!("  {}", rel_path.display());
                }
            } else {
                println!("  {}", rel_path.display());
            }
        } else if let Ok(metadata) = arq.metadata() {
            let size = metadata.len();
            println!("  {} ({} bytes)", rel_path.display(), size);
        } else {
            println!("  {}", rel_path.display());
        }
    }

    Ok(())
}

fn dep_cmd(
    acao: &str,
    nome: Option<&str>,
    versao: Option<&str>,
    caminho_local: Option<&Path>,
    caminho_projeto: &Path,
) -> Result<()> {
    let raiz = toolchain::localizar_raiz(caminho_projeto);
    let proj_path = raiz.join("pordosol.proj");
    if !proj_path.exists() {
        bail!(
            "Arquivo de projeto nao encontrado em {}",
            proj_path.display()
        );
    }
    let mut json: serde_json::Value = serde_json::from_str(&fs::read_to_string(&proj_path)?)?;
    let deps = json
        .get_mut("dependencias")
        .and_then(|d| d.as_object_mut())
        .ok_or_else(|| anyhow!("Campo 'dependencias' ausente ou invalido"))?;

    match acao.to_ascii_lowercase().as_str() {
        "add" => {
            let nome = nome.ok_or_else(|| anyhow!("Informe o nome da dependencia"))?;
            if deps.contains_key(nome) {
                println!("Dependencia '{}' ja existe. Atualizando...", nome);
            }
            let valor = if let Some(c) = caminho_local {
                serde_json::json!({"path": c.to_string_lossy()})
            } else {
                let ver = versao.unwrap_or("*");
                serde_json::json!(ver)
            };
            deps.insert(nome.to_string(), valor);
            fs::write(&proj_path, serde_json::to_string_pretty(&json)?)?;
            println!("Dependencia '{}' adicionada/atualizada.", nome);
        }
        "remove" | "rm" => {
            let nome = nome.ok_or_else(|| anyhow!("Informe o nome da dependencia"))?;
            if deps.remove(nome).is_some() {
                fs::write(&proj_path, serde_json::to_string_pretty(&json)?)?;
                println!("Dependencia '{}' removida.", nome);
            } else {
                println!("Dependencia '{}' nao encontrada.", nome);
            }
        }
        "list" | "ls" | "listar" => {
            if deps.is_empty() {
                println!("Nenhuma dependencia declarada.");
            } else {
                println!("Dependencias:");
                for (k, v) in deps.iter() {
                    match v {
                        serde_json::Value::String(s) => println!("  - {} = {}", k, s),
                        serde_json::Value::Object(o) => {
                            if let Some(p) = o.get("path") {
                                println!("  - {} (path = {})", k, p);
                            } else {
                                println!("  - {} (obj) = {}", k, v);
                            }
                        }
                        _ => println!("  - {} = {}", k, v),
                    }
                }
            }
        }
        outra => {
            bail!("Acao desconhecida: {} (use add|remove|list)", outra);
        }
    }
    Ok(())
}

fn doctor_cmd(caminho: &Path) -> Result<()> {
    let raiz = toolchain::localizar_raiz(caminho);
    let diag = toolchain::diagnosticar_toolchain(&raiz);

    println!("=== Diagnostico do ambiente Por do Sol ===");
    println!("Caminho de referencia: {}", raiz.display());
    println!();

    let mut pendencias: Vec<&'static str> = Vec::new();

    imprimir_item_doctor(
        &diag.compilador,
        true,
        toolchain::detectar_versao_binario(&diag.compilador.caminho),
        "Defina PORDOSOL_COMPILADOR_PATH ou coloque o compilador em <instalacao>/tools.",
        &mut pendencias,
    );
    imprimir_item_doctor(
        &diag.interpretador,
        true,
        toolchain::detectar_versao_binario(&diag.interpretador.caminho),
        "Defina PORDOSOL_INTERPRETADOR_PATH ou coloque o interpretador em <instalacao>/tools.",
        &mut pendencias,
    );
    imprimir_item_doctor(
        &diag.stdlib,
        false,
        None,
        "Defina PORDOSOL_STDLIB_PATH ou instale a stdlib em <instalacao>/tools/stdlib.",
        &mut pendencias,
    );

    println!();
    if diag.pronto() {
        println!("Resultado: ambiente pronto para `pordosol build` e `pordosol run`.");
    } else {
        println!("Resultado: ambiente com pendencias.");
        println!("Acoes corretivas sugeridas:");
        for (idx, item) in pendencias.iter().enumerate() {
            println!("{}. {}", idx + 1, item);
        }
        println!("Dica: configure `PORDOSOL_HOME` para centralizar tools e templates.");
    }

    Ok(())
}

fn imprimir_item_doctor(
    item: &toolchain::DiagnosticoFerramenta,
    mostrar_versao: bool,
    versao: Option<String>,
    acao: &'static str,
    pendencias: &mut Vec<&'static str>,
) {
    let status = if item.encontrado { "OK" } else { "FALHA" };
    println!("{}: {}", item.nome, status);
    println!("  caminho: {}", item.caminho.display());
    println!("  origem: {}", item.origem);
    if mostrar_versao {
        if let Some(v) = versao {
            println!("  versao: {}", v);
        } else if item.encontrado {
            println!("  versao: nao detectada");
        }
    }
    if !item.encontrado {
        pendencias.push(acao);
    }
}

fn update_cmd(check_only: bool) -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("=== Verificando atualizacoes do Por do Sol SDK ===");
    println!("Versao atual: {}", current_version);
    println!();

    // GitHub API to check for latest release
    let api_url = "https://api.github.com/repos/Adriano-Severino/ferramentas-cli/releases/latest";
    
    match check_github_update(api_url, current_version, check_only) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Erro ao verificar atualizacoes automaticamente: {}", e);
            println!();
            println!("Voce pode verificar manualmente em: https://github.com/Adriano-Severino/ferramentas-cli/releases");
            Ok(()) // Don't fail the command, just report the error
        }
    }
}

fn check_github_update(api_url: &str, current_version: &str, check_only: bool) -> Result<()> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    
    let response = client.get(api_url)
        .header("User-Agent", "pordosol-cli")
        .send()?;

    if !response.status().is_success() {
        anyhow::bail!("API retornou status: {}", response.status());
    }

    let release: serde_json::Value = response.json()?;
    
    if let Some(tag_name) = release.get("tag_name").and_then(|v| v.as_str()) {
        let latest_version = tag_name.trim_start_matches('v');
        println!("Versao mais recente: {}", latest_version);
        println!();
        
        if latest_version == current_version {
            println!("Voce ja esta usando a versao mais recente!");
            return Ok(());
        }
        
        println!("Nova versao disponivel: {} -> {}", current_version, latest_version);
        
        if let Some(html_url) = release.get("html_url").and_then(|v| v.as_str()) {
            println!("Release notes: {}", html_url);
        }
        
        if let Some(body) = release.get("body").and_then(|v| v.as_str()) {
            println!();
            println!("Notas da release:");
            println!("{}", body);
        }
        
        if check_only {
            println!();
            println!("Use 'pordosol update' para ver instrucoes de atualizacao.");
            println!("Baixe manualmente de: https://github.com/Adriano-Severino/ferramentas-cli/releases");
        } else {
            println!();
            println!("Iniciando atualização automática para a versão {}...", latest_version);
            if let Err(e) = executar_atualizacao_automatica(&release) {
                println!("Falha na atualização automática: {}", e);
                println!("Por favor, atualize manualmente baixando em: https://github.com/Adriano-Severino/ferramentas-cli/releases");
            }
        }
    } else {
        println!("Nao foi possivel determinar a versao mais recente.");
    }

    Ok(())
}

fn executar_atualizacao_automatica(release: &serde_json::Value) -> Result<()> {
    let pordosol_home = std::env::var("PORDOSOL_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            // Fallback: tentar descobrir pela raiz do binário
            if let Ok(exe) = std::env::current_exe() {
                if let Some(parent) = exe.parent() { // ex: bin
                    if let Some(home) = parent.parent() { // ex: PORDOSOL_HOME
                        return home.to_path_buf();
                    }
                }
            }
            std::path::PathBuf::from(".")
        });

    let assets = release.get("assets").and_then(|a| a.as_array()).ok_or_else(|| anyhow::anyhow!("Nenhum asset encontrado na release"))?;
    
    let is_msi = atualizacao::is_msi_installation(&pordosol_home);
    let target_ext = if cfg!(target_os = "windows") {
        if is_msi { ".msi" } else { ".zip" }
    } else {
        ".tar.gz"
    };

    let mut asset_url = None;
    let mut asset_name = String::new();

    for asset in assets {
        if let Some(name) = asset.get("name").and_then(|n| n.as_str()) {
            if name.ends_with(target_ext) {
                asset_url = asset.get("browser_download_url").and_then(|u| u.as_str()).map(|s| s.to_string());
                asset_name = name.to_string();
                break;
            }
        }
    }

    let download_url = asset_url.ok_or_else(|| anyhow::anyhow!("Asset {} não encontrado para esta release", target_ext))?;
    
    let temp_dir = tempfile::Builder::new().prefix("pordosol-download").tempdir()?;
    let arquivo_baixado = temp_dir.path().join(&asset_name);

    atualizacao::baixar_arquivo(&download_url, &arquivo_baixado)?;

    if target_ext == ".msi" {
        atualizacao::executar_atualizacao_msi(&arquivo_baixado)?;
    } else {
        atualizacao::atualizar_via_arquivo(&arquivo_baixado, &pordosol_home)?;
    }

    Ok(())
}

fn clean_cmd(caminho: &Path) -> Result<()> {
    let raiz = toolchain::localizar_raiz(caminho);
    let build_dir = raiz.join("build");

    if !build_dir.exists() {
        println!("Pasta build/ nao existe em {}", raiz.display());
        return Ok(());
    }

    let entries = fs::read_dir(&build_dir)?;
    let mut count = 0;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            fs::remove_file(&path).context(format!("Falha ao remover {}", path.display()))?;
            count += 1;
        } else if path.is_dir() {
            fs::remove_dir_all(&path)
                .context(format!("Falha ao remover diretorio {}", path.display()))?;
            count += 1;
        }
    }

    println!(
        "Limpeza concluida: {} item(s) removido(s) de {}",
        count,
        build_dir.display()
    );
    Ok(())
}
