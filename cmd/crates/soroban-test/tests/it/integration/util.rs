use soroban_test::{AssertExt, TestEnv, Wasm};
use std::fmt::Display;

pub const HELLO_WORLD: &Wasm = &Wasm::Custom("test-wasms", "test_hello_world");
pub const CUSTOM_TYPES: &Wasm = &Wasm::Custom("test-wasms", "test_custom_types");

pub async fn invoke_with_roundtrip<D>(e: &TestEnv, id: &str, func: &str, data: D)
where
    D: Display,
{
    let data = data.to_string();
    println!("{data}");
    let res = e
        .invoke_with_test(&["--id", id, "--", func, &format!("--{func}"), &data])
        .await
        .unwrap();
    assert_eq!(res, data);
}

pub const TEST_SALT: &str = "f55ff16f66f43360266b95db6f8fec01d76031054306ae4a4b380598f6cfd114";

pub fn deploy_hello(sandbox: &TestEnv) -> String {
    deploy_contract(sandbox, HELLO_WORLD)
}

pub fn deploy_custom(sandbox: &TestEnv) -> String {
    deploy_contract(sandbox, CUSTOM_TYPES)
}

pub fn deploy_contract(sandbox: &TestEnv, wasm: &Wasm) -> String {
    sandbox
        .new_assert_cmd("contract")
        .arg("deploy")
        .arg("--fee")
        .arg("1000000")
        .arg("--instructions")
        .arg(&u32::MAX.to_string())
        .arg("--wasm")
        .arg(wasm.path())
        .arg("--salt")
        .arg(TEST_SALT)
        .arg("--ignore-checks")
        .assert()
        .stderr("")
        .stdout_as_str()
}

pub async fn extend_contract(sandbox: &TestEnv, id: &str) {
    extend(sandbox, id, None).await;
}

pub async fn extend(sandbox: &TestEnv, id: &str, value: Option<&str>) {
    let mut args = vec![
        "--id",
        id,
        "--durability",
        "persistent",
        "--ledgers-to-extend",
        "100000",
    ];
    if let Some(value) = value {
        args.push("--key");
        args.push(value);
    }
    sandbox
        .new_assert_cmd("contract")
        .arg("extend")
        .args(args)
        .assert()
        .success();
}
