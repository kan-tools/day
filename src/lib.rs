//! day — the structured *process* layer that sits next to kan's structured
//! *knowledge* layer.
//!
//! kan owns durable claims and pure reads over them. day owns process:
//! teloi (desired world-states held up to weak equivalence), the atoms that
//! bridge between them, and the assessment of whether work actually landed
//! where it intended. The division is kan's ADR-18 boundary rule, and it is
//! load-bearing here: **day keeps no store of its own**. Every durable thing
//! day knows is a kan claim under the naming conventions in
//! `docs/CONVENTIONS.md`, read back through kan's public CLI.
//!
//! See `docs/TELOS.md` for the model this implements.

pub mod atoms;
pub mod cli;
pub mod doctor;
pub mod hooks;
pub mod kan_client;
pub mod mcp;
