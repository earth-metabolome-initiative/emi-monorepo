//! Submodule for inserting collection procedures into the database.
use diesel::PgConnection;

/// Insert the samples collected and stored in the Directus procedure
/// imputing where possible the missing information.
///
/// # Implementation details
///
/// For instance we know the product model used for the coffee filters, even
/// though they are not inserted in the Directus Database at this time. The same
/// thing applies to Gloves, but unfortunately this does not apply to the
/// pipette tips as several different models were used.
///
/// # Arguments
///
/// * `directus_conn` - A mutable reference to the Directus database connection.
/// * `portal_conn` - A mutable reference to the portal database connection.
///
/// # Errors
///
/// * If the insertion fails, an error of type `error::Error` is returned.
pub fn insert_directus_collection_procedures(
    directus_conn: &mut PgConnection,
    portal_conn: &mut PgConnection,
) -> Result<(), crate::error::Error> {
    let gloves_product_brand = "weitaglove";
    let gloves_product_models = "NITRIL STRONG BLUE";
    let falcon_product_brand = "greiner BIO-ONE";
    let falcon_product_model = "TUBE, 50 ML, PP, 30/115 MM, CONICAL BOTTOM";
    let falcon_rack_product_brand = "";
    let falcon_rack_product_model = "";

    // Levels of tracking:
    // T0: Category-level tracking
    // ^ T1: Commercial product model -level tracking
    // ^ T2: Batch or Commercial batch product model -level tracking
    // ^ T3: Trackable instance -level tracking

    // - Collection preparation (?)
    // 		- Prepare the 70% ethanol (we call this trackable category E70)
    // 		- Put E70 into the sprayer (S70)
    // 		- Prepare the DBGI solvent (we call this trackable category DS)
    //
    // - Field collection preparation
    // 		- Box filled with 3 liters of liquid nitrogen
    //      - Nameplating Falcon tubes (T0)
    //        - Requires:
    //          - Falcon tubes (T1)
    //          - Nameplates for the Falcon tubes (T3)
    // - Collection
    // 		- Requires:
    // 			- Gloves (T1)
    // 			- Cutting utensil (T0)
    // 			- Sample container wrapper (T0)
    // 			- Paper-towel (T0)
    // 			- S70 sprayer (T0)
    // 			- Camera (T0)
    // 			- Sample container (T3)
    // - Sample preparation
    // 		- Requires:
    // 			- DS solvent (DS, T2)
    // 			- Pipette tips (PT, T0)
    // 			- Pipette 1000 (T1)
    // 			- Pipette 200 (T1)
    // 			- Freezer (T0)
    // 			- Freeze-dryer (T0)
    // 			- Printer (T0)
    // 			- Tweezers (T0)
    // 			- Weighing scale (T0)
    // 			- Ball mill (T3)
    // 			- Metal beads (T0)
    // 			- Nameplate for the vial (T3)
    //          - Nameplate for the eppendorfs (T3)
    //          - Vials (T1)
    //          - Glass inserts (T1)
    //          - Eppendorfs (T0)
    // 			- Vial box (T0) (9*9 configuration)
    //          - Falcon racks (T1) (3*8 configuration)
    // 		- Sub procedures:
    // 			- Freezing (T0)
    // 			- Freeze-drying (T0)
    //          - Storage: stores the nameplated Falcon tubes (T3) inside a
    //            nameplated Falcon rack (T3)
    // 			- Fractioning (also collects weight) (T0): creates a new containers
    //      (Eppendorfs (T0)) using the nameplate for the eppendorfs
    //          - Mix Countable (T0): adds the metal beads (3) to the Eppendorfs
    //            (T0) containing the grinded material
    //          - Ball mill (T3)
    //          - Extraction
    //            - Requires:
    //              - DS solvent (DS, T2)
    //              - Pipette tips (PT, T0)
    //              - Pipette (1000) (T1)
    //              - Pipette (200) (T1)
    //            - Sub procedures:
    //              - Mix liquid: mixes 1.7 ml of DS solvent (DS, T2) to (Eppendorfs
    //                (T0) containing grinded material), actually some samples will
    //                have 1.5 ml, and these will have to be handled in a different
    //                procedure.
    //              - Ball mill (T3) (again)
    //           - Centrifugation (T0): centrifuges for 2 minutes at 13000 RPM the
    //             Eppendorfs (T0) containing the grinded material and the DS
    //             solvent (DS, T2)
    //           - Aliquoting: aliquots 1000 plus minus 500 microliters of the
    //             supernatant into new containers (Vials (T1)) using the nameplate
    //             for the vials (mother extracts library)
    //             - Requires:
    //               - Vials (T1)
    //               - Nameplate for the vials (T3)
    //               - Pipette tips (PT, T0)
    //               - Pipette (1000) (T1)
    //           - Aliquoting: aliquots 200 microliters of the Vials (T1) into new
    //             containers (Vials (T1) with Glass inserts(T1)) using the
    //             nameplate for the vials (MS aliquots library)
    //             - Requires:
    //               - Vials (T1)
    //               - Nameplate for the vials (T3)
    //               - Pipette tips (PT, T0)
    //               - Pipette (200) (T1)
    //           - Storage: stores the Vials (T1) with Glass inserts(T1) inside a
    //             Vial box (T0) with a nameplate for the vial box (T3)
    //             - Requires:
    //               - Vial box (T0) 9*9 configuration
    //               - Nameplate for the vial box (T3)
    //
    // - Mass spectrometry analysis
    // - Data enrichment
    // 		- Weather data retrieval
    // 		- MGF data enrichment
    // - Storage

    Ok(())
}

/// Inserts DBGI collection procedures.
///
/// # Arguments
///
/// * `directus_conn` - A mutable reference to the Directus database connection.
/// * `portal_conn` - A mutable reference to the portal database connection.
///
/// # Errors
///
/// * If the insertion fails, an error of type `error::Error` is returned.
pub fn insert_dbgi_collection_procedures(
    directus_conn: &mut PgConnection,
    portal_conn: &mut PgConnection,
) -> Result<(), crate::error::Error> {
    // - Lab collection preparation (?)
    // 		- Prepare the 70% ethanol (we call this trackable category E70)
    // 		- Prepare the EMI solvent (we call this trackable category ES)
    // - Field collection preparation
    // 		- Box filled with 3 liters of liquid nitrogen
    // 		- Put E70 into the sprayer (S70)
    // 		- (skippable) Scan the QR code representing the batch of coffee filters
    //     (CF) to store which commercial product batch is used
    // 		- (skippable) Scan the Gloves batch which will be used in the collection
    // - Collection
    // 		- Requires:
    // 			- Gloves (non-trackable tool)
    // 			- Cutting utensil (non-trackable tool)
    // 			- We use the CF from previous field prep (non-trackable tool)
    // 			- Paper-towel (non-trackable tool)
    // 			- S70 sprayer (non-trackable tool)
    // - Extraction
    // 		- Requires:
    // 			- EMI solvent (ES, imputable when done recently)
    // 			- Pipette tips (PT, imputable when done recently)
    //

    Ok(())
}
