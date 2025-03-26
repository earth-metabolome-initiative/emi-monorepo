//! Structs for the rows in the injection sequence file.
// Header's example :
// Sample Type,File Name,Path,Instrument Method,Process Method,Calibration File,Position,Inj Vol,Level,Sample Wt,Sample Vol,ISTD Amt,Dil Factor,L1 Study,L2 Client,L3 Laboratory,L4 Company,L5 Phone,Comment,Sample Name

use std::path::Path;
use serde::Deserialize;
use traits::rack_format::RackFormat;

#[derive(Debug, Deserialize, Eq, PartialEq)]
/// Full row in the Xcalibur injection sequence.
struct SparseRow<'a> {
    /// Headers of the a full Xcalibur injection sequence file.
    /// Sample Type,File Name,Path,Instrument Method,Process Method,Calibration File,Position,Inj Vol,Level,Sample Wt,Sample Vol,ISTD Amt,Dil Factor,L1 Study,L2 Client,L3 Laboratory,L4 Company,L5 Phone,Comment,Sample Name
    #[serde(rename = "Sample Type")]
    sample_type : Option<&'a str>,
    #[serde(rename = "File Name")]
    file_name : &'a str,
    #[serde(rename = "Path")]
    path : &'a Path,
    #[serde(rename = "Instrument Method")]
    instrument_method : &'a Path,
    #[serde(rename = "Process Method")]
    process_method : Option<&'a Path>,
    #[serde(rename = "Calibration File")]
    calibration_file : Option<&'a Path>,
    #[serde(rename = "Position")]
    position : &'a str,
    #[serde(rename = "Inj Vol")]
    inj_vol : i32,
    #[serde(rename = "Level")]
    level : Option<i32>,
    #[serde(rename = "Sample Wt")]
    sample_wt : Option<i32>,
    #[serde(rename = "Sample Vol")]
    sample_vol : Option<i32>,
    #[serde(rename = "ISTD Amt")]
    istd_amt : Option<i32>,
    #[serde(rename = "Dil Factor")]
    dil_factor : Option<i32>,
    #[serde(rename = "L1 Study")]
    l1_study : Option<&'a str>,
    #[serde(rename = "L2 Client")]
    l2_client : Option<&'a str>,
    #[serde(rename = "L3 Laboratory")]
    l3_laboratory : Option<&'a str>,
    #[serde(rename = "L4 Company")]
    l4_company : Option<&'a str>,
    #[serde(rename = "L5 Phone")]
    l5_phone : Option<&'a str>,
    #[serde(rename = "Comment")]
    comment : Option<&'a str>,
    #[serde(rename = "Sample Name")]
    sample_name : Option<&'a str>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
/// Row in the Xcalibur injection sequence.
struct Row {
    /// Headers to be specified for  Xcalibur injection sequence file.
    /// File Name,Path,Instrument Method,Position,Inj Vol
    #[serde(rename = "File Name")]
    file_name : String,
    #[serde(rename = "Path")]
    path : String,
    #[serde(rename = "Instrument Method")]
    instrument_method : String,
    #[serde(rename = "Position")]
    position : format!("{}:{}{}", rack_color, row, column),
    #[serde(rename = "Inj Vol")]
    inj_vol : i32,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Expand)]
/// Struct for the position of the injection.
struct InjectionPosition {
    /// A struct for the position of the injection.
    /// ex: Y:A2
    /// The first letter correspond to the racks color codes (Y: yellow, B: blue, G: green, R: red)
    /// The second part corresponds to the position in the 54 positions rack. From A1 to F9.
    /// We build this second part by concatenating the row and the column.
    /// For this we use the trait RackFormat.
    #[serde(rename = "Position")]
    rack_color : RackColor,
    
}

/// Enum for the color of the rack.
enum RackColor {
    /// Yellow rack.
    Yellow,
    /// Blue rack.
    Blue,
    /// Green rack.
    Green,
    /// Red rack.
    Red,
}

/// Enum for the row of the rack.
/// From A to F.
enum Row {
    /// Row A.
    A,
    /// Row B.
    B,
    /// Row C.
    C,
    /// Row D.
    D,
    /// Row E.
    E,
    /// Row F.
    F,
}

/// Enum for the column of the rack.
/// From 1 to 9.
enum Column {
    /// Column 1.
    One