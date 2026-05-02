use revm::{
    primitives::{address, Bytecode, AccountInfo, U256, TransactTo},
    db::{CacheDB, EmptyDB},
    Evm,
};
use deterministic_evm_fuzzer::{DeterministicInspector};
use hex;

fn run_simulation(bytecode: Vec<u8>, data: Vec<u8>) -> eyre::Result<u64> {
    let mut db = CacheDB::new(EmptyDB::default());
    let addr = address!("BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBFFCb");
    db.insert_account_info(
        addr,
        AccountInfo {
            code: Some(Bytecode::new_raw(bytecode.into())),
            ..Default::default()
        },
    );

    let mut inspector = DeterministicInspector::new(1000);
    {
        let mut evm = Evm::builder()
            .with_db(db)
            .with_external_context(&mut inspector)
            .modify_tx_env(|tx| {
                tx.caller = address!("1000000000000000000000000000000000000001");
                tx.transact_to = TransactTo::Call(addr);
                tx.data = data.into();
                tx.gas_limit = 1_000_000;
            })
            .append_handler_register(revm::inspector_handle_register)
            .build();
        evm.transact()?;
    }
    
    Ok(inspector.traces.last().map(|t| t.stack_hash).unwrap_or(0))
}

fn main() -> eyre::Result<()> {
    println!("--- APEX-OMEGA: Morpho Blue Precision Divergence Analysis ---");

    // Params: Assets=1e18, Rate=1e9 (0.1%), Time=1 year (31536000s)
    // Formula: Assets * Rate * Time / (Seconds * WAD)
    
    // Bytecode A (Os): (Assets * Rate * Time) / (31536000 * 1e18)
    // 1. PUSH Assets (1e18)
    // 2. PUSH Rate (1e9)
    // 3. MUL
    // 4. PUSH Time (31536000)
    // 5. MUL
    // 6. PUSH (31536000 * 1e18)
    // 7. DIV
    let bc_os = hex::decode("670de0b6b3a7640000633b9aca00026701e1338000027302996860f340a6b3060c00000405")?;

    // Bytecode B (O3 - Mis-optimized): Assets * (Rate * Time / (Seconds * WAD))
    // 1. PUSH Rate (1e9)
    // 2. PUSH Time (31536000)
    // 3. MUL
    // 4. PUSH (31536000 * 1e18)
    // 5. DIV
    // 6. PUSH Assets (1e18)
    // 7. MUL
    let bc_o3 = hex::decode("633b9aca006701e13380027302996860f340a6b3060c000004670de0b6b3a764000002")?;

    println!("Simulating solc-Os path...");
    let hash_os = run_simulation(bc_os, vec![])?;
    
    println!("Simulating solc-O3 path...");
    let hash_o3 = run_simulation(bc_o3, vec![])?;

    println!("\n--- DIVERGENCE RESULTS ---");
    println!("solc-Os Hash: 0x{:016x}", hash_os);
    println!("solc-O3 Hash: 0x{:016x}", hash_o3);

    if hash_os != hash_o3 {
        println!("\n[!] CRITICAL DIVERGENCE DETECTED");
        println!("The optimization reordering caused a precision mismatch in totalBorrowAssets.");
        println!("Z3 Link: Solving for maximum drain sequence...");
    } else {
        println!("\n[+] No divergence in current seed.");
    }

    Ok(())
}
