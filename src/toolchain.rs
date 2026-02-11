use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use path_absolutize::Absolutize;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct DiagnosticoFerramenta {
    pub nome: String,
    pub caminho: PathBuf,
    pub origem: String,
    pub encontrado: bool,
}

#[derive(Clone, Debug)]
pub struct DiagnosticoToolchain {
    pub compilador: DiagnosticoFerramenta,
    pub interpretador: DiagnosticoFerramenta,
    pub stdlib: DiagnosticoFerramenta,
}

impl DiagnosticoToolchain {
    pub fn pronto(&self) -> bool {
        self.compilador.encontrado && self.interpretador.encontrado && self.stdlib.encontrado
    }
}

pub fn localizar_raiz(caminho: &Path) -> PathBuf {
    let mut p = caminho.absolutize().unwrap().to_path_buf();
    if p.is_file() {
        if let Some(parent) = p.parent() {
            p = parent.to_path_buf();
        }
    }

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

pub fn listar_prs(raiz: &Path) -> Vec<PathBuf> {
    let src = raiz.join("src");
    let mut arquivos: Vec<PathBuf> = WalkDir::new(&src)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| p.is_file() && p.extension() == Some(OsStr::new("pr")))
        .collect();

    let preferido = src.join("programa.pr");
    if let Some(pos) = arquivos.iter().position(|p| p == &preferido) {
        let pref = arquivos.remove(pos);
        arquivos.insert(0, pref);
    }
    arquivos
}

pub fn localizar_binarios(raiz: &Path) -> (PathBuf, PathBuf) {
    let diag = diagnosticar_toolchain(raiz);
    (diag.compilador.caminho, diag.interpretador.caminho)
}

pub fn diagnosticar_toolchain(raiz: &Path) -> DiagnosticoToolchain {
    DiagnosticoToolchain {
        compilador: localizar_executavel("compilador", "PORDOSOL_COMPILADOR_PATH", raiz),
        interpretador: localizar_executavel("interpretador", "PORDOSOL_INTERPRETADOR_PATH", raiz),
        stdlib: localizar_stdlib_diagnostico(raiz),
    }
}

pub fn detectar_versao_binario(caminho: &Path) -> Option<String> {
    if !caminho.is_file() {
        return None;
    }

    for flag in ["--versao", "--version", "-V"] {
        if let Ok(out) = Command::new(caminho).arg(flag).output() {
            let mut texto = String::from_utf8_lossy(&out.stdout).to_string();
            if !out.stderr.is_empty() {
                if !texto.is_empty() {
                    texto.push('\n');
                }
                texto.push_str(&String::from_utf8_lossy(&out.stderr));
            }
            if let Some(v) = extrair_versao(&texto) {
                return Some(v);
            }
        }
    }

    None
}

pub fn carregar_configuracao_projeto(raiz: &Path) -> Option<serde_json::Value> {
    let projeto_file = raiz.join("pordosol.proj");
    if projeto_file.exists() {
        let conteudo = fs::read_to_string(&projeto_file).ok()?;
        serde_json::from_str(&conteudo).ok()
    } else {
        None
    }
}

fn localizar_executavel(nome_base: &str, variavel_env: &str, raiz: &Path) -> DiagnosticoFerramenta {
    let nome_exec = nome_executavel(nome_base);
    let mut primeira_falha: Option<DiagnosticoFerramenta> = None;

    if let Some(path) = ler_env_path(variavel_env) {
        if path.is_file() {
            return ok(nome_base, path, format!("env:{}", variavel_env));
        }
        primeira_falha.get_or_insert_with(|| {
            falha(nome_base, path, format!("env:{} (invalido)", variavel_env))
        });
    }

    for path in caminhos_tools_instalacao(&nome_exec) {
        if path.is_file() {
            return ok(nome_base, path, "instalacao-cli/tools".to_string());
        }
        primeira_falha.get_or_insert_with(|| {
            falha(
                nome_base,
                path,
                "instalacao-cli/tools (ausente)".to_string(),
            )
        });
    }

    if let Some(path) = caminho_pordosol_home_tools(&nome_exec) {
        if path.is_file() {
            return ok(nome_base, path, "env:PORDOSOL_HOME/tools".to_string());
        }
        primeira_falha.get_or_insert_with(|| {
            falha(
                nome_base,
                path,
                "env:PORDOSOL_HOME/tools (ausente)".to_string(),
            )
        });
    }

    if let Ok(path) = which::which(&nome_exec) {
        return ok(nome_base, path, "PATH".to_string());
    }
    primeira_falha
        .get_or_insert_with(|| falha(nome_base, PathBuf::from(&nome_exec), "PATH".to_string()));

    for path in candidatos_lib_local(raiz, &nome_exec) {
        if path.is_file() {
            return ok(nome_base, path, "fallback:./lib".to_string());
        }
        primeira_falha
            .get_or_insert_with(|| falha(nome_base, path, "fallback:./lib (ausente)".to_string()));
    }

    primeira_falha.unwrap_or_else(|| {
        falha(
            nome_base,
            PathBuf::from(&nome_exec),
            "nao resolvido".to_string(),
        )
    })
}

fn localizar_stdlib_diagnostico(raiz: &Path) -> DiagnosticoFerramenta {
    let mut primeira_falha: Option<DiagnosticoFerramenta> = None;

    for var in ["PORDOSOL_STDLIB_PATH", "PORDOSOL_BIBLIOTECA_PADRAO_PATH"] {
        if let Some(path) = ler_env_path(var) {
            if eh_stdlib_valida(&path) {
                return ok("biblioteca padrao", path, format!("env:{}", var));
            }
            primeira_falha.get_or_insert_with(|| {
                falha("biblioteca padrao", path, format!("env:{} (invalido)", var))
            });
        }
    }

    for candidato in ["stdlib", "sistema-padrao"] {
        for path in caminhos_tools_instalacao(candidato) {
            if eh_stdlib_valida(&path) {
                return ok(
                    "biblioteca padrao",
                    path,
                    format!("instalacao-cli/tools/{}", candidato),
                );
            }
            primeira_falha.get_or_insert_with(|| {
                falha(
                    "biblioteca padrao",
                    path,
                    format!("instalacao-cli/tools/{} (ausente)", candidato),
                )
            });
        }
    }

    if let Some(path) = caminho_pordosol_home_tools("stdlib") {
        if eh_stdlib_valida(&path) {
            return ok(
                "biblioteca padrao",
                path,
                "env:PORDOSOL_HOME/tools/stdlib".to_string(),
            );
        }
        primeira_falha.get_or_insert_with(|| {
            falha(
                "biblioteca padrao",
                path,
                "env:PORDOSOL_HOME/tools/stdlib (ausente)".to_string(),
            )
        });
    }

    if let Some(path) = caminho_pordosol_home_tools("sistema-padrao") {
        if eh_stdlib_valida(&path) {
            return ok(
                "biblioteca padrao",
                path,
                "env:PORDOSOL_HOME/tools/sistema-padrao".to_string(),
            );
        }
    }

    for path in candidatos_stdlib_local(raiz) {
        if eh_stdlib_valida(&path) {
            return ok("biblioteca padrao", path, "fallback:local".to_string());
        }
        primeira_falha.get_or_insert_with(|| {
            falha(
                "biblioteca padrao",
                path,
                "fallback:local (ausente)".to_string(),
            )
        });
    }

    primeira_falha.unwrap_or_else(|| {
        falha(
            "biblioteca padrao",
            raiz.join("sistema-padrao"),
            "nao resolvido".to_string(),
        )
    })
}

fn extrair_versao(texto: &str) -> Option<String> {
    if let Some(pos) = texto.find("(v") {
        let sub = &texto[pos + 1..];
        if let Some(end) = sub.find(')') {
            return Some(sub[..end].to_string());
        }
    }

    let chars: Vec<char> = texto.chars().collect();
    if chars.len() < 2 {
        return None;
    }

    for i in 0..(chars.len() - 1) {
        let c = chars[i];
        let prox = chars[i + 1];
        if (c == 'v' || c == 'V') && prox.is_ascii_digit() {
            let mut j = i + 1;
            while j < chars.len()
                && (chars[j].is_ascii_digit() || chars[j] == '.' || chars[j] == '-')
            {
                j += 1;
            }
            let token: String = chars[i..j].iter().collect();
            return Some(token);
        }
    }

    None
}

fn ok(nome: &str, caminho: PathBuf, origem: String) -> DiagnosticoFerramenta {
    DiagnosticoFerramenta {
        nome: nome.to_string(),
        caminho,
        origem,
        encontrado: true,
    }
}

fn falha(nome: &str, caminho: PathBuf, origem: String) -> DiagnosticoFerramenta {
    DiagnosticoFerramenta {
        nome: nome.to_string(),
        caminho,
        origem,
        encontrado: false,
    }
}

fn ler_env_path(nome: &str) -> Option<PathBuf> {
    let valor = std::env::var(nome).ok()?;
    let valor = valor.trim();
    if valor.is_empty() {
        return None;
    }
    Some(PathBuf::from(valor))
}

fn caminhos_tools_instalacao(nome: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(base) = exe.parent() {
            out.push(base.join("tools").join(nome));
            if let Some(parent) = base.parent() {
                out.push(parent.join("tools").join(nome));
            }
        }
    }
    out
}

fn caminho_pordosol_home_tools(nome: &str) -> Option<PathBuf> {
    let home = ler_env_path("PORDOSOL_HOME")?;
    Some(home.join("tools").join(nome))
}

fn candidatos_lib_local(raiz: &Path, nome_exec: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut atual = raiz.to_path_buf();
    for _ in 0..6 {
        out.push(atual.join("lib").join(nome_exec));
        if let Some(parent) = atual.parent() {
            atual = parent.to_path_buf();
        } else {
            break;
        }
    }
    out
}

fn candidatos_stdlib_local(raiz: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut atual = raiz.to_path_buf();
    for _ in 0..6 {
        out.push(atual.join("lib").join("stdlib"));
        out.push(atual.join("lib").join("sistema-padrao"));
        out.push(atual.join("stdlib"));
        out.push(atual.join("sistema-padrao"));
        if let Some(parent) = atual.parent() {
            atual = parent.to_path_buf();
        } else {
            break;
        }
    }
    out
}

fn eh_stdlib_valida(path: &Path) -> bool {
    path.is_dir() && (path.join("Sistema.toml").is_file() || path.join("src").is_dir())
}

fn nome_executavel(nome: &str) -> String {
    if cfg!(windows) {
        format!("{}.exe", nome)
    } else {
        nome.to_string()
    }
}
