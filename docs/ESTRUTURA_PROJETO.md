# Estrutura do Projeto Gateway

Este documento descreve a arquitetura e a utilidade de cada módulo do projeto Gateway, um monolito modular em Rust para colaboração em tempo real.

## Visão Geral

O projeto segue uma arquitetura modular onde cada crate representa um módulo com responsabilidades bem definidas. A comunicação entre módulos ocorre através de interfaces (traits) e tipos compartilhados, permitindo isolamento e testabilidade.

---

## Estrutura de Diretórios

```
├── Cargo.toml (Workspace definition)
├── src/main.rs (O único binário)
├── crates/
│   ├── gateway/ (Adaptador Primário: Recebe I/O)
│   ├── broker/ (Módulo de Domínio/Infraestrutura: Comunicação)
│   ├── collab/ (Módulo de Domínio: Lógica de Edição)
│   ├── persistence/ (Adaptador Secundário: Storage)
│   ├── domain/ (Tipos Comuns e Portas)
│   ├── config/ (Infraestrutura Comum)
│   └── metrics/ (Infraestrutura Comum)
```

---

## Descrição dos Módulos

### 📁 `src/main.rs` - Ponto de Entrada Único

**Utilidade:** O único binário do sistema que:
- Inicializa logging e tracing
- Carrega configuração
- Cria as dependências compartilhadas
- Registra e inicia cada módulo do sistema
- Inicia o runtime async e bloqueia até o shutdown

**Responsabilidades:**
- Orquestração do ciclo de vida do sistema
- Bootstrap de todos os módulos na ordem correta
- Gerenciamento de shutdown gracioso

---

### 📁 `crates/domain/` - O Núcleo Compartilhado (Tipos Comuns e Portas)

**Utilidade:** Define as Portas e Entidades para toda a aplicação. Módulo central para tipos de domínio compartilhados entre todos os módulos.

**Estrutura Interna:**
```
/crates/domain
├── Cargo.toml
└── src
    ├── lib.rs
    ├── events.rs (Eventos do broker, atualizações de documentos)
    ├── commands.rs (Comandos de edição e controle)
    ├── types.rs (Tipos básicos: UserID, DocumentID, etc.)
    └── traits.rs (Definição de Portas/Interfaces)
        ├── broker_port.rs (trait Producer, trait Consumer)
        └── persistence_port.rs (trait DocumentStorage)
```

**Responsabilidades:**
- **Tipos de Eventos do Broker (`events.rs`):**
  - Estruturas de dados para mensagens entre módulos
  - Eventos de atualização de documentos
  - Eventos de presença e auditoria
- **Tipos de Comandos (`commands.rs`):**
  - Comandos de edição e controle
  - Estruturas para operações de edição
- **Tipos Básicos (`types.rs`):**
  - `UserID`, `DocumentID`, etc.
  - Autenticação e usuário
  - Documentos e sessões
- **Definição de Portas/Interfaces (`traits.rs`):**
  - **`broker_port.rs`:** Define traits `Producer` e `Consumer` para comunicação assíncrona
  - **`persistence_port.rs`:** Define trait `DocumentStorage` para abstração de storage
- **Erros Comuns do Sistema:**
  - Tipos de erro padronizados
  - Result types compartilhados

**Benefício:** Todos os módulos usam esses tipos para se comunicar, evitando strings soltas e garantindo type safety. As traits permitem inversão de dependência, onde módulos de domínio dependem de abstrações, não de implementações concretas.

---

### 📁 `crates/collab/` - Módulo de Domínio (Lógica de Edição)

**Utilidade:** Núcleo de lógica de negócio para colaboração em tempo real. Contém a lógica de edição e se isola usando as traits de domain.

**Estrutura Interna:**
```
/crates/collab
├── Cargo.toml (Depende de domain, broker, persistence)
└── src
    ├── lib.rs (Define a CollabModule struct)
    ├── domain_model.rs (Estrutura interna do documento, CRDT/OT)
    ├── use_cases.rs (Funções que implementam a lógica: apply_operation)
    └── broker_handlers.rs (Consome comandos do broker, usa use_cases)
```

**Responsabilidades:**
- **Núcleo de Edição (`domain_model.rs`):**
  - Mantém o estado dos documentos em memória
  - Gerencia CRDT ou OT (Operational Transform) para texto
  - Estrutura interna do documento
- **Casos de Uso (`use_cases.rs`):**
  - Funções que implementam a lógica de negócio
  - `apply_operation`: Aplica operações de edição no documento
  - Validação de operações
  - Cálculo de novas versões e deltas
- **Handlers do Broker (`broker_handlers.rs`):**
  - Consome comandos do broker
  - Usa os casos de uso para processar comandos
  - Publica eventos de atualização após processar
- **Módulo Principal (`lib.rs`):**
  - Define a struct `CollabModule`
  - Expõe a API pública do módulo
  - Gerencia o ciclo de vida do módulo
- **Presença e Sessão de Documento:**
  - Controla quem está conectado em cada documento
  - Gerencia eventos de join e leave
  - Opção de estado de presença (digitando, só visualizando, etc)
- **Integração com Persistência:**
  - Carrega snapshot do documento ao primeiro uso
  - Grava snapshots periódicos
  - Grava histórico de operações se estiver ativado

**Dependências:** Depende de `domain` (para tipos e traits), `broker` (para consumir comandos e publicar eventos), e `persistence` (para carregar/gravar documentos).

---

### 📁 `crates/persistence/` - Adaptador Secundário (Storage)

**Utilidade:** Camada de abstração para armazenamento de dados. Implementa as Portas (traits) definidas em domain.

**Estrutura Interna:**
```
/crates/persistence
├── Cargo.toml (Depende de domain, sqlx/sled, etc.)
└── src
    ├── lib.rs (Define o PersistenceAdapter struct)
    ├── adapters
    │   └── sql_storage.rs (Implementa trait DocumentStorage, usando sqlx)
    └── schema.rs (Definição de modelos de DB)
```

**Responsabilidades:**
- **Interface Genérica de Storage:**
  - Implementa o trait `DocumentStorage` definido em `domain`
  - Salvar e carregar documentos
  - Salvar e carregar snapshots
  - Registrar eventos de auditoria (se desejado)
- **Adaptadores (`adapters/sql_storage.rs`):**
  - Implementação concreta do trait `DocumentStorage`
  - Pode usar sqlx, sled, ou outros backends
  - Isola detalhes de implementação
- **Schema (`schema.rs`):**
  - Definição de modelos de banco de dados
  - Migrations e estrutura de tabelas
- **Isolamento de Implementação:**
  - Os detalhes de implementação ficam isolados
  - O resto do monolito enxerga apenas a interface (trait)
  - Permite trocar a implementação sem afetar outros módulos

**Dependências:** Depende de `domain` (para implementar as traits) e bibliotecas de storage (sqlx, sled, etc.).

---

### 📁 `crates/gateway/` - Adaptador Primário (Recebe I/O)

**Utilidade:** Camada de entrada do sistema que recebe e traduz comunicação externa. Adaptador Primário, responsável por I/O, autenticação e tradução.

**Estrutura Interna:**
```
/crates/gateway
├── Cargo.toml (Depende de domain, broker, actix-web/axum)
└── src
    ├── lib.rs (Define o build do servidor)
    ├── http
    │   ├── routes.rs (Rotas REST)
    │   └── handlers.rs (Converte request HTTP em Command para o broker)
    ├── ws
    │   ├── socket_manager.rs (Gerencia sessões, mapeamento conn <-> doc id)
    │   └── ws_handlers.rs (Endpoint WebSocket)
    └── auth.rs (Middleware de validação de token e permissão)
```

**Responsabilidades:**
- **Servidor HTTP e WebSocket (`lib.rs`):**
  - Define o build do servidor
  - Expõe uma porta HTTP única para o mundo externo
  - Rotas REST para controle: login, criação de documento, listagem, histórico
  - Endpoint WebSocket para comunicação em tempo real com clientes
- **Rotas HTTP (`http/routes.rs`):**
  - Define todas as rotas REST do sistema
  - Mapeia URLs para handlers
- **Handlers HTTP (`http/handlers.rs`):**
  - Converte request HTTP em Command para o broker
  - Processa respostas e retorna ao cliente
- **Gerenciamento de WebSocket (`ws/socket_manager.rs`):**
  - Gerencia sessões WebSocket
  - Mapeamento de conexão <-> documento ID
  - Mapeamento de conexão <-> usuário ID
  - Necessário para decidir para quem enviar cada atualização
- **Handlers WebSocket (`ws/ws_handlers.rs`):**
  - Endpoint WebSocket
  - Recebe mensagens dos clientes
  - Envia atualizações aos clientes conectados
- **Autenticação e Autorização (`auth.rs`):**
  - Middleware para validar tokens em cada request
  - Regras de permissão por usuário e por documento
  - Rate limiting básico por IP ou usuário
- **Tradução de Eventos:**
  - Converte requisições do cliente em comandos internos para o broker
  - Se inscreve em eventos de atualização do broker e os envia via WebSocket aos clientes

**Não faz:** Implementa regras de negócio de colaboração (isso é responsabilidade do módulo `collab`)

**Dependências:** Depende de `domain` (para tipos e comandos), `broker` (para publicar comandos e consumir eventos), e framework web (actix-web ou axum).

---

### 📁 `crates/broker/` - Módulo de Domínio/Infraestrutura (Comunicação)

**Utilidade:** Sistema de mensageria interno que gerencia comunicação assíncrona entre módulos.

**Responsabilidades:**
- **Tópicos e Partições Internos:**
  - Criar tópicos e registrar consumidores
  - Publicar mensagens em um tópico
  - Consumir mensagens na ordem para uma partição
- **Grupos de Tópicos Lógicos:**
  - Comandos de documentos
  - Atualizações de documentos
  - Eventos de presença e auditoria
- **API Interna em Rust:**
  - Implementa as traits `Producer` e `Consumer` definidas em `domain`
  - Gateway e collab dependem dessas traits (não da implementação concreta)
  - Permite futura separação do broker em outro processo

**Implementação:** 
- Em memória com filas assíncronas, suportando múltiplos produtores e consumidores
- Opcionalmente com persistência simples em disco (log de append)

---

### 📁 `crates/config/` - Infraestrutura Comum

**Utilidade:** Gerenciamento centralizado de configuração do sistema.

**Responsabilidades:**
- **Fonte de Configuração:**
  - Carrega configurações de arquivo (YAML ou TOML)
  - Carrega de variáveis de ambiente
  - Suporta argumentos de linha de comando (se necessário)
- **Estrutura de Config Tipada:**
  - Struct `AppConfig` contendo:
    - Portas do servidor HTTP
    - Parâmetros do broker (tamanho de fila, diretório de armazenamento, etc)
    - Parâmetros de colaboração (tamanho máximo de documento, timeouts)
    - Opções de banco de dados
    - Nível de log e opções de tracing

**Uso:** O `main.rs` carrega essa config e a repassa para os módulos.

---

### 📁 `crates/metrics/` - Infraestrutura Comum

**Utilidade:** Sistema de observabilidade e monitoramento do sistema.

**Responsabilidades:**
- **Métricas de Negócio:**
  - Contadores e histogramas para:
    - Quantidade de documentos ativos
    - Quantidade de usuários conectados
    - Latência de operações de edição
    - Taxa de mensagens no broker
- **Health Check e Readiness:**
  - Expõe endpoint HTTP simples para health check
  - Gateway, broker, collab e persistência reportam seu status
  - Permite verificar se o sistema está pronto para receber tráfego

**Integração:** Todos os módulos reportam métricas e status para este módulo centralizado.

---

## Fluxo de Comunicação

```
Cliente → Gateway → Broker → Collab → Persistence
                ↓         ↓
            Metrics    Domain
                ↑
            Config
```

1. **Cliente** se conecta via HTTP/WebSocket ao **Gateway**
2. **Gateway** traduz requisições em comandos e publica no **Broker**
3. **Collab** consome comandos do **Broker**, processa e publica atualizações
4. **Gateway** consome atualizações do **Broker** e envia aos clientes via WebSocket
5. **Persistence** é usado por **Collab** para carregar/gravar documentos
6. Todos os módulos usam tipos do **Domain** para comunicação
7. **Config** fornece configuração para todos os módulos
8. **Metrics** coleta observabilidade de todos os módulos

---

## Princípios de Design

1. **Separação de Responsabilidades:** Cada módulo tem uma responsabilidade clara e bem definida
2. **Comunicação por Interfaces:** Módulos se comunicam através de traits, não implementações concretas
3. **Inversão de Dependência:** Módulos de domínio dependem de abstrações (traits), não de implementações concretas
4. **Tipos Compartilhados:** O módulo `domain` centraliza tipos comuns, evitando duplicação
5. **Isolamento:** Detalhes de implementação ficam isolados dentro de cada módulo
6. **Testabilidade:** Cada módulo pode ser testado independentemente através de suas interfaces
7. **Extensibilidade:** A arquitetura permite futura separação de módulos em processos distintos

---

## Arquitetura Hexagonal (Ports and Adapters)

O projeto segue os princípios da Arquitetura Hexagonal:

- **Portas (Ports):** Definidas em `domain/traits.rs`
  - `Producer` e `Consumer` para comunicação
  - `DocumentStorage` para persistência
  
- **Adaptadores (Adapters):**
  - **Primário:** `gateway` - adapta I/O externa (HTTP/WebSocket) para o domínio
  - **Secundário:** `persistence` - adapta storage externo (DB) para o domínio
  - **Infraestrutura:** `broker` - implementa as portas de comunicação

- **Domínio:**
  - `collab` - contém a lógica de negócio pura
  - `domain` - define tipos e contratos (portas)

---

## Ordem de Inicialização

1. Carregar configuração (`config`)
2. Inicializar logs e tracing
3. Inicializar runtime async
4. Criar recursos compartilhados (DB, configs, etc)
5. Inicializar `broker`
6. Inicializar `persistence`
7. Inicializar `collab` com acesso ao broker e persistência
8. Inicializar `gateway` com acesso ao broker
9. Iniciar servidor HTTP do gateway
10. Registrar handlers de shutdown gracioso

---

## Dependências entre Módulos

```
domain (sem dependências)
  ↑
  ├── broker (depende de domain)
  ├── persistence (depende de domain)
  ├── collab (depende de domain, broker, persistence)
  ├── gateway (depende de domain, broker)
  ├── config (sem dependências ou depende apenas de domain)
  └── metrics (depende de domain)
```

**Regra:** Módulos de domínio (`collab`, `domain`) não devem depender de adaptadores (`gateway`, `persistence`). Adaptadores dependem de domínio, não o contrário.

