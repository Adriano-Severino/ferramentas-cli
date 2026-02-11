use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn bin_path() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_BIN_EXE_pordosol"));
    if cfg!(windows) && p.extension().is_none() {
        p.set_extension("exe");
    }
    p
}

fn criar_toolchain_fake(dir: &Path) -> (PathBuf, PathBuf) {
    fs::create_dir_all(dir).unwrap();

    #[cfg(windows)]
    {
        let compilador = dir.join("compilador.cmd");
        let interpretador = dir.join("interpretador.cmd");

        let compilador_script = r#"@echo off
setlocal EnableDelayedExpansion
for %%A in (%*) do (
  if /I "%%~xA"==".pr" (
    > "%%~nA.pbc" echo fake-bytecode
  )
)
exit /b 0
"#;
        let interpretador_script = r#"@echo off
if "%~1"=="" exit /b 1
if not exist "%~1" exit /b 1
echo [fake interpreter] %~1
exit /b 0
"#;

        fs::write(&compilador, compilador_script).unwrap();
        fs::write(&interpretador, interpretador_script).unwrap();
        return (compilador, interpretador);
    }

    #[cfg(not(windows))]
    {
        use std::os::unix::fs::PermissionsExt;

        let compilador = dir.join("compilador");
        let interpretador = dir.join("interpretador");

        let compilador_script = r#"#!/usr/bin/env bash
set -euo pipefail
for arg in "$@"; do
  case "$arg" in
    *.pr)
      stem="$(basename "${arg%.*}")"
      printf "fake-bytecode\n" > "${stem}.pbc"
      ;;
  esac
done
"#;
        let interpretador_script = r#"#!/usr/bin/env bash
set -euo pipefail
[[ -n "${1:-}" ]]
[[ -f "$1" ]]
echo "[fake interpreter] $1"
"#;

        fs::write(&compilador, compilador_script).unwrap();
        fs::write(&interpretador, interpretador_script).unwrap();

        let mut p1 = fs::metadata(&compilador).unwrap().permissions();
        p1.set_mode(0o755);
        fs::set_permissions(&compilador, p1).unwrap();

        let mut p2 = fs::metadata(&interpretador).unwrap().permissions();
        p2.set_mode(0o755);
        fs::set_permissions(&interpretador, p2).unwrap();

        (compilador, interpretador)
    }
}

fn copiar_diretorio(origem: &Path, destino: &Path) {
    fs::create_dir_all(destino).unwrap();
    for entry in fs::read_dir(origem).unwrap() {
        let entry = entry.unwrap();
        let origem_path = entry.path();
        let destino_path = destino.join(entry.file_name());
        if origem_path.is_dir() {
            copiar_diretorio(&origem_path, &destino_path);
        } else {
            fs::copy(&origem_path, &destino_path).unwrap();
        }
    }
}

fn extensao_executavel() -> &'static str {
    if cfg!(windows) {
        ".exe"
    } else {
        ""
    }
}

#[test]
fn e2e_console_new_build_run() {
    let bin = bin_path();
    let temp = tempfile::tempdir().unwrap();
    let workspace = temp.path().join("workspace");
    let tools = temp.path().join("fake-tools");
    let (compilador, interpretador) = criar_toolchain_fake(&tools);

    let status_new = Command::new(&bin)
        .arg("new")
        .arg("console")
        .arg("-n")
        .arg("app")
        .arg("-o")
        .arg(&workspace)
        .status()
        .expect("run new");
    assert!(status_new.success());

    let projeto = workspace.join("app");
    let status_build = Command::new(&bin)
        .arg("build")
        .arg("--project")
        .arg(&projeto)
        .env("PORDOSOL_COMPILADOR_PATH", &compilador)
        .env("PORDOSOL_INTERPRETADOR_PATH", &interpretador)
        .status()
        .expect("run build");
    assert!(status_build.success());

    let pbc = projeto.join("build").join("programa.pbc");
    assert!(pbc.exists(), "build/programa.pbc deve existir");

    let status_run = Command::new(&bin)
        .arg("run")
        .arg("--project")
        .arg(&projeto)
        .arg("--no-build")
        .env("PORDOSOL_COMPILADOR_PATH", &compilador)
        .env("PORDOSOL_INTERPRETADOR_PATH", &interpretador)
        .status()
        .expect("run run");
    assert!(status_run.success());
}

#[test]
fn e2e_web_new_build_run() {
    let bin = bin_path();
    let temp = tempfile::tempdir().unwrap();
    let workspace = temp.path().join("workspace");
    let tools = temp.path().join("fake-tools");
    let (compilador, interpretador) = criar_toolchain_fake(&tools);

    let status_new = Command::new(&bin)
        .arg("new")
        .arg("web")
        .arg("-n")
        .arg("site")
        .arg("-o")
        .arg(&workspace)
        .status()
        .expect("run new web");
    assert!(status_new.success());

    let projeto = workspace.join("site");
    let status_build = Command::new(&bin)
        .arg("build")
        .arg("--project")
        .arg(&projeto)
        .env("PORDOSOL_COMPILADOR_PATH", &compilador)
        .env("PORDOSOL_INTERPRETADOR_PATH", &interpretador)
        .status()
        .expect("run build web");
    assert!(status_build.success());

    let pbc = projeto.join("build").join("programa.pbc");
    assert!(pbc.exists(), "build/programa.pbc deve existir");

    let status_run = Command::new(&bin)
        .arg("run")
        .arg("--project")
        .arg(&projeto)
        .arg("--no-build")
        .env("PORDOSOL_COMPILADOR_PATH", &compilador)
        .env("PORDOSOL_INTERPRETADOR_PATH", &interpretador)
        .status()
        .expect("run run web");
    assert!(status_run.success());
}

#[test]
fn e2e_instalacao_layout_limpo() {
    let bin = bin_path();
    let temp = tempfile::tempdir().unwrap();
    let install_root = temp.path().join("pordosol");
    let bin_dir = install_root.join("bin");
    let tools_dir = install_root.join("tools");
    let templates_dir = install_root.join("templates");

    fs::create_dir_all(&bin_dir).unwrap();
    fs::create_dir_all(&tools_dir).unwrap();
    fs::create_dir_all(&templates_dir).unwrap();

    let bin_destino = bin_dir.join(format!("pordosol{}", extensao_executavel()));
    fs::copy(&bin, &bin_destino).unwrap();

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    copiar_diretorio(&manifest_dir.join("templates"), &templates_dir);

    let comp_path = tools_dir.join(format!("compilador{}", extensao_executavel()));
    let interp_path = tools_dir.join(format!("interpretador{}", extensao_executavel()));
    fs::write(&comp_path, b"stub").unwrap();
    fs::write(&interp_path, b"stub").unwrap();

    let stdlib = tools_dir.join("stdlib");
    fs::create_dir_all(&stdlib).unwrap();
    fs::write(stdlib.join("Sistema.toml"), "nome = \"stdlib\"").unwrap();

    let out_doctor = Command::new(&bin_destino)
        .arg("doctor")
        .env("PORDOSOL_HOME", &install_root)
        .env_remove("PORDOSOL_COMPILADOR_PATH")
        .env_remove("PORDOSOL_INTERPRETADOR_PATH")
        .env_remove("PORDOSOL_STDLIB_PATH")
        .output()
        .expect("run doctor em layout limpo");
    assert!(out_doctor.status.success());
    let s = String::from_utf8_lossy(&out_doctor.stdout).to_lowercase();
    assert!(s.contains("resultado: ambiente pronto"));

    let out_list = Command::new(&bin_destino)
        .arg("new")
        .arg("list")
        .env("PORDOSOL_HOME", &install_root)
        .output()
        .expect("run new list em layout limpo");
    assert!(out_list.status.success());
    let s = String::from_utf8_lossy(&out_list.stdout).to_lowercase();
    assert!(s.contains("console"));
    assert!(s.contains("web"));
}
