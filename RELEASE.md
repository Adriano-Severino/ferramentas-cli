# Release E CI

Este projeto possui dois workflows:

- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`

## CI

Executa build e testes em:

- `ubuntu-latest`
- `windows-latest`
- `macos-latest`

## Release

Publica release automaticamente ao criar tag `v*`.

Exemplo:

```bash
git tag v0.1.0
git push origin v0.1.0
```

O workflow gera artefatos versionados por plataforma:

- `pordosol-<versao>-linux-x64.tar.gz`
- `pordosol-<versao>-macos-x64.tar.gz`
- `pordosol-<versao>-windows-x64.zip`

E também:

- checksums `.sha256`
- `SHA256SUMS.txt` consolidado
- assinaturas `.sig` quando `COSIGN_PRIVATE_KEY` estiver configurada nos secrets

## Validacoes De Versao

O workflow `release` valida:

- formato semver da versao (`X.Y.Z`)
- consistencia entre tag (`vX.Y.Z`) e versao de release
- consistencia entre versao de release e `Cargo.toml`

Se houver divergencia, o workflow falha antes de publicar.

## Checklist

Use `CHECKLIST-RELEASE.md` antes de criar a tag.
