use std::io::{self, BufRead};
use crate::lua_vm::LuaVm;
use crate::store::Store;
use crate::errors::AppError;

fn print_help() {
	println!("Comandos disponíveis:\n  ADD <chave> <valor>  - adiciona um par\n  GET <chave>         - obtém e formata\n  LIST                - lista chaves\n  HELP                - mostra esta ajuda\n  EXIT                - sai");
}

pub fn run() -> anyhow::Result<()> {
	let lua_vm = LuaVm::new("lua/extensions.lua")?;
	let mut store = Store::new(lua_vm);

	println!("Mini KV DB (Rust+Lua). Digite HELP para ajuda.");
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		let l = line?;
		let l = l.trim();
		if l.is_empty() { continue; }
		let parts: Vec<&str> = l.splitn(3, ' ').collect();
		let cmd = parts[0].to_uppercase();
		if cmd == "EXIT" { break; }

		match cmd.as_str() {
			"ADD" => {
				if parts.len() < 3 { println!("ERROR: uso correto: ADD <chave> <valor>"); continue; }
				let key = parts[1]; let value = parts[2];
				match store.add(key, value) {
					Ok(_) => println!("OK"),
					Err(e) => println!("ERROR: {}", e),
				}
			}
			"GET" => {
				if parts.len() < 2 { println!("ERROR: uso correto: GET <chave>"); continue; }
				let key = parts[1];
				match store.get(key) {
					Ok(v) => println!("{}", v),
					Err(AppError::NotFound) => println!("NOTFOUND"),
					Err(e) => println!("ERROR: {}", e),
				}
			}
			"LIST" => {
				let mut keys = store.list_keys();
				keys.sort();
				for k in keys { println!("{}", k); }
			}
			"HELP" => print_help(),
			_ => println!("Comando desconhecido. Digite HELP."),
		}
	}
	Ok(())
}

