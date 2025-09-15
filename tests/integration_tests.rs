use rust_lua_kv_db::{lua_vm::LuaVm, store::Store};
use rust_lua_kv_db::errors::AppError;

#[test]
fn test_add_get_cpf_valid() {
    let lua = LuaVm::new("lua/extensions.lua").expect("lua vm");
    let mut store = Store::new(lua);
    assert!(store.add("cpf_zezinho", "12345678909").is_ok());
    let got = store.get("cpf_zezinho").expect("get");
    assert_eq!(got, "123.456.789-09");
}

#[test]
fn test_add_cpf_invalid() {
    let lua = LuaVm::new("lua/extensions.lua").expect("lua vm");
    let mut store = Store::new(lua);
    let res = store.add("cpf_bad", "12345678900");
    assert!(matches!(res, Err(AppError::Invalid(_))));
}

#[test]
fn test_add_get_date_valid() {
    let lua = LuaVm::new("lua/extensions.lua").expect("lua vm");
    let mut store = Store::new(lua);
    assert!(store.add("data_joao", "2000-01-23").is_ok());
    let got = store.get("data_joao").expect("get");
    assert_eq!(got, "23/01/2000");
}

#[test]
fn test_add_date_invalid() {
    let lua = LuaVm::new("lua/extensions.lua").expect("lua vm");
    let mut store = Store::new(lua);
    let res = store.add("data_bad", "2000-02-30");
    assert!(matches!(res, Err(AppError::Invalid(_))));
}

#[test]
fn test_non_special_key() {
    let lua = LuaVm::new("lua/extensions.lua").expect("lua vm");
    let mut store = Store::new(lua);
    assert!(store.add("name_foo", "hello").is_ok());
    let got = store.get("name_foo").expect("get");
    assert_eq!(got, "hello");
}

#[test]
fn test_get_notfound() {
    let lua = LuaVm::new("lua/extensions.lua").expect("lua vm");
    let store = Store::new(lua);
    let res = store.get("no_such_key");
    assert!(matches!(res, Err(AppError::NotFound)));
}

#[test]
fn test_list_keys() {
    let lua = LuaVm::new("lua/extensions.lua").expect("lua vm");
    let mut store = Store::new(lua);
    store.add("k1", "v1").unwrap();
    store.add("k2", "v2").unwrap();
    let mut keys = store.list_keys();
    keys.sort();
    assert_eq!(keys, vec!["k1", "k2"]);
}
