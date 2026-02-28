# Testes E2E para `pordosol`

Este diretório contém scripts simples para validar o fluxo `new -> build -> run` usando o CLI `pordosol`.

Pré-requisitos:
- `pordosol` instalado e disponível no `PATH`, ou o binário `pordosol` presente no diretório corrente.

Execução (Linux/macOS):

```bash
chmod +x e2e_new_build_run.sh
./e2e_new_build_run.sh
```

Execução (Windows PowerShell):

```powershell
.\e2e_new_build_run.ps1
```

O teste cria um projeto temporário chamado `testapp`, executa `pordosol build` e `pordosol run --no-build`.

Próximos passos recomendados:
- Integrar estes scripts a um workflow do GitHub Actions para execução automática em PRs.
- Ajustar para usar o binário local durante desenvolvimento (ex.: `cargo run -p ferramentas-cli --`).
