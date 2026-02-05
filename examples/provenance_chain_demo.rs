use lineage::{CustodyEventType, MetadataHash, ProvenanceVault};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut vault = ProvenanceVault::new();

    let metadata = MetadataHash::from_bytes(b"invoice-123");
    let asset_id = vault.create_asset(
        "Invoice-123".to_string(),
        metadata,
        "Factory-A".to_string(),
    )?;

    vault.transfer(
        &asset_id,
        "Factory-A".to_string(),
        "Carrier-7".to_string(),
        5,
    )?;

    vault.record_event(
        &asset_id,
        CustodyEventType::Inspection {
            notes: "Seal intact".to_string(),
        },
    )?;

    vault.transfer(
        &asset_id,
        "Carrier-7".to_string(),
        "Warehouse-3".to_string(),
        5,
    )?;

    let report = vault.verify(&asset_id)?;
    println!("Verify status: {:?}", report.status);
    if !report.errors.is_empty() {
        println!("Errors: {:#?}", report.errors);
    }

    Ok(())
}
