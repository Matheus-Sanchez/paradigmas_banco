# Rust + Lua Key-Value DB (modular)

Pequeno armazenamento chave/valor em memória com regras de validação e formatação delegadas a scripts Lua. Suporte a chaves especiais:
- `cpf_*` valida dígitos e formata para `XXX.XXX.XXX-YY` na leitura
- `data_*` valida formato `YYYY-MM-DD` e formata para `DD/MM/YYYY` na leitura

## Estrutura
- `src/cli.rs` — interface com o usuário (stdin), parsing de comandos e comandos HELP/LIST.
- `src/store.rs` — armazenamento e delegação de validação/formatação para Lua.
- `src/lua_vm.rs` — wrapper `mlua`, expõe `dispatch_response` retornando `LuaResponse` tipado.
- `src/errors.rs` — erros aplicacionais (`AppError`).
- `src/main.rs` — bootstrap da CLI.
- `lua/extensions.lua` — dispatcher e regras de validação/formatação.

## Comandos da CLI
```
ADD <chave> <valor>
GET <chave>
LIST            # lista chaves armazenadas
HELP            # mostra ajuda
EXIT            # encerra
```

Exemplo:
```
ADD cpf_maria 12345678909   -> OK
GET cpf_maria               -> 123.456.789-09
ADD data_evento 2024-02-29  -> OK (ano bissexto)
# Projeto Reiniciado

Este repositório foi limpo para reimplementação do banco chave-valor Rust + Lua.

## Próximos Passos Planejados
1. Implementar estrutura simples de armazenamento (HashMap).
2. Integrar `mlua` carregando dispatcher Lua (`extensions.lua`).
3. Definir protocolo `{ success, result, error }` para validação/formatação.
4. Adicionar regras: `cpf_*` e `data_*`.
5. Criar CLI com comandos: ADD / GET / LIST / EXIT.
6. Escrever testes automatizados.

## Ambiente (Windows)
Instalar toolchain Rust + Build Tools (MSVC):
```powershell
winget install Rustlang.Rustup -e
winget install Microsoft.VisualStudio.2022.BuildTools -e --source winget --override "--quiet --wait --norestart --add Microsoft.VisualStudio.Workload.VCTools"
```
Verificar:
```powershell
rustc -V
cargo -V
```

## Executar (placeholder atual)
```powershell
cargo run
```

## Lua Dispatcher Atual
Arquivo `src/lua/extensions.lua` retorna sempre sucesso e ecoa valor.

$env:Path += ";$env:USERPROFILE\.cargo\bin"
cargo --version
cargo 1.89.0 (c24e10642 2025-06-23)
