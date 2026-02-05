use lineage::{CustodyEventType, MetadataHash, ProvenanceVault};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vault = ProvenanceVault::new();

    let metadata = MetadataHash::from_bytes(b"batch-77|lot-2026-02");
    let asset_id = vault.create_asset(
        "Batch-77".to_string(),
        metadata,
        "Plant-5".to_string(),
    )?;

    vault.transfer(
        &asset_id,
        "Plant-5".to_string(),
        "Distributor-2".to_string(),
        10,
    )?;

    vault.record_event(
        &asset_id,
        CustodyEventType::Inspection {
            notes: "Temperature variance detected".to_string(),
        },
    )?;

    vault.transfer(
        &asset_id,
        "Distributor-2".to_string(),
        "Retail-11".to_string(),
        10,
    )?;

    vault.seal(&asset_id, "Recall issued: contamination risk".to_string())?;

    let report = vault.verify(&asset_id)?;
    println!("Recall chain verification: {:?}", report.status);
    if !report.errors.is_empty() {
        println!("Errors: {:#?}", report.errors);
    }

    Ok(())
}
