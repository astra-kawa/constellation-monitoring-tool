Backend structure:

```
backend/
├── Cargo.toml                  # [workspace] definition
├── crates/
│   ├── domain/
│   │   ├── Cargo.toml          # Zero or near-zero external deps
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models.rs
│   │       ├── events.rs
│   │       ├── errors.rs
│   │       └── services.rs     # Pure domain logic, no I/O
│   │
│   ├── ports/
│   │   ├── Cargo.toml          # Depends on: domain
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── inbound.rs
│   │       └── outbound.rs
│   │
│   ├── application/
│   │   ├── Cargo.toml          # Depends on: domain, ports
│   │   └── src/
│   │       ├── lib.rs
│   │       └── use_cases.rs
│   │
│   ├── adapters/
│   │   ├── Cargo.toml          # Depends on: domain, ports. Brings in sqlx, reqwest, etc.
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── persistence/
│   │       │   ├── mod.rs
│   │       │   ├── pg_ephemeris_repo.rs
│   │       │   └── models.rs   # DB row types, From<DomainModel> impls
│   │       ├── julia_client/
│   │       │   ├── mod.rs
│   │       │   ├── client.rs
│   │       │   └── dto.rs      # Julia API request/response shapes
│   │       ├── tle_provider/
│   │       │   └── mod.rs
│   │       └── http/
│   │           ├── mod.rs
│   │           ├── routes.rs
│   │           ├── handlers.rs
│   │           ├── dto.rs      # API request/response types
│   │           ├── errors.rs   # ApiError, IntoResponse impl
│   │           └── ws.rs
│   │
│   └── server/
│       ├── Cargo.toml          # Depends on: application, adapters. This is the composition root.
│       └── src/
│           └── main.rs         # Wiring, config, startup
```