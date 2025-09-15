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
GET data_evento             -> 29/02/2024
```

## Requisitos
- Rust (edition 2021)
- Crate `mlua` (feature `lua54`) já baixa runtime embutido quando suportado; em alguns ambientes pode exigir `lua5.4` do sistema.

## Executar
```bash
cargo run
```
Ou versão otimizada:
```bash
cargo run --release
```

## Testes
```bash
cargo test
```

## Extensão via Lua
Adicione novas regras no `dispatch` do `lua/extensions.lua` criando novos prefixos de chave.

## License
MIT (ajuste se necessário)