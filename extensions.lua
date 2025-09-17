-- =============================================================
-- EXTENSÃO: Validador e Formatador de CPF
-- =============================================================

local cpf_extension = {}

-- Função interna para validar o dígito verificador do CPF.
-- Baseado na matemática do site da OBMEP referenciado.
function cpf_extension.is_valid(cpf)
    if not cpf or not cpf:match("^%d{11}$") then
        return false, "CPF deve conter exatamente 11 dígitos numéricos."
    end

    -- Verifica se todos os dígitos são iguais (ex: 111.111.111-11), o que é inválido.
    if cpf:match("^(%d)%1{10}$") then
        return false, "CPF com todos os dígitos iguais é inválido."
    end

    local digits = {}
    for i = 1, 11 do
        table.insert(digits, tonumber(cpf:sub(i, i)))
    end

    -- Cálculo do primeiro dígito verificador
    local sum = 0
    for i = 1, 9 do
        sum = sum + digits[i] * (11 - i)
    end
    local d1 = (sum * 10) % 11
    if d1 == 10 then d1 = 0 end

    if d1 ~= digits[10] then
        return false, "Primeiro dígito verificador do CPF é inválido."
    end

    -- Cálculo do segundo dígito verificador
    sum = 0
    for i = 1, 10 do
        sum = sum + digits[i] * (12 - i)
    end
    local d2 = (sum * 10) % 11
    if d2 == 10 then d2 = 0 end

    if d2 ~= digits[11] then
        return false, "Segundo dígito verificador do CPF é inválido."
    end

    return true
end

-- Função para formatar o CPF no padrão 000.000.000-00.
function cpf_extension.format(cpf)
    return cpf:gsub("(%d%d%d)(%d%d%d)(%d%d%d)(%d%d)", "%1.%2.%3-%4")
end

-- =============================================================
-- EXTENSÃO: Validador e Formatador de Data
-- =============================================================

local date_extension = {}

-- Valida se a data está no formato ISO 8601 (YYYY-MM-DD).
function date_extension.is_valid(date_str)
    local y, m, d = date_str:match("^(%d%d%d%d)-(%d%d)-(%d%d)$")
    if not y then
        return false, "Formato de data inválido. Use AAAA-MM-DD."
    end
    -- Simplificação: não estamos validando dias do mês (ex: 31 de Fev).
    -- Uma validação completa exigiria uma biblioteca de data.
    return true
end

-- Formata a data para o padrão brasileiro (dd/mm/aaaa).
function date_extension.format(date_str)
    local y, m, d = date_str:match("^(%d%d%d%d)-(%d%d)-(%d%d)$")
    return string.format("%s/%s/%s", d, m, y)
end


-- =============================================================
-- FUNÇÃO PRINCIPAL (ROTEADOR DE COMANDOS)
-- Esta é a única função que o Rust conhece e chama.
-- =============================================================
-- Recebe: command ("add" ou "get"), key, value, e a tabela 'db' do Rust.
-- Retorna: (bool sucesso, string mensagem_ou_valor)
function process_command(command, key, value, db)
    -- Tratamento para o comando ADD
    if command == "add" then
        if not value then
            return false, "Comando ADD requer uma chave e um valor."
        end

        -- Roteamento para extensões baseadas no prefixo da chave
        if key:match("^cpf_") then
            local is_valid, err_msg = cpf_extension.is_valid(value)
            if not is_valid then
                return false, err_msg
            end
        elseif key:match("^data_") then
            local is_valid, err_msg = date_extension.is_valid(value)
            if not is_valid then
                return false, err_msg
            end
        end

        -- Se passou por todas as validações, adiciona ao banco.
        db[key] = value
        return true, "Chave '" .. key .. "' adicionada com sucesso."

    -- Tratamento para o comando GET
    elseif command == "get" then
        local stored_value = db[key]
        if not stored_value then
            return false, "Chave '" .. key .. "' não encontrada."
        end

        -- Roteamento para formatadores
        if key:match("^cpf_") then
            return true, cpf_extension.format(stored_value)
        elseif key:match("^data_") then
            return true, date_extension.format(stored_value)
        end

        -- Se não houver formatação especial, retorna o valor original.
        return true, stored_value
    
    -- Comando desconhecido
    else
        return false, "Comando '" .. command .. "' desconhecido."
    end
end