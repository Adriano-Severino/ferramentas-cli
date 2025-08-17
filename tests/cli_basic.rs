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

    // --versao
    let out = Command::new(&bin)
        .arg("--versao")
        .output()
        .expect("run versao");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("pordosol CLI v"));

    // --ajuda
    let out = Command::new(&bin)
        .arg("--ajuda")
        .output()
        .expect("run ajuda");
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("Compilar"));
}

#[test]
fn cria_projeto_novo() {
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
