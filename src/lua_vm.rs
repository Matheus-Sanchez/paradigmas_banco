use mlua::{Lua, Table, Value, Function};
use crate::errors::AppError;

/// Resultado tipado de uma chamada Lua.
pub struct LuaResponse {
	pub success: bool,
	pub result: Option<String>,
	pub error: Option<String>,
}

impl LuaResponse {
	fn from_table(tbl: Table) -> Self {
		let success: bool = tbl.get("success").unwrap_or(false);
		let result: Option<String> = tbl.get("result").ok();
		let error: Option<String> = tbl.get("error").ok();
		LuaResponse { success, result, error }
	}
}

/// Wrapper da VM Lua escondendo detalhes de mlua e expondo dispatch seguro.
pub struct LuaVm {
	lua: Lua,
}

impl LuaVm {
	/// Cria nova VM e carrega script de extensões (`extensions.lua`).
	pub fn new(lua_file_path: &str) -> Result<Self, AppError> {
		let lua = Lua::new();
		let code = std::fs::read_to_string(lua_file_path)?;
		lua.load(&code).set_name("extensions.lua")?.exec()?;
		Ok(LuaVm { lua })
	}

	/// Dispatch cru: retorna a `Table` original.
	pub fn dispatch(&self, action: &str, key: &str, value: Option<&str>) -> Result<Table, AppError> {
		let globals = self.lua.globals();
		let dispatch: Function = globals.get("dispatch")?;
		let val = match value { Some(v) => Value::String(self.lua.create_string(v)?), None => Value::Nil };
		let res: Table = dispatch.call((action, key, val))?;
		Ok(res)
	}

	/// Dispatch que já converte para `LuaResponse` para ergonomia.
	pub fn dispatch_response(&self, action: &str, key: &str, value: Option<&str>) -> Result<LuaResponse, AppError> {
		let tbl = self.dispatch(action, key, value)?;
		Ok(LuaResponse::from_table(tbl))
	}
}