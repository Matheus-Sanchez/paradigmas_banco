-- Dispatcher de extensões em Lua
-- Retorno: { success = boolean, result = string?, error = string? }


local function starts_with(str, prefix)
return str:sub(1, #prefix) == prefix
end


local function validate_cpf_digits(s)
if not s then return false, "empty cpf" end
if #s ~= 11 then return false, "CPF must have 11 digits" end
if not s:match("^%d+$") then return false, "CPF must contain only digits" end
local digits = {}
for i=1,11 do digits[i] = tonumber(s:sub(i,i)) end
local sum = 0
for i=1,9 do sum = sum + digits[i] * (11 - i) end
local r = (sum * 10) % 11
if r == 10 then r = 0 end
if r ~= digits[10] then return false, "first check digit invalid" end
sum = 0
for i=1,10 do sum = sum + digits[i] * (12 - i) end
r = (sum * 10) % 11
if r == 10 then r = 0 end
if r ~= digits[11] then return false, "second check digit invalid" end
return true
end


local function format_cpf(s)
return s:sub(1,3) .. "." .. s:sub(4,6) .. "." .. s:sub(7,9) .. "-" .. s:sub(10,11)
end


local function is_leap(year)
return (year % 4 == 0 and year % 100 ~= 0) or (year % 400 == 0)
end


local function validate_date_iso(s)
if not s then return false, "empty date" end
local y,m,d = s:match("^(%d%d%d%d)%-(%d%d)%-(%d%d)$")
if not y then return false, "date must be in YYYY-MM-DD" end
y = tonumber(y); m = tonumber(m); d = tonumber(d)
if m < 1 or m > 12 then return false, "invalid month" end
local mdays = {31,28,31,30,31,30,31,31,30,31,30,31}
if is_leap(y) then mdays[2] = 29 end
if d < 1 or d > mdays[m] then return false, "invalid day" end
return true
end


local function format_date(s)
local y,m,d = s:match("^(%d%d%d%d)%-(%d%d)%-(%d%d)$")
return d .. "/" .. m .. "/" .. y
end


function dispatch(action, key, value)
local out = {}
if action == "ADD" then
if starts_with(key, "cpf_") then
local ok, msg = validate_cpf_digits(value)
if ok then out.success = true else out.success = false; out.error = msg end
elseif starts_with(key, "data_") then
local ok, msg = validate_date_iso(value)
if ok then out.success = true else out.success = false; out.error = msg end
else
out.success = true
end
elseif action == "GET" then
if starts_with(key, "cpf_") then
if value and #value == 11 and value:match("^%d+$") then
out.success = true; out.result = format_cpf(value)
else
out.success = false; out.error = "stored CPF invalid"
end
elseif starts_with(key, "data_") then
local ok, msg = validate_date_iso(value)
if ok then out.success = true; out.result = format_date(value) else out.success = false; out.error = "stored date invalid: " .. (msg or "") end
else
out.success = true
end
else
out.success = false; out.error = "unknown action"
end
return out
end