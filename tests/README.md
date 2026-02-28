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

Artefatos e logs
-----------------

Os scripts geram um arquivo de log `e2e.log`. Quando executados pelo workflow do GitHub Actions com `KEEP_ARTIFACTS=1`, os artefatos são copiados para um diretório no workspace com padrão `ferramentas-cli-e2e-artifacts-<runId>` e o workflow faz o upload desses artefatos.

Exemplo de saída de log (trecho):

```
[pordosol-e2e] inicio do teste: 2026-02-28T12:34:56Z
[pordosol-e2e] usando tmpdir: /tmp/tmpXyz
[pordosol-e2e] criando novo projeto 'testapp'
Created project: testapp
[pordosol-e2e] build
Build succeeded
[pordosol-e2e] run (com --no-build)
Hello from Pordosol app!
[pordosol-e2e] SUCESSO: new -> build -> run concluído
[pordosol-e2e] fim do teste: 2026-02-28T12:35:10Z
```

Notificações (opcional)
------------------------

O workflow pode enviar notificações externas (Slack, email, etc.) usando secrets do repositório. Exemplo simples para postar no Slack usando um webhook (adicionar `SLACK_WEBHOOK` em Settings → Secrets):

```yaml
- name: Notify Slack
	if: failure() || success()
	run: |
		payload="{\"text\": \"E2E concluído: ${{ github.workflow }} #${{ github.run_number }} — ${{ job.status }}\" }"
		curl -s -X POST -H 'Content-type: application/json' --data "$payload" "$SLACK_WEBHOOK"
	env:
		SLACK_WEBHOOK: ${{ secrets.SLACK_WEBHOOK }}
```

Para enviar emails, considere usar uma ação que suporte SMTP e mantenha as credenciais em secrets (`SMTP_HOST`, `SMTP_USER`, `SMTP_PASS`). Outra opção é integrar com plataformas de notificação (Teams, Discord, etc.) via webhooks.

Boas práticas
------------

- Limite a retenção dos artefatos (no workflow usamos `retention-days: 7`) para economizar espaço.
- Não exponha credenciais nos logs; use `secrets` do GitHub Actions.
- Se quiser logs mais detalhados, exporte variáveis de ambiente do `pordosol` para modo verbose e capture a saída completa no `e2e.log`.

