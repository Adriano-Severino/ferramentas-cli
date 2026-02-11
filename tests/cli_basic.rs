use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn bin_path() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_BIN_EXE_pordosol"));
    if cfg!(windows) && p.extension().is_none() {
        p.set_extension("exe");
    }
    p
}

#[test]
fn mostra_versao_e_ajuda() {
    let bin = bin_path();

    let out = Command::new(&bin)
        .arg("--versao")
        .output()
        .expect("run versao");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("pordosol CLI v"));

    let out = Command::new(&bin)
        .arg("--ajuda")
        .output()
        .expect("run ajuda");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout).to_lowercase();
    assert!(s.contains("build"));
    assert!(s.contains("new"));
    assert!(s.contains("run"));
    assert!(s.contains("doctor"));
}

#[test]
fn cria_projeto_novo_modo_legado() {
    let bin = bin_path();
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path().join("meu_proj");

    let status = Command::new(&bin)
        .arg("novo")
        .arg(&dir)
        .status()
        .expect("run novo");
    assert!(status.success());

    let programa = dir.join("src").join("programa.pr");
    assert!(programa.exists(), "programa.pr deve existir");
}

#[test]
fn cria_projeto_new_estilo_dotnet() {
    let bin = bin_path();
    let temp = tempfile::tempdir().unwrap();
    let out = temp.path();

    let status = Command::new(&bin)
        .arg("new")
        .arg("console")
        .arg("-n")
        .arg("app")
        .arg("-o")
        .arg(out)
        .status()
        .expect("run new");
    assert!(status.success());

    let programa = out.join("app").join("src").join("programa.pr");
    assert!(programa.exists(), "programa.pr deve existir em output/nome");
}

#[test]
fn ajuda_build_expoe_project() {
    let bin = bin_path();

    let out = Command::new(&bin)
        .arg("build")
        .arg("--help")
        .output()
        .expect("run build --help");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout).to_lowercase();
    assert!(s.contains("--project"));
}

#[test]
fn ajuda_run_expoe_no_build() {
    let bin = bin_path();

    let out = Command::new(&bin)
        .arg("run")
        .arg("--help")
        .output()
        .expect("run run --help");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout).to_lowercase();
    assert!(s.contains("--no-build"));
}

#[test]
fn new_list_exibe_templates_disponiveis() {
    let bin = bin_path();

    let out = Command::new(&bin)
        .arg("new")
        .arg("list")
        .output()
        .expect("run new list");
    assert!(out.status.success());

    let s = String::from_utf8_lossy(&out.stdout).to_lowercase();
    assert!(s.contains("templates disponiveis"));
    assert!(s.contains("console"));
    assert!(s.contains("web"));
}

#[test]
fn new_console_renderiza_placeholders() {
    let bin = bin_path();
    let temp = tempfile::tempdir().unwrap();
    let out = temp.path();

    let status = Command::new(&bin)
        .arg("new")
        .arg("console")
        .arg("-n")
        .arg("meu_app")
        .arg("-o")
        .arg(out)
        .status()
        .expect("run new console");
    assert!(status.success());

    let projeto = out.join("meu_app").join("pordosol.proj");
    let programa = out.join("meu_app").join("src").join("programa.pr");
    let readme = out.join("meu_app").join("README.md");

    let projeto_txt = fs::read_to_string(projeto).unwrap();
    let programa_txt = fs::read_to_string(programa).unwrap();
    let readme_txt = fs::read_to_string(readme).unwrap();

    assert!(projeto_txt.contains("\"nome\": \"meu_app\""));
    assert!(projeto_txt.contains("\"target_padrao\": \"bytecode\""));
    assert!(programa_txt.contains("Namespace: Meu.App"));
    assert!(readme_txt.contains("`Meu.App`"));
}

#[test]
fn doctor_exibe_diagnostico() {
    let bin = bin_path();

    let out = Command::new(&bin)
        .arg("doctor")
        .output()
        .expect("run doctor");
    assert!(out.status.success());

    let s = String::from_utf8_lossy(&out.stdout).to_lowercase();
    assert!(s.contains("diagnostico"));
    assert!(s.contains("compilador"));
    assert!(s.contains("interpretador"));
    assert!(s.contains("biblioteca"));
}

#[test]
fn scripts_de_instalacao_existem() {
    let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(raiz.join("install.ps1").exists());
    assert!(raiz.join("install.sh").exists());
}

#[test]
fn workflows_e_scripts_release_existem() {
    let raiz = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(raiz
        .join(".github")
        .join("workflows")
        .join("ci.yml")
        .exists());
    assert!(raiz
        .join(".github")
        .join("workflows")
        .join("release.yml")
        .exists());
    assert!(raiz
        .join("scripts")
        .join("release")
        .join("package.sh")
        .exists());
    assert!(raiz
        .join("scripts")
        .join("release")
        .join("package.ps1")
        .exists());
    assert!(raiz.join("MATRIZ-COMPATIBILIDADE.md").exists());
    assert!(raiz.join("CHECKLIST-RELEASE.md").exists());
}
