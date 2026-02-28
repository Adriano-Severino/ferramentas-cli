# Plano De Evolucao Do `pordosol` CLI (Estilo Dotnet)

## Objetivo
Entregar uma experiencia de instalacao e uso equivalente ao `dotnet`:
- instalou uma vez, funciona em qualquer pasta
- `pordosol new`, `pordosol build`, `pordosol run`
- templates oficiais (console/web/biblioteca)
- toolchain (compilador + interpretador + biblioteca padrao) descoberta automaticamente

## Estado Atual (Resumo)
- A CLI ja possui comandos base: `novo`, `compilar`, `run`, `producao`, `clean`, `info`, `listar`, `dep`.
- A implementacao principal ainda esta concentrada em `src/main.rs`.
- O compilador e o interpretador existem como bins separados no projeto `compilador-portugues`.
- A biblioteca padrao `sistema-padrao` tem estrutura de modulos consolidada, mas com scripts ainda desalinhados.
- O framework `net-por-do-sol` ainda esta parcialmente implementado.

## Arquitetura Alvo
- Executavel unico: `pordosol`
- Resolucao de toolchain por prioridade:
1. variaveis de ambiente (`PORDOSOL_COMPILADOR_PATH`, `PORDOSOL_INTERPRETADOR_PATH`)
2. caminho ao lado da instalacao do CLI (`tools/`)
3. `PORDOSOL_HOME/tools`
4. `PATH`
5. fallback local (`./lib`) para desenvolvimento
- Templates versionados:
1. `console`
2. `web`
3. `biblioteca`
- Diagnostico de ambiente com `pordosol doctor`

## Fases

### Fase 1 - Refatoracao Interna Da CLI (sem mudar comportamento)
Objetivo: quebrar o monolito de `main.rs` em modulos para facilitar evolucao segura.

Entregas:
- Extrair logica de comandos para modulos:
1. `src/novo.rs`
2. `src/construir.rs`
3. `src/executar.rs`
- Criar utilitarios compartilhados em `src/toolchain.rs`.
- Manter compatibilidade total de comandos e parametros atuais.
- Validar com testes (`cargo test`) e fluxo manual basico.

Status: **Concluida**.

### Fase 2 - Contrato Publico De Comandos (estilo dotnet)
Objetivo: padronizar UX de comando.

Entregas:
- Suportar explicitamente:
1. `pordosol new <tipo> -n <nome> -o <saida>`
2. `pordosol build [--project <caminho>]`
3. `pordosol run [--project <caminho>] [--no-build]`
- Manter aliases em portugues por compatibilidade:
1. `novo`
2. `compilar`
3. `rodar`

Status: **Concluida**.

### Fase 3 - Sistema De Templates
Objetivo: criar projetos prontos para desenvolvimento real.

Entregas:
- Estrutura `templates/` com placeholders:
1. `{{PROJECT_NAME}}`
2. `{{NAMESPACE}}`
3. `{{TARGET}}`
- `pordosol new list`
- Template `console` completo e template `web` inicial funcional.

Status: **Concluida**.

### Fase 4 - Toolchain Global E Doctor
Objetivo: remover dependencia de layout local do repositorio.

Entregas:
- Resolver binarios por estrategia global.
- Comando `pordosol doctor` com checks:
1. compilador encontrado
2. interpretador encontrado
3. stdlib encontrada
4. versoes detectadas
- Mensagens de erro com acao corretiva clara.

Status: **Concluida**.

### Fase 5 - Empacotamento E Instalacao
Objetivo: instalacao unica por usuario.

Entregas:
- Estrutura de instalacao (Windows/Linux/macOS):
1. `bin/`
2. `tools/`
3. `templates/`
- Scripts de instalacao:
1. `install.ps1`
2. `install.sh`
- Adicao de `bin` no `PATH` do usuario.

Status: **Concluida**.

### Fase 6 - CI/CD E Release
Objetivo: distribuicao confiavel.

Entregas:
- GitHub Actions para build multi-plataforma.
- Artefatos assinados/versionados.
- Publicacao automatica de release.

Status: **Concluida**.

### Fase 7 - Qualidade E Compatibilidade
Objetivo: evitar regressao no crescimento do ecossistema.

Entregas:
- Testes E2E:
1. `new -> build -> run` (console)
2. `new -> build -> run` (web minimo)
- Teste de instalacao em maquina limpa.
- Matriz de compatibilidade entre:
1. `ferramentas-cli`
2. `compilador-portugues`
3. `sistema-padrao`
4. `pordosol-language-server`

Status: **Concluida**.

## Riscos E Mitigacoes
- Risco: divergencia entre scripts antigos e contrato novo.
  Mitigacao: manter aliases e deprecar gradualmente.
- Risco: localizacao de binarios quebrar cenarios de dev.
  Mitigacao: fallback local (`./lib`) na fase de transicao.
- Risco: template web depender de framework incompleto.
  Mitigacao: template web inicial minimo, evoluindo junto do `net-por-do-sol`.

## Criterio De Sucesso Final
Em uma maquina limpa:
1. instalar `pordosol`
2. rodar `pordosol new console -n app`
3. entrar na pasta `app`
4. executar `pordosol run`
5. obter execucao sem configuracao manual adicional
