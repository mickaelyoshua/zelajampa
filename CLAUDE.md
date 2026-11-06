# ZelaJampa - Contexto para Agentes de IA

## Visão Geral
Plataforma de mapeamento e agregação de demandas de infraestrutura urbana em João Pessoa.
Foco em mitigar duplicidade via geohashing e identificar clusters críticos.

## Stack Tecnológico
- **Backend:** Rust
- **Roteamento:** axum
- **Persistência:** PostgreSQL + PostGIS (extensão espacial)
- **ORM/Data Access:** sqlx (queries validadas em tempo de compilação)
- **Frontend/Interface:** SSR via Askama + HTMX (TailwindCSS para estilos)
- **Ambiente:** Docker / Docker Compose

## Diretrizes Arquiteturais
- **Top-Down & Hexagonal:** Roteamento (`api`), Domínio (`core`), e Dados (`infra`).
- **Geohashing:** O processamento pesado de agrupamento espacial deve ser feito no PostGIS (`ST_GeoHash`, etc), delegando ao BD para eficiência.
- **Tipagem Forte:** "Parse, don't validate". Uso de Newtypes para fronteiras e isolamento de estados inválidos no compile-time.
- **TDD:** Prioridade para testes de integração de rotas (axum::test) e infra (sqlx isolation).

## Regras Globais
- **Código:** Todo o código gerado (variáveis, funções, classes, comentários, nomes de arquivos) DEVE ser em inglês.
- **Comunicação:** Exclusivamente no estilo "Caveman Full" em Português. Sem encheção de linguiça, conciso, direto ao ponto técnico. Sempre mostre os diffs de alterações.
- **Filosofia:** Soluções pragmáticas, menor aninhamento possível, retornos antecipados (guard clauses).
- **Entregas:** Responder e popular as entregas do projeto na plataforma Dreamshaper conforme avançamos.

## Referências
- Arquivo de planejamento principal: `plan.md`

- **Observabilidade:** Uso mandatório da crate `tracing` para structured logging. Sem macros legadas do `log`.
- **Error Handling:** Erros envelopados via `thiserror` para fronteiras claras sem vazar stack de banco (sqlx) pro HTTP.
