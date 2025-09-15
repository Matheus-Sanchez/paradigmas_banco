//! Biblioteca principal do mini banco KV com validação/transformação via Lua.
//! Módulos expostos:
//! - cli: interação linha de comando
//! - store: armazenamento e operações
//! - lua_vm: integração com scripts Lua
//! - errors: tipos de erro
pub mod cli;
pub mod store;
pub mod lua_vm;
pub mod errors;