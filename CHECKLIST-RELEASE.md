# Checklist De Release

## Antes Da Tag

1. Confirmar que `Cargo.toml` tem a versao final desejada.
2. Executar:
```bash
cargo fmt
cargo test
```
3. Garantir que docs de release/instalacao estao atualizados.
4. Revisar `MATRIZ-COMPATIBILIDADE.md`.

## Publicacao

1. Criar tag no formato `vX.Y.Z`:
```bash
git tag v0.1.0
git push origin v0.1.0
```
2. Acompanhar workflow `release`.
3. Validar se artefatos foram anexados:
- `.zip` (Windows)
- `.tar.gz` (Linux/macOS)
- `.sha256`
- `SHA256SUMS.txt`
- `.sig` (quando cosign estiver configurado)

## Pos-Release

1. Testar instalacao com `install.ps1` e `install.sh`.
2. Executar `pordosol doctor` em ambiente limpo.
3. Registrar eventual nota de migracao.
