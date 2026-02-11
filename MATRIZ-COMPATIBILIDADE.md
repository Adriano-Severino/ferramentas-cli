# Matriz De Compatibilidade Por do Sol

Data de validacao: 2026-02-11

## Componentes

| Componente | Versao alvo | Papel no ecossistema | Status |
| --- | --- | --- | --- |
| `ferramentas-cli` | `0.1.x` | Comandos `new/build/run/doctor` e instalacao | Validado |
| `compilador-portugues` | `0.1.x` | Binarios `compilador` e `interpretador` | Validado (contrato de CLI) |
| `sistema-padrao` | `main` (estrutura com `Sistema.toml`/`src`) | Biblioteca padrao | Validado (deteccao por `doctor`) |
| `pordosol-language-server` | `main` | Suporte de editor/LSP | Compatibilidade declarada |

## Contrato Minimo De Compatibilidade

- O CLI deve localizar:
1. `compilador` (ou `compilador.exe`)
2. `interpretador` (ou `interpretador.exe`)
3. stdlib valida em `tools/stdlib` ou caminho configurado
- Os fluxos devem funcionar:
1. `new -> build -> run` para template `console`
2. `new -> build -> run` para template `web`
- Em layout instalado (`bin/tools/templates`), o comando `doctor` deve retornar ambiente pronto.

## Verificacao Recomendada

No repositório `ferramentas-cli`:

```bash
cargo test
pordosol doctor
pordosol new console -n app -o .
pordosol build --project ./app
pordosol run --project ./app --no-build
```

## Observacoes

- `pordosol-language-server` nao e acoplado ao pipeline de build/run, entao a compatibilidade e de protocolo e extensao de sintaxe.
- Ao subir versao major do compilador, atualizar esta matriz e repetir os testes E2E.
