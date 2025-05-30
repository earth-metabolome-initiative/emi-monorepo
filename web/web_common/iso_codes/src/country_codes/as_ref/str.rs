//! Submodule implementing the `AsRef<str>` trait for country codes.

use crate::country_codes::CountryCode;

impl AsRef<str> for CountryCode {
    #[allow(clippy::too_many_lines)]
    fn as_ref(&self) -> &str {
        match self {
            CountryCode::AD => "AD",
            CountryCode::AE => "AE",
            CountryCode::AF => "AF",
            CountryCode::AG => "AG",
            CountryCode::AI => "AI",
            CountryCode::AL => "AL",
            CountryCode::AM => "AM",
            CountryCode::AO => "AO",
            CountryCode::AQ => "AQ",
            CountryCode::AR => "AR",
            CountryCode::AS => "AS",
            CountryCode::AT => "AT",
            CountryCode::AU => "AU",
            CountryCode::AW => "AW",
            CountryCode::AX => "AX",
            CountryCode::AZ => "AZ",
            CountryCode::BA => "BA",
            CountryCode::BB => "BB",
            CountryCode::BD => "BD",
            CountryCode::BE => "BE",
            CountryCode::BF => "BF",
            CountryCode::BG => "BG",
            CountryCode::BH => "BH",
            CountryCode::BI => "BI",
            CountryCode::BJ => "BJ",
            CountryCode::BL => "BL",
            CountryCode::BM => "BM",
            CountryCode::BN => "BN",
            CountryCode::BO => "BO",
            CountryCode::BQ => "BQ",
            CountryCode::BR => "BR",
            CountryCode::BS => "BS",
            CountryCode::BT => "BT",
            CountryCode::BV => "BV",
            CountryCode::BW => "BW",
            CountryCode::BY => "BY",
            CountryCode::BZ => "BZ",
            CountryCode::CA => "CA",
            CountryCode::CC => "CC",
            CountryCode::CD => "CD",
            CountryCode::CF => "CF",
            CountryCode::CG => "CG",
            CountryCode::CH => "CH",
            CountryCode::CI => "CI",
            CountryCode::CK => "CK",
            CountryCode::CL => "CL",
            CountryCode::CM => "CM",
            CountryCode::CN => "CN",
            CountryCode::CO => "CO",
            CountryCode::CR => "CR",
            CountryCode::CU => "CU",
            CountryCode::CV => "CV",
            CountryCode::CW => "CW",
            CountryCode::CX => "CX",
            CountryCode::CY => "CY",
            CountryCode::CZ => "CZ",
            CountryCode::DE => "DE",
            CountryCode::DJ => "DJ",
            CountryCode::DK => "DK",
            CountryCode::DM => "DM",
            CountryCode::DO => "DO",
            CountryCode::DZ => "DZ",
            CountryCode::EC => "EC",
            CountryCode::EE => "EE",
            CountryCode::EG => "EG",
            CountryCode::EH => "EH",
            CountryCode::ER => "ER",
            CountryCode::ES => "ES",
            CountryCode::ET => "ET",
            CountryCode::FI => "FI",
            CountryCode::FJ => "FJ",
            CountryCode::FK => "FK",
            CountryCode::FM => "FM",
            CountryCode::FO => "FO",
            CountryCode::FR => "FR",
            CountryCode::GA => "GA",
            CountryCode::GB => "GB",
            CountryCode::GD => "GD",
            CountryCode::GE => "GE",
            CountryCode::GF => "GF",
            CountryCode::GG => "GG",
            CountryCode::GH => "GH",
            CountryCode::GI => "GI",
            CountryCode::GL => "GL",
            CountryCode::GM => "GM",
            CountryCode::GN => "GN",
            CountryCode::GP => "GP",
            CountryCode::GQ => "GQ",
            CountryCode::GR => "GR",
            CountryCode::GS => "GS",
            CountryCode::GT => "GT",
            CountryCode::GU => "GU",
            CountryCode::GW => "GW",
            CountryCode::GY => "GY",
            CountryCode::HK => "HK",
            CountryCode::HM => "HM",
            CountryCode::HN => "HN",
            CountryCode::HR => "HR",
            CountryCode::HT => "HT",
            CountryCode::HU => "HU",
            CountryCode::ID => "ID",
            CountryCode::IE => "IE",
            CountryCode::IL => "IL",
            CountryCode::IM => "IM",
            CountryCode::IN => "IN",
            CountryCode::IO => "IO",
            CountryCode::IQ => "IQ",
            CountryCode::IR => "IR",
            CountryCode::IS => "IS",
            CountryCode::IT => "IT",
            CountryCode::JE => "JE",
            CountryCode::JM => "JM",
            CountryCode::JO => "JO",
            CountryCode::JP => "JP",
            CountryCode::KE => "KE",
            CountryCode::KG => "KG",
            CountryCode::KH => "KH",
            CountryCode::KI => "KI",
            CountryCode::KM => "KM",
            CountryCode::KN => "KN",
            CountryCode::KP => "KP",
            CountryCode::KR => "KR",
            CountryCode::KW => "KW",
            CountryCode::KY => "KY",
            CountryCode::KZ => "KZ",
            CountryCode::LA => "LA",
            CountryCode::LB => "LB",
            CountryCode::LC => "LC",
            CountryCode::LI => "LI",
            CountryCode::LK => "LK",
            CountryCode::LR => "LR",
            CountryCode::LS => "LS",
            CountryCode::LT => "LT",
            CountryCode::LU => "LU",
            CountryCode::LV => "LV",
            CountryCode::LY => "LY",
            CountryCode::MA => "MA",
            CountryCode::MC => "MC",
            CountryCode::MD => "MD",
            CountryCode::ME => "ME",
            CountryCode::MF => "MF",
            CountryCode::MG => "MG",
            CountryCode::MH => "MH",
            CountryCode::MK => "MK",
            CountryCode::ML => "ML",
            CountryCode::MM => "MM",
            CountryCode::MN => "MN",
            CountryCode::MO => "MO",
            CountryCode::MP => "MP",
            CountryCode::MQ => "MQ",
            CountryCode::MR => "MR",
            CountryCode::MS => "MS",
            CountryCode::MT => "MT",
            CountryCode::MU => "MU",
            CountryCode::MV => "MV",
            CountryCode::MW => "MW",
            CountryCode::MX => "MX",
            CountryCode::MY => "MY",
            CountryCode::MZ => "MZ",
            CountryCode::NA => "NA",
            CountryCode::NC => "NC",
            CountryCode::NE => "NE",
            CountryCode::NF => "NF",
            CountryCode::NG => "NG",
            CountryCode::NI => "NI",
            CountryCode::NL => "NL",
            CountryCode::NO => "NO",
            CountryCode::NP => "NP",
            CountryCode::NR => "NR",
            CountryCode::NU => "NU",
            CountryCode::NZ => "NZ",
            CountryCode::OM => "OM",
            CountryCode::PA => "PA",
            CountryCode::PE => "PE",
            CountryCode::PF => "PF",
            CountryCode::PG => "PG",
            CountryCode::PH => "PH",
            CountryCode::PK => "PK",
            CountryCode::PL => "PL",
            CountryCode::PM => "PM",
            CountryCode::PN => "PN",
            CountryCode::PR => "PR",
            CountryCode::PS => "PS",
            CountryCode::PT => "PT",
            CountryCode::PW => "PW",
            CountryCode::PY => "PY",
            CountryCode::QA => "QA",
            CountryCode::RE => "RE",
            CountryCode::RO => "RO",
            CountryCode::RS => "RS",
            CountryCode::RU => "RU",
            CountryCode::RW => "RW",
            CountryCode::SA => "SA",
            CountryCode::SB => "SB",
            CountryCode::SC => "SC",
            CountryCode::SD => "SD",
            CountryCode::SE => "SE",
            CountryCode::SG => "SG",
            CountryCode::SH => "SH",
            CountryCode::SI => "SI",
            CountryCode::SJ => "SJ",
            CountryCode::SK => "SK",
            CountryCode::SL => "SL",
            CountryCode::SM => "SM",
            CountryCode::SN => "SN",
            CountryCode::SO => "SO",
            CountryCode::SR => "SR",
            CountryCode::SS => "SS",
            CountryCode::ST => "ST",
            CountryCode::SV => "SV",
            CountryCode::SX => "SX",
            CountryCode::SY => "SY",
            CountryCode::SZ => "SZ",
            CountryCode::TC => "TC",
            CountryCode::TD => "TD",
            CountryCode::TF => "TF",
            CountryCode::TG => "TG",
            CountryCode::TH => "TH",
            CountryCode::TJ => "TJ",
            CountryCode::TK => "TK",
            CountryCode::TL => "TL",
            CountryCode::TM => "TM",
            CountryCode::TN => "TN",
            CountryCode::TO => "TO",
            CountryCode::TR => "TR",
            CountryCode::TT => "TT",
            CountryCode::TV => "TV",
            CountryCode::TW => "TW",
            CountryCode::TZ => "TZ",
            CountryCode::UA => "UA",
            CountryCode::UG => "UG",
            CountryCode::UM => "UM",
            CountryCode::US => "US",
            CountryCode::UY => "UY",
            CountryCode::UZ => "UZ",
            CountryCode::VA => "VA",
            CountryCode::VC => "VC",
            CountryCode::VE => "VE",
            CountryCode::VG => "VG",
            CountryCode::VI => "VI",
            CountryCode::VN => "VN",
            CountryCode::VU => "VU",
            CountryCode::WF => "WF",
            CountryCode::WS => "WS",
            CountryCode::YE => "YE",
            CountryCode::YT => "YT",
            CountryCode::ZA => "ZA",
            CountryCode::ZM => "ZM",
            CountryCode::ZW => "ZW",
            CountryCode::XK => "XK",
        }
    }
}
