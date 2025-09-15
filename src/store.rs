use std::collections::HashMap;
use crate::lua_vm::{LuaVm, LuaResponse};
use crate::errors::AppError;

/// Armazena pares chave/valor e delega validações/formatações à camada Lua.
pub struct Store {
	map: HashMap<String, String>,
	lua: LuaVm,
}

impl Store {
	pub fn new(lua_vm: LuaVm) -> Self { Store { map: HashMap::new(), lua: lua_vm } }

	/// Adiciona um valor validado por Lua. Falha se script retornar `success=false`.
	pub fn add(&mut self, key: &str, value: &str) -> Result<(), AppError> {
		let resp: LuaResponse = self.lua.dispatch_response("ADD", key, Some(value))?;
		if resp.success { 
			self.map.insert(key.to_string(), value.to_string());
			Ok(())
		} else {
			Err(AppError::Invalid(resp.error.unwrap_or_else(|| "unknown lua error".into())))
		}
	}

	/// Obtém um valor, aplicando possível formatação Lua (ex: CPF formatado).
	pub fn get(&self, key: &str) -> Result<String, AppError> {
		let val = self.map.get(key).ok_or(AppError::NotFound)?;
		let resp = self.lua.dispatch_response("GET", key, Some(val))?;
		if resp.success { Ok(resp.result.unwrap_or_else(|| val.clone())) } else { Err(AppError::Invalid(resp.error.unwrap_or_else(|| "unknown lua error".into()))) }
	}

	/// Lista todas as chaves armazenadas (ordem não garantida).
	pub fn list_keys(&self) -> Vec<String> {
		self.map.keys().cloned().collect()
	}
}