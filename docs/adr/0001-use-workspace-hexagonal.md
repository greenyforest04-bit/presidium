# ADR 001: Cargo Workspace with Hexagonal Architecture

**Date:** 2026-05-22

**Status:** Accepted

## Context

Presidium Messenger is a complex, long-term project (365+ days of development) that requires a sustainable, maintainable, and testable codebase architecture. We need to decide how to organize the Rust codebase to support:

1. **Multiple bounded contexts**: crypto, P2P networking, messaging, storage, LLM inference, and mobile bridge — each with distinct concerns and dependencies.
2. **Independent testability**: each component must be testable in isolation with mock dependencies.
3. **Long-term evolution**: the architecture must accommodate changes without cascading rewrites.
4. **Team scalability**: multiple developers (or AI agents) should be able to work on different parts concurrently.

### Alternatives Considered

#### Option A: Single Binary Crate

A single `presidium` crate with all logic in a flat module structure.

- **Pros**: Simple setup, no workspace overhead, easy to navigate initially.
- **Cons**: Compiles as a single unit (slow incremental builds), no enforced boundaries between domains, quickly becomes a "big ball of mud", difficult to test in isolation, dependency conflicts between subdomains.

#### Option B: Multi-Repository (Polyrepo)

Each bounded context in its own Git repository with versioned inter-dependencies.

- **Pros**: Complete isolation, independent versioning, team ownership.
- **Cons**: Cross-repo coordination overhead, complex CI/CD, dependency version drift, difficult to do atomic changes across domains, release coordination nightmare for a project at this scale.

#### Option C: Cargo Workspace with Hexagonal Architecture (Chosen)

A monorepo organized as a Cargo workspace, with each bounded context in its own crate following Hexagonal (Ports & Adapters) architecture.

- **Pros**: Single source of truth, atomic commits across crates, shared CI/CD, Cargo workspace enables dependency sharing and faster builds, Hexagonal architecture enforces boundaries at the type level.
- **Cons**: Some initial boilerplate, requires discipline to maintain layer separation, slightly more complex `Cargo.toml` management.

## Decision

We choose **Option C**: Cargo Workspace with Hexagonal Architecture.

### Workspace Structure

```
presidium/
├── Cargo.toml          # [workspace] with members = ["crates/*"]
├── crates/
│   ├── presidium-core/     # Hexagonal core: domain, application ports
│   ├── presidium-crypto/   # E2EE crypto ports & adapters
│   ├── presidium-p2p/      # P2P networking ports & adapters
│   ├── presidium-storage/  # Storage ports & adapters
│   ├── presidium-llm/      # On-device LLM ports & adapters
│   ├── presidium-messaging/ # Messaging domain
│   └── presidium-bridge/   # UniFFI mobile bridge
```

### Hexagonal Architecture per Crate

Each crate follows the same internal structure:

```
src/
├── domain/           # Pure business logic, ZERO external dependencies
│   ├── entities/     # Objects with identity
│   ├── value_objects/ # Immutable, equality-by-value
│   ├── aggregates/   # Consistency boundaries
│   └── events/       # Domain events
├── application/      # Use cases and port definitions
│   ├── use_cases/    # Interactors (business orchestration)
│   └── ports/        # Trait interfaces for adapters
└── infrastructure/   # Adapter implementations
    ├── adapters/     # External system integrations
    └── repositories/ # Data persistence
```

### Dependency Direction (Strict)

```
domain ← application ← infrastructure
```

- `domain` MUST NOT depend on `application` or `infrastructure`.
- `application` depends on `domain` only (via imports).
- `infrastructure` depends on `application` (implements ports) and `domain`.

### Cross-Crate Dependencies

- `presidium-core` is the root dependency — it defines shared value objects, errors, and core port traits.
- Other crates depend on `presidium-core` for shared types.
- No circular dependencies between crates.

## Consequences

### Positive

1. **Enforced boundaries**: Rust's module system and visibility rules naturally enforce the hexagonal layer separation. The `domain` layer cannot accidentally import infrastructure concerns.
2. **Independent testability**: Each port can be mocked independently. Use cases are tested with `mockall` or hand-written test adapters.
3. **Faster incremental builds**: Cargo workspace shares compiled dependencies. Only changed crates need recompilation.
4. **Atomic refactoring**: Cross-crate changes can be made in a single commit, unlike polyrepo.
5. **Clear ownership**: Each crate maps to a bounded context with well-defined responsibilities.
6. **Swappable implementations**: Port traits allow swapping crypto libraries, storage engines, or LLM backends without changing business logic.

### Negative

1. **Boilerplate overhead**: Each new feature requires creating port traits, adapter implementations, and wiring. This is intentional — the ceremony enforces thinking about boundaries.
2. **Learning curve**: Developers must understand Hexagonal Architecture and DDD concepts. The ADR and code examples should help.
3. **Workspace management**: Dependency version conflicts between crates require coordination in `Cargo.toml` workspace dependencies.
4. **Compilation time**: The initial full build takes longer than a single crate. However, incremental builds are fast due to workspace caching.

### Risks and Mitigations

| Risk | Mitigation |
|------|-----------|
| Layer violations (domain importing infrastructure) | Architecture integration test + CI clippy lints |
| Boilerplate fatigue | Code generation scripts for new crates (future) |
| Dependency version drift | Centralized workspace.dependencies in root Cargo.toml |
| Over-engineering for simple features | Apply hexagonal only where it adds value; stubs are acceptable |
