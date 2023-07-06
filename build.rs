use ethers::{prelude::Abigen, solc::Solc};
use std::path::Path;

/// Contract details.
const CONTRACT: &str = "./contract/TestContract.sol";
const CONTRACT_NAME: &str = "TestContract";
const CONTRACT_RS: &str = "test_contract.rs";

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() -> anyhow::Result<()> {
    let contract_path = std::env::current_dir()
        .expect("Current dir must be defined")
        .join(CONTRACT)
        .canonicalize()
        .expect("Full path must be defined")
        .display()
        .to_string();

    p!("Building contract: {contract_path}");

    // Compile the contract.
    let (abi, _bytecode) = {
        // This will only work if the Solidity compiler is installed,
        // otherwise error
        // Io(SolcIoError { io: Os { code: 2, kind: NotFound, message: "No such file or directory" }, path: "solc" })
        // will be produced.
        // See installation manual
        // https://docs.soliditylang.org/en/latest/installing-solidity.html
        let compiled = Solc::default().compile_source(&contract_path)?;

        let (abi, bytecode, _) = compiled
            .get(&contract_path, CONTRACT_NAME)
            .ok_or(anyhow::anyhow!("It must be possible to get the contract"))?
            .into_parts_or_default();

        (abi, bytecode)
    };

    p!("Generating bindings for {CONTRACT}");

    let out_dir = std::env::var("OUT_DIR")?;
    let dest_path = Path::new(&out_dir).join(CONTRACT_RS);

    p!("Bindings will be placed in {}", dest_path.display());
    let abi_json = serde_json::to_string(&abi)?;

    let abi_defn = Abigen::new(CONTRACT_NAME, abi_json)
        .map_err(|_err| anyhow::anyhow!("Cannot instantiate Abigen"))?
        .generate()
        .map_err(|_err| anyhow::anyhow!("Cannot generate Abi"))?
        .to_string();

    std::fs::write(dest_path, abi_defn)?;

    println!("cargo:rerun-if-env-changed=RESTART");
    println!("cargo:rerun-if-changed=contract/TestContract.sol");

    Ok(())
}

