use mlua::{Lua, Result as LuaResult, Value as LuaValue};
use std::collections::HashMap;
use std::io::{self, Write};

// Enum para representar o resultado de uma operação vinda do Lua.
// Isso atende à exigência de ter estruturas específicas para sucesso e erro.
#[derive(Debug)]
enum CommandResult {
    Success(String), // Retorna uma mensagem de sucesso.
    Value(String),   // Retorna um valor (usado pelo GET).
    Error(String),   // Retorna uma mensagem de erro detalhada.
}

fn main() -> LuaResult<()> {
    // Inicializa a VM do Lua.
    let lua = Lua::new();

    // Cria nosso "banco de dados" em memória.
    // Usamos um tipo específico (HashMap) para que o Rust possa armazená-lo
    // e o Lua possa acessá-lo como um "userdata".
    let mut db: HashMap<String, String> = HashMap::new();

    // Carrega o script Lua que contém toda a nossa lógica de validação e formatação.
    // O Rust não sabe o que há dentro, apenas o executa.
    let lua_script = std::fs::read_to_string("extensions.lua")
        .expect("Não foi possível ler o arquivo extensions.lua");
    
    // Executa o script Lua, tornando suas funções disponíveis.
    lua.load(&lua_script).exec()?;

    println!("Bem-vindo ao Banco de Dados Rust+Lua!");
    println!("Use os comandos: ADD chave valor, GET chave, ou 'sair' para terminar.");

    // Loop principal para ler a entrada do usuário.
    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Garante que ">" apareça antes da entrada.

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Erro ao ler a entrada.");
            continue;
        }

        let trimmed_input = input.trim();
        if trimmed_input.eq_ignore_ascii_case("sair") {
            break;
        }

        let parts: Vec<&str> = trimmed_input.splitn(3, ' ').collect();
        
        let command = parts.get(0).unwrap_or(&"").to_lowercase();
        let key = parts.get(1).map(|s| s.to_string());
        let value = parts.get(2).map(|s| s.to_string());

        // Validação básica da entrada
        if command.is_empty() || key.is_none() {
            println!("Comando inválido. Use: ADD chave valor ou GET chave");
            continue;
        }

        // Pega a função 'process_command' do escopo global do Lua.
        let process_command: mlua::Function = lua.globals().get("process_command")?;
        
        // Chama a função Lua, passando o comando, a chave, o valor e o banco de dados.
        // O Lua modificará o 'db' diretamente se a operação for um ADD válido.
        let result: LuaResult<(bool, String)> = process_command.call((command, key, value, &mut db));
        
        // Mapeia o resultado do Lua para nosso enum CommandResult.
        let command_result = match result {
            Ok((true, msg)) => {
                // Se o primeiro valor de retorno do Lua for 'true', foi um sucesso.
                // Verificamos o comando original para diferenciar a mensagem.
                if parts[0].to_lowercase() == "get" {
                    CommandResult::Value(msg)
                } else {
                    CommandResult::Success(msg)
                }
            }
            Ok((false, err_msg)) => {
                // Se o primeiro valor de retorno for 'false', foi um erro.
                CommandResult::Error(err_msg)
            }
            Err(e) => {
                // Se a própria chamada à função Lua falhou.
                CommandResult::Error(format!("Erro na execução do script Lua: {}", e))
            }
        };

        // Imprime o resultado formatado para o usuário.
        match command_result {
            CommandResult::Success(msg) => println!("[SUCESSO] {}", msg),
            CommandResult::Value(val) => println!("{}", val),
            CommandResult::Error(err) => println!("[ERRO] {}", err),
        }
    }

    Ok(())
}