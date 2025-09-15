use thiserror::Error;

/// Application wide error type centralizando erros de domínio e integração Lua.
#[derive(Error, Debug)]
pub enum AppError {
	/// Erro originado ao executar código Lua (mlua::Error convertido em String)
	#[error("Lua error: {0}")]
	Lua(String),
	/// Chave não encontrada no armazenamento
	#[error("Not found")]
	NotFound,
	/// Entrada inválida (mensagem detalhada)
	#[error("Invalid input: {0}")]
	Invalid(String),
	/// Erro de I/O (por exemplo: leitura de arquivo .lua)
	#[error("I/O error: {0}")]
	Io(String),
}

impl From<mlua::Error> for AppError {
	fn from(value: mlua::Error) -> Self { Self::Lua(value.to_string()) }
}

impl From<std::io::Error> for AppError {
	fn from(value: std::io::Error) -> Self { Self::Io(value.to_string()) }
}