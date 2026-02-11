use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::{bail, Context, Result};
use path_absolutize::Absolutize;
use walkdir::WalkDir;

struct TemplateVars {
    project_name: String,
    namespace: String,
    target: String,
}

pub fn listar_templates_cmd() -> Result<()> {
    let templates = listar_templates_disponiveis()?;
    if templates.is_empty() {
        println!("Nenhum template encontrado.");
        return Ok(());
    }

    println!("Templates disponiveis:");
    for template in templates {
        println!("  {}", template);
    }
    Ok(())
}

pub fn novo_cmd(destino: &Path, nao_sobrescrever: bool, template: &str) -> Result<()> {
    let raiz = destino
        .absolutize()
        .context("Falha ao resolver caminho do projeto")?
        .to_path_buf();
    fs::create_dir_all(&raiz).context("Falha ao criar pasta do projeto")?;
    fs::create_dir_all(raiz.join("build")).ok();

    let template_final = template.trim().to_ascii_lowercase();
    if template_final.is_empty() {
        bail!("Template invalido. Informe um tipo com `pordosol new list`.");
    }

    let nome_projeto = raiz
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let vars = TemplateVars {
        project_name: nome_projeto,
        namespace: gerar_namespace(&raiz),
        target: target_padrao(&template_final).to_string(),
    };

    if aplicar_template_em_arquivos(&raiz, nao_sobrescrever, &template_final, &vars)? {
        println!("Projeto {} pronto em {}", template_final, raiz.display());
        return Ok(());
    }

    if aplicar_template_legado(&raiz, nao_sobrescrever, &template_final)? {
        println!("Projeto {} pronto em {}", template_final, raiz.display());
        return Ok(());
    }

    let disponiveis = listar_templates_disponiveis()?;
    if disponiveis.is_empty() {
        bail!(
            "Template '{}' nao encontrado e nenhum template foi detectado.",
            template_final
        );
    }
    bail!(
        "Template '{}' nao encontrado. Use `pordosol new list` para ver os disponiveis.",
        template_final
    );
}

fn aplicar_template_em_arquivos(
    destino: &Path,
    nao_sobrescrever: bool,
    template: &str,
    vars: &TemplateVars,
) -> Result<bool> {
    let Some(templates_root) = localizar_diretorio_templates() else {
        return Ok(false);
    };

    let template_dir = templates_root.join(template);
    if !template_dir.is_dir() {
        return Ok(false);
    }

    for entry in WalkDir::new(&template_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
    {
        let origem = entry.path();
        let rel = origem
            .strip_prefix(&template_dir)
            .context("Falha ao resolver caminho relativo do template")?;
        let destino_rel = renderizar_caminho_relativo(rel, vars);
        let arquivo_destino = destino.join(destino_rel);

        if arquivo_destino.exists() && nao_sobrescrever {
            println!(
                "Arquivo {} ja existe (nao sobrescrito).",
                arquivo_destino.display()
            );
            continue;
        }

        if let Some(parent) = arquivo_destino.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Falha ao criar diretorio de destino {}", parent.display())
            })?;
        }

        copiar_ou_renderizar_arquivo(origem, &arquivo_destino, vars)?;
        println!("Criado {}", arquivo_destino.display());
    }

    Ok(true)
}

fn copiar_ou_renderizar_arquivo(origem: &Path, destino: &Path, vars: &TemplateVars) -> Result<()> {
    let bytes = fs::read(origem)
        .with_context(|| format!("Falha ao ler arquivo de template {}", origem.display()))?;

    match String::from_utf8(bytes.clone()) {
        Ok(texto) => {
            let renderizado = substituir_placeholders(&texto, vars);
            fs::write(destino, renderizado)
                .with_context(|| format!("Falha ao escrever arquivo {}", destino.display()))?;
        }
        Err(_) => {
            fs::write(destino, bytes)
                .with_context(|| format!("Falha ao copiar arquivo {}", destino.display()))?;
        }
    }

    Ok(())
}

fn renderizar_caminho_relativo(rel: &Path, vars: &TemplateVars) -> PathBuf {
    let mut out = PathBuf::new();

    for componente in rel.components() {
        if let Component::Normal(nome) = componente {
            let nome = nome.to_string_lossy();
            let mut renderizado = substituir_placeholders(&nome, vars);
            if renderizado.ends_with(".tpl") {
                renderizado.truncate(renderizado.len() - ".tpl".len());
            }
            out.push(renderizado);
        }
    }

    out
}

fn substituir_placeholders(valor: &str, vars: &TemplateVars) -> String {
    valor
        .replace("{{PROJECT_NAME}}", &vars.project_name)
        .replace("{{NAMESPACE}}", &vars.namespace)
        .replace("{{TARGET}}", &vars.target)
}

fn listar_templates_disponiveis() -> Result<Vec<String>> {
    if let Some(templates_root) = localizar_diretorio_templates() {
        let mut templates = fs::read_dir(templates_root)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.file_name().to_string_lossy().to_string())
            .collect::<Vec<_>>();
        templates.sort();
        return Ok(templates);
    }

    Ok(vec![
        "biblioteca".to_string(),
        "classe".to_string(),
        "console".to_string(),
        "web".to_string(),
    ])
}

fn localizar_diretorio_templates() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("PORDOSOL_TEMPLATES_PATH") {
        let p = PathBuf::from(path);
        if p.is_dir() {
            return Some(p);
        }
    }

    if let Ok(home) = std::env::var("PORDOSOL_HOME") {
        let p = PathBuf::from(home).join("templates");
        if p.is_dir() {
            return Some(p);
        }
    }

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let templates = exe_dir.join("templates");
            if templates.is_dir() {
                return Some(templates);
            }
            if let Some(parent) = exe_dir.parent() {
                let templates = parent.join("templates");
                if templates.is_dir() {
                    return Some(templates);
                }
            }
        }
    }

    let templates_local = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");
    if templates_local.is_dir() {
        return Some(templates_local);
    }

    None
}

fn target_padrao(template: &str) -> &'static str {
    match template {
        "biblioteca" => "llvm-ir",
        _ => "bytecode",
    }
}

fn gerar_namespace(destino: &Path) -> String {
    let nome = destino
        .file_name()
        .unwrap_or_else(|| OsStr::new("Projeto"))
        .to_string_lossy()
        .to_string();

    let partes = nome
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|parte| !parte.is_empty())
        .map(formatar_token_namespace)
        .collect::<Vec<_>>();

    if partes.is_empty() {
        return "Projeto".to_string();
    }

    partes.join(".")
}

fn formatar_token_namespace(token: &str) -> String {
    let mut chars = token.chars().filter(|c| c.is_ascii_alphanumeric());
    let Some(first) = chars.next() else {
        return "Projeto".to_string();
    };

    let mut out = String::new();
    if first.is_ascii_digit() {
        out.push('_');
        out.push(first);
    } else {
        out.push(first.to_ascii_uppercase());
    }

    for c in chars {
        out.push(c.to_ascii_lowercase());
    }

    out
}

fn aplicar_template_legado(destino: &Path, nao_sobrescrever: bool, template: &str) -> Result<bool> {
    match template {
        "console" | "web" | "biblioteca" | "classe" => {}
        _ => return Ok(false),
    }

    fs::create_dir_all(destino.join("src")).ok();
    fs::create_dir_all(destino.join("build")).ok();

    let nome_projeto = destino
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let projeto_file = destino.join("pordosol.proj");
    if !projeto_file.exists() || !nao_sobrescrever {
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
            "web" => format!(
                r#"{{
    "nome": "{}",
    "tipo": "web",
    "versao": "1.0.0",
    "descricao": "Uma aplicacao web em Por do Sol",
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
    "descricao": "Uma aplicacao console em Por do Sol",
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

    let prog = destino.join("src").join("programa.pr");
    if prog.exists() && nao_sobrescrever {
        println!("Projeto ja contem src/programa.pr (nao sobrescrito).");
    } else if !prog.exists() || !nao_sobrescrever {
        let exemplo = match template {
            "biblioteca" => {
                r#"// biblioteca.pr - template de biblioteca
usando Sistema.IO;

classe publica MinhaClasse
{
    inteiro valor { get; set; }

    publico MinhaClasse(inteiro valorInicial)
    {
        este.valor = valorInicial;
    }

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
    texto nome { get; set; }
    inteiro idade { get; set; }

    publico MinhaClasse(texto nome, inteiro idade)
    {
        este.nome = nome;
        este.idade = idade;
    }

    publico vazio ApresentarSe()
    {
        imprima($"Ola, eu sou {este.nome} e tenho {este.idade} anos.");
    }
}

funcao vazio Principal()
{
    var pessoa = novo MinhaClasse("Joao", 25);
    pessoa.ApresentarSe();
}
"#
            }
            "web" => {
                r#"// programa.pr - template web inicial
funcao vazio Principal()
{
    imprima("Projeto web Por do Sol criado.");
    imprima("Proximo passo: configure rotas e servidor no seu framework web.");
}
"#
            }
            _ => {
                r#"// programa.pr - exemplo inicial
funcao vazio Principal()
{
    imprima("Ola, Por do Sol!");

    var nome = "Mundo";
    var numero = 42;

    imprima($"Ola, {nome}! O numero e {numero}");
}
"#
            }
        };

        fs::write(&prog, exemplo)?;
        println!("Criado {}", prog.display());
    }

    let readme = destino.join("README.md");
    if !readme.exists() || !nao_sobrescrever {
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
pordosol build
```

### Compilar para producao
```bash
pordosol producao
```

### Limpar build
```bash
pordosol clean
```

## Estrutura do projeto

- `src/` - Codigo fonte
- `build/` - Artefatos de build
- `pordosol.proj` - Configuracao do projeto
"#,
            nome_projeto
        );

        fs::write(&readme, conteudo_readme)?;
        println!("Criado {}", readme.display());
    }

    Ok(true)
}
