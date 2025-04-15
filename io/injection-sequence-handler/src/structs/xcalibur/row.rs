//! Structs for the rows in the injection sequence file.
// Header's example :
// Sample Type,File Name,Path,Instrument Method,Process Method,Calibration File,Position,Inj Vol,Level,Sample Wt,Sample Vol,ISTD Amt,Dil Factor,L1 Study,L2 Client,L3 Laboratory,L4 Company,L5 Phone,Comment,Sample Name

use crate::traits::RackFormat;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::{fmt::Display, path::Path};

#[serde_as]
#[derive(Debug, Serialize, PartialEq)]
/// Full row in the Xcalibur injection sequence.
struct SparseRow<'a> {
    /// Headers of the a full Xcalibur injection sequence file.
    /// Sample Type,File Name,Path,Instrument Method,Process Method,Calibration File,Position,Inj Vol,Level,Sample Wt,Sample Vol,ISTD Amt,Dil Factor,L1 Study,L2 Client,L3 Laboratory,L4 Company,L5 Phone,Comment,Sample Name
    #[serde(rename = "Sample Type")]
    sample_type: Option<&'a str>,
    #[serde(rename = "File Name")]
    file_name: &'a str,
    #[serde(rename = "Path")]
    path: &'a Path,
    #[serde(rename = "Instrument Method")]
    instrument_method: &'a Path,
    #[serde(rename = "Process Method")]
    process_method: Option<&'a Path>,
    #[serde(rename = "Calibration File")]
    calibration_file: Option<&'a Path>,
    #[serde(rename = "Position")]
    #[serde_as(as = "DisplayFromStr")]
    position: InjectionPosition,
    #[serde(rename = "Inj Vol")]
    inj_vol: f32,
    #[serde(rename = "Level")]
    level: Option<i32>,
    #[serde(rename = "Sample Wt")]
    sample_wt: Option<f32>,
    #[serde(rename = "Sample Vol")]
    sample_vol: Option<f32>,
    #[serde(rename = "ISTD Amt")]
    istd_amt: Option<f32>,
    #[serde(rename = "Dil Factor")]
    dil_factor: Option<f32>,
    #[serde(rename = "L1 Study")]
    l1_study: Option<&'a str>,
    #[serde(rename = "L2 Client")]
    l2_client: Option<&'a str>,
    #[serde(rename = "L3 Laboratory")]
    l3_laboratory: Option<&'a str>,
    #[serde(rename = "L4 Company")]
    l4_company: Option<&'a str>,
    #[serde(rename = "L5 Phone")]
    l5_phone: Option<&'a str>,
    #[serde(rename = "Comment")]
    comment: Option<&'a str>,
    #[serde(rename = "Sample Name")]
    sample_name: Option<&'a str>,
}

#[serde_as]
#[derive(Debug, Serialize, PartialEq)]
/// Row in the Xcalibur injection sequence.
struct Row {
    /// Headers to be specified for  Xcalibur injection sequence file.
    /// File Name,Path,Instrument Method,Position,Inj Vol
    #[serde(rename = "File Name")]
    file_name: String,
    #[serde(rename = "Path")]
    path: String,
    #[serde(rename = "Instrument Method")]
    instrument_method: String,
    #[serde(rename = "Position")]
    #[serde_as(as = "DisplayFromStr")]
    position: InjectionPosition,
    #[serde(rename = "Inj Vol")]
    inj_vol: f32,
}

#[derive(Debug, Eq, PartialEq)]
/// Struct for the position of the injection.
struct InjectionPosition {
    /// A struct for the position of the injection.
    /// ex: Y:A2
    /// The first letter correspond to the racks color codes (Y: yellow, B: blue, G: green, R: red)
    /// The second part corresponds to the position in the 54 positions rack. From A1 to F9.
    /// We build this second part by concatenating the row and the column.
    /// For this we use the trait RackFormat.
    rack_color: RackColor,
    row_letter: RowLetter,
    column_number: ColumnNumber,
}

impl Display for InjectionPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}{}", self.rack_color, self.row_letter, self.column_number)
    }
}


#[derive(Debug, Eq, PartialEq)]
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

impl Display for RackColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RackColor::Yellow => write!(f, "Y"),
            RackColor::Blue => write!(f, "B"),
            RackColor::Green => write!(f, "G"),
            RackColor::Red => write!(f, "R"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Enum for the row of the rack.
/// From A to F.
enum RowLetter {
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

impl Display for RowLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RowLetter::A => write!(f, "A"),
            RowLetter::B => write!(f, "B"),
            RowLetter::C => write!(f, "C"),
            RowLetter::D => write!(f, "D"),
            RowLetter::E => write!(f, "E"),
            RowLetter::F => write!(f, "F"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Enum for the column of the rack.
/// From 1 to 9.
enum ColumnNumber {
    /// Column 1.
    One,
    /// Column 2.
    Two,
    /// Column 3.
    Three,
    /// Column 4.
    Four,
    /// Column 5.
    Five,
    /// Column 6.
    Six,
    /// Column 7.
    Seven,
    /// Column 8.
    Eight,
    /// Column 9.
    Nine,
}

impl Display for ColumnNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColumnNumber::One => write!(f, "1"),
            ColumnNumber::Two => write!(f, "2"),
            ColumnNumber::Three => write!(f, "3"),
            ColumnNumber::Four => write!(f, "4"),
            ColumnNumber::Five => write!(f, "5"),
            ColumnNumber::Six => write!(f, "6"),
            ColumnNumber::Seven => write!(f, "7"),
            ColumnNumber::Eight => write!(f, "8"),
            ColumnNumber::Nine => write!(f, "9"),
        }
    }
}
