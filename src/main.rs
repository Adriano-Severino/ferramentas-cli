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
    #[command(alias = "criar", visible_alias = "Criar")] // evitar alias duplicado
    Novo {
        /// Caminho do diretório do projeto a criar (padrão: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
        /// Não sobrescrever arquivos existentes
        #[arg(long, action = clap::ArgAction::SetTrue)]
        nao_sobrescrever: bool,
        /// Tipo de template do projeto (console|biblioteca|classe)
        #[arg(long, default_value = "console")]
        template: String,
    },

    /// Compila arquivos .pr para bytecode (.pbc) por padrão
    #[command(alias = "build", visible_alias = "Build")]
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

    /// Compila e executa o programa (equivalente a dotnet run)
    // Nome primário: run (via variante Run). Alias adicional: rodar
    #[command(name="run", alias="rodar", visible_aliases=["Rodar"]) ]
    Exec {
        /// Caminho do projeto ou arquivo .pr (padrão: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
        /// Força recompilação mesmo se bytecode estiver atualizado
        #[arg(long, action = clap::ArgAction::SetTrue)]
        force: bool,
        /// Arquivo .pbc específico para executar (pula dedução)
        #[arg(long)]
        arquivo: Option<PathBuf>,
    },

    /// Compila para produção (LLVM), podendo especificar target
    #[command(name="producao", alias="release", visible_aliases=["Release","Producao"])]
    ReleaseInterno {
        /// Caminho do projeto ou arquivo .pr (padrão: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
        /// Target de produção (ex.: llvm-ir)
        #[arg(long, default_value = "llvm-ir")]
        target: String,
    },

    /// Limpa os artefatos de build (pasta build/)
    #[command(alias = "limpar", visible_alias = "Limpar")]
    Clean {
        /// Caminho do projeto (padrão: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
    },

    /// Mostra informações sobre o projeto
    #[command(visible_alias = "Info")]
    Info {
        /// Caminho do projeto (padrão: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
    },

    /// Lista os arquivos .pr do projeto
    #[command(visible_alias = "Listar")]
    Listar {
        /// Caminho do projeto (padrão: cwd)
        #[arg(default_value = ".")]
        caminho: PathBuf,
        /// Mostrar apenas arquivos modificados recentemente
        #[arg(long, action = clap::ArgAction::SetTrue)]
        recentes: bool,
    },

    /// Gerencia dependências do projeto (add, remove, list)
    #[command(visible_alias = "Dep")]
    Dep {
        /// Ação: add|remove|list
        #[arg(value_name = "ACAO", default_value = "list")]
        acao: String,
        /// Nome da dependência (para add/remove)
        #[arg(value_name = "NOME")]
        nome: Option<String>,
        /// Versão (apenas para add)
        #[arg(long, value_name = "VERSAO")]
        versao: Option<String>,
        /// Caminho local (substitui versão se fornecido)
        #[arg(long, value_name = "CAMINHO")]
        caminho: Option<PathBuf>,
        /// Caminho do projeto (padrão: cwd)
        #[arg(long, default_value = ".")]
        caminho_projeto: PathBuf,
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
            template,
        }) => novo_cmd(&caminho, nao_sobrescrever, &template),
        Some(CommandEnum::Compilar {
            caminho,
            target,
            saida,
        }) => compilar_cmd(&caminho, &target, saida.as_deref()),
        Some(CommandEnum::Exec {
            caminho,
            force,
            arquivo,
        }) => run_cmd(&caminho, force, arquivo.as_deref()),
        Some(CommandEnum::ReleaseInterno { caminho, target }) => producao_cmd(&caminho, &target),
        Some(CommandEnum::Clean { caminho }) => clean_cmd(&caminho),
        Some(CommandEnum::Info { caminho }) => info_cmd(&caminho),
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
    // Procura pasta lib com compilador/interpretador subindo até 5 níveis
    let mut atual = raiz.to_path_buf();
    for _ in 0..6 {
        // inclui nível atual
        let lib = atual.join("lib");
        let mut comp = lib.join("compilador");
        let mut interp = lib.join("interpretador");
        if cfg!(windows) {
            comp.set_extension("exe");
            interp.set_extension("exe");
        }
        if comp.exists() && interp.exists() {
            return (comp, interp);
        }
        if let Some(par) = atual.parent() {
            atual = par.to_path_buf();
        } else {
            break;
        }
    }
    // fallback: caminho esperado dentro da raiz
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

fn novo_cmd(destino: &Path, nao_sobrescrever: bool, template: &str) -> Result<()> {
    let raiz = destino.absolutize().unwrap().to_path_buf();
    fs::create_dir_all(raiz.join("src"))?;
    fs::create_dir_all(raiz.join("build")).ok();

    // Criar arquivo de projeto (pordosol.proj)
    let projeto_file = raiz.join("pordosol.proj");
    if !projeto_file.exists() || !nao_sobrescrever {
        let nome_projeto = raiz
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let conteudo_projeto = match template {
            "biblioteca" => format!(
                r#"{{
    "nome": "{}",
    "tipo": "biblioteca",
    "versao": "1.0.0",
    "descricao": "Uma biblioteca em Por do Sol",
    "autor": "",
    "dependencias": {{}},
    "configuracao": {{
        "target_padrao": "llvm-ir",
        "otimizacao": true
    }}
}}"#,
                nome_projeto
            ),
            "classe" => format!(
                r#"{{
    "nome": "{}",
    "tipo": "classe",
    "versao": "1.0.0",
    "descricao": "Uma classe em Por do Sol",
    "autor": "",
    "dependencias": {{}},
    "configuracao": {{
        "target_padrao": "bytecode",
        "otimizacao": false
    }}
}}"#,
                nome_projeto
            ),
            _ => format!(
                r#"{{
    "nome": "{}",
    "tipo": "console",
    "versao": "1.0.0",
    "descricao": "Uma aplicação console em Por do Sol",
    "autor": "",
    "dependencias": {{}},
    "configuracao": {{
        "target_padrao": "bytecode",
        "otimizacao": false
    }}
}}"#,
                nome_projeto
            ),
        };

        fs::write(&projeto_file, conteudo_projeto)?;
        println!("Criado {}", projeto_file.display());
    }

    // Criar arquivo principal baseado no template
    let prog = raiz.join("src").join("programa.pr");
    if prog.exists() && nao_sobrescrever {
        println!("Projeto já contém src/programa.pr (não sobrescrito).");
    } else if !prog.exists() || !nao_sobrescrever {
        let exemplo = match template {
            "biblioteca" => {
                r#"// biblioteca.pr - template de biblioteca
usando Sistema.IO;

classe publica MinhaClasse
{
    // Propriedade pública
    inteiro valor { get; set; }
    
    // Construtor
    publico MinhaClasse(inteiro valorInicial)
    {
        este.valor = valorInicial;
    }
    
    // Método público
    publico inteiro ObterValorDobrado()
    {
        retorne este.valor * 2;
    }
}
"#
            }
            "classe" => {
                r#"// classe.pr - template de classe
usando Sistema.IO;

classe MinhaClasse
{
    // Propriedades
    texto nome { get; set; }
    inteiro idade { get; set; }
    
    // Construtor
    publico MinhaClasse(texto nome, inteiro idade)
    {
        este.nome = nome;
        este.idade = idade;
    }
    
    // Métodos
    publico vazio ApresentarSe()
    {
        imprima($"Olá, eu sou {este.nome} e tenho {este.idade} anos.");
    }
}

função vazio Principal()
{
    var pessoa = novo MinhaClasse("João", 25);
    pessoa.ApresentarSe();
}
"#
            }
            _ => {
                r#"// programa.pr - exemplo inicial
função vazio Principal()
{
    imprima("Olá, Por do Sol!");
    
    // Exemplo com variáveis
    var nome = "Mundo";
    var numero = 42;
    
    imprima($"Olá, {nome}! O número é {numero}");
}
"#
            }
        };

        fs::write(&prog, exemplo)?;
        println!("Criado {}", prog.display());
    }

    // Criar README.md
    let readme = raiz.join("README.md");
    if !readme.exists() || !nao_sobrescrever {
        let nome_projeto = raiz
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let conteudo_readme = format!(
            r#"# {}

Um projeto em Por do Sol.

## Como usar

### Compilar e executar
```bash
pordosol run
```

### Apenas compilar
```bash
pordosol compilar
```

### Compilar para produção
```bash
pordosol producao
```

### Limpar build
```bash
pordosol clean
```

## Estrutura do projeto

- `src/` - Código fonte
- `build/` - Artefatos de build
- `pordosol.proj` - Configuração do projeto
"#,
            nome_projeto
        );

        fs::write(&readme, conteudo_readme)?;
        println!("Criado {}", readme.display());
    }

    println!("Projeto {} pronto em {}", template, raiz.display());
    Ok(())
}

fn carregar_configuracao_projeto(raiz: &Path) -> Option<serde_json::Value> {
    let projeto_file = raiz.join("pordosol.proj");
    if projeto_file.exists() {
        let conteudo = fs::read_to_string(&projeto_file).ok()?;
        serde_json::from_str(&conteudo).ok()
    } else {
        None
    }
}

fn info_cmd(caminho: &Path) -> Result<()> {
    let raiz = localizar_raiz(caminho);

    println!("=== Informações do Projeto ===");
    println!("Raiz do projeto: {}", raiz.display());

    // Informações do arquivo de projeto
    if let Some(config) = carregar_configuracao_projeto(&raiz) {
        if let Some(nome) = config.get("nome").and_then(|v| v.as_str()) {
            println!("Nome: {}", nome);
        }
        if let Some(tipo) = config.get("tipo").and_then(|v| v.as_str()) {
            println!("Tipo: {}", tipo);
        }
        if let Some(versao) = config.get("versao").and_then(|v| v.as_str()) {
            println!("Versão: {}", versao);
        }
        if let Some(descricao) = config.get("descricao").and_then(|v| v.as_str()) {
            println!("Descrição: {}", descricao);
        }
    } else {
        println!("Arquivo de projeto (pordosol.proj) não encontrado.");
    }

    // Arquivos .pr
    let arquivos = listar_prs(&raiz);
    println!("\nArquivos .pr encontrados: {}", arquivos.len());
    for arq in &arquivos {
        let rel_path = arq.strip_prefix(&raiz).unwrap_or(arq);
        println!("  - {}", rel_path.display());
    }

    // Status dos binários
    let (compilador, interpretador) = localizar_binarios(&raiz);
    println!("\n=== Ferramentas ===");
    println!(
        "Compilador: {} ({})",
        compilador.display(),
        if compilador.exists() {
            "✓"
        } else {
            "✗ não encontrado"
        }
    );
    println!(
        "Interpretador: {} ({})",
        interpretador.display(),
        if interpretador.exists() {
            "✓"
        } else {
            "✗ não encontrado"
        }
    );

    // Status do build
    let build_dir = raiz.join("build");
    if build_dir.exists() {
        let entries: Vec<_> = fs::read_dir(&build_dir)
            .unwrap_or_else(|_| fs::read_dir(".").unwrap())
            .filter_map(|e| e.ok())
            .collect();
        println!("\nPasta build/: {} arquivo(s)", entries.len());
    } else {
        println!("\nPasta build/: não existe");
    }

    Ok(())
}

fn listar_cmd(caminho: &Path, recentes: bool) -> Result<()> {
    let raiz = localizar_raiz(caminho);
    let arquivos = listar_prs(&raiz);

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
                        // mais de 1 dia
                        continue;
                    }

                    let size = metadata.len();
                    println!(
                        "  {} ({} bytes, modificado há {}s)",
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
        } else {
            if let Ok(metadata) = arq.metadata() {
                let size = metadata.len();
                println!("  {} ({} bytes)", rel_path.display(), size);
            } else {
                println!("  {}", rel_path.display());
            }
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
    let raiz = localizar_raiz(caminho_projeto);
    let proj_path = raiz.join("pordosol.proj");
    if !proj_path.exists() {
        bail!(
            "Arquivo de projeto não encontrado em {}",
            proj_path.display()
        );
    }
    let mut json: serde_json::Value = serde_json::from_str(&fs::read_to_string(&proj_path)?)?;
    let deps = json
        .get_mut("dependencias")
        .and_then(|d| d.as_object_mut())
        .ok_or_else(|| anyhow::anyhow!("Campo 'dependencias' ausente ou inválido"))?;

    match acao.to_ascii_lowercase().as_str() {
        "add" => {
            let nome = nome.ok_or_else(|| anyhow::anyhow!("Informe o nome da dependência"))?;
            if deps.contains_key(nome) {
                println!("Dependência '{}' já existe. Atualizando...", nome);
            }
            let valor = if let Some(c) = caminho_local {
                serde_json::json!({"path": c.to_string_lossy()})
            } else {
                let ver = versao.unwrap_or("*");
                serde_json::json!(ver)
            };
            deps.insert(nome.to_string(), valor);
            fs::write(&proj_path, serde_json::to_string_pretty(&json)?)?;
            println!("Dependência '{}' adicionada/atualizada.", nome);
        }
        "remove" | "rm" => {
            let nome = nome.ok_or_else(|| anyhow::anyhow!("Informe o nome da dependência"))?;
            if deps.remove(nome).is_some() {
                fs::write(&proj_path, serde_json::to_string_pretty(&json)?)?;
                println!("Dependência '{}' removida.", nome);
            } else {
                println!("Dependência '{}' não encontrada.", nome);
            }
        }
        "list" | "ls" | "listar" => {
            if deps.is_empty() {
                println!("Nenhuma dependência declarada.");
            } else {
                println!("Dependências:");
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
            bail!("Ação desconhecida: {} (use add|remove|list)", outra);
        }
    }
    Ok(())
}

fn compilar_cmd(caminho: &Path, target: &str, saida: Option<&Path>) -> Result<()> {
    let raiz = localizar_raiz(caminho);

    // Carregar configuração do projeto
    let config = carregar_configuracao_projeto(&raiz);

    // Usar target da configuração se não foi especificado e existe no projeto
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

    // Descobrir lista de arquivos .pr
    let arquivos: Vec<PathBuf> =
        if caminho.is_file() && caminho.extension() == Some(OsStr::new("pr")) {
            // Canonicaliza para evitar problemas de relativo após mudar current_dir
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
            "Compilador não encontrado em {}. Rode configurar-ambiente.ps1.",
            compilador.display()
        );
    }

    let saida_dir = saida
        .map(|p| p.to_path_buf())
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
        bail!("Compilação falhou (status {})", status);
    }

    println!("Compilado com sucesso. Saída em {}", saida_dir.display());

    // Mostrar arquivos gerados
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

fn run_cmd(caminho: &Path, force: bool, arquivo: Option<&Path>) -> Result<()> {
    run_unificado(caminho, force, arquivo)
}

fn run_unificado(caminho: &Path, force: bool, arquivo: Option<&Path>) -> Result<()> {
    let raiz = localizar_raiz(caminho);
    // Caso arquivo fornecido seja .pbc, apenas executa (compila se force ou inexistente fonte correspondente?)
    let arquivo_path = arquivo.map(|p| p.to_path_buf());

    // Determinar se modo somente execução de .pbc foi solicitado
    let somente_pbc = arquivo_path
        .as_ref()
        .map(|p| p.extension() == Some(OsStr::new("pbc")))
        .unwrap_or(false);

    // Coletar arquivos fonte
    let arquivos_fontes: Vec<PathBuf> = if somente_pbc {
        // tentar deduzir fonte correspondente para checar se recompila se force=true
        let v = listar_prs(&raiz);
        v
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

    // Verificar se os binários existem
    if !compilador.exists() {
        bail!(
            "Compilador não encontrado em {}. Rode configurar-ambiente.ps1.",
            compilador.display()
        );
    }
    if !interpretador.exists() {
        bail!(
            "Interpretador não encontrado em {}. Rode configurar-ambiente.ps1.",
            interpretador.display()
        );
    }

    let saida_dir = raiz.join("build");
    fs::create_dir_all(&saida_dir).ok();

    // Determinar se precisa recompilar
    // Determinar caminho do .pbc alvo
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
        && (force || !pbc.exists() || {
            // Verificar se algum .pr é mais novo que o .pbc
            let pbc_modified = pbc.metadata().ok().and_then(|m| m.modified().ok());
            arquivos_fontes.iter().any(|pr| {
                let pr_modified = pr.metadata().ok().and_then(|m| m.modified().ok());
                match (pbc_modified, pr_modified) {
                    (Some(pbc_time), Some(pr_time)) => pr_time > pbc_time,
                    _ => true, // Se não conseguir verificar, recompila
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
            bail!("Compilação falhou (status {})", status);
        }
        println!("Compilação concluída.");
    } else {
        println!("Bytecode está atualizado, pulando compilação...");
    }

    // Executar
    println!("Executando bytecode {}...", pbc.display());
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

fn clean_cmd(caminho: &Path) -> Result<()> {
    let raiz = localizar_raiz(caminho);
    let build_dir = raiz.join("build");

    if !build_dir.exists() {
        println!("Pasta build/ não existe em {}", raiz.display());
        return Ok(());
    }

    // Remover todos os arquivos na pasta build, mas manter a pasta
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
                .context(format!("Falha ao remover diretório {}", path.display()))?;
            count += 1;
        }
    }

    println!(
        "Limpeza concluída: {} item(s) removido(s) de {}",
        count,
        build_dir.display()
    );
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
