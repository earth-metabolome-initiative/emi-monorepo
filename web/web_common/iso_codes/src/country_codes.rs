//! Submodule providing an enumeration of ISO 3166-1 alpha-2 country codes.

mod as_ref;
pub mod diesel_impls;
mod display;
mod emoji;
mod from;
mod name;
mod unicode;
mod try_from;

#[cfg(feature = "pgrx")]
use pgrx::FromDatum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "pgrx", derive(pgrx::PostgresEnum))]
#[cfg_attr(feature = "diesel", derive(diesel::FromSqlRow, diesel::AsExpression))]
#[cfg_attr(
	feature = "diesel",
	diesel(sql_type = crate::country_codes::diesel_impls::CountryCode)
)]
/// Country codes as defined by ISO 3166-1 alpha-2.
pub enum CountryCode {
    /// Country code of Andorra
    AD,
    /// Country code of United Arab Emirates
    AE,
    /// Country code of Afghanistan
    AF,
    /// Country code of Antigua and Barbuda
    AG,
    /// Country code of Anguilla
    AI,
    /// Country code of Albania
    AL,
    /// Country code of Armenia
    AM,
    /// Country code of Angola
    AO,
    /// Country code of Antarctica
    AQ,
    /// Country code of Argentina
    AR,
    /// Country code of American Samoa
    AS,
    /// Country code of Austria
    AT,
    /// Country code of Australia
    AU,
    /// Country code of Aruba
    AW,
    /// Country code of Åland Islands
    AX,
    /// Country code of Azerbaijan
    AZ,
    /// Country code of Bosnia and Herzegovina
    BA,
    /// Country code of Barbados
    BB,
    /// Country code of Bangladesh
    BD,
    /// Country code of Belgium
    BE,
    /// Country code of Burkina Faso
    BF,
    /// Country code of Bulgaria
    BG,
    /// Country code of Bahrain
    BH,
    /// Country code of Burundi
    BI,
    /// Country code of Benin
    BJ,
    /// Country code of Saint Barthélemy
    BL,
    /// Country code of Bermuda
    BM,
    /// Country code of Brunei Darussalam
    BN,
    /// Country code of Bolivia
    BO,
    /// Country code of Bonaire, Sint Eustatius and Saba
    BQ,
    /// Country code of Brazil
    BR,
    /// Country code of Bahamas
    BS,
    /// Country code of Bhutan
    BT,
    /// Country code of Bouvet Island
    BV,
    /// Country code of Botswana
    BW,
    /// Country code of Belarus
    BY,
    /// Country code of Belize
    BZ,
    /// Country code of Canada
    CA,
    /// Country code of Cocos (Keeling) Islands
    CC,
    /// Country code of Democratic Republic of the Congo
    CD,
    /// Country code of Central African Republic
    CF,
    /// Country code of Republic of the Congo
    CG,
    /// Country code of Switzerland
    CH,
    /// Country code of Côte D'Ivoire
    CI,
    /// Country code of Cook Islands
    CK,
    /// Country code of Chile
    CL,
    /// Country code of Cameroon
    CM,
    /// Country code of China
    CN,
    /// Country code of Colombia
    CO,
    /// Country code of Costa Rica
    CR,
    /// Country code of Cuba
    CU,
    /// Country code of Cape Verde
    CV,
    /// Country code of Curaçao
    CW,
    /// Country code of Christmas Island
    CX,
    /// Country code of Cyprus
    CY,
    /// Country code of Czech Republic
    CZ,
    /// Country code of Germany
    DE,
    /// Country code of Djibouti
    DJ,
    /// Country code of Denmark
    DK,
    /// Country code of Dominica
    DM,
    /// Country code of Dominican Republic
    DO,
    /// Country code of Algeria
    DZ,
    /// Country code of Ecuador
    EC,
    /// Country code of Estonia
    EE,
    /// Country code of Egypt
    EG,
    /// Country code of Western Sahara
    EH,
    /// Country code of Eritrea
    ER,
    /// Country code of Spain
    ES,
    /// Country code of Ethiopia
    ET,
    /// Country code of Finland
    FI,
    /// Country code of Fiji
    FJ,
    /// Country code of Falkland Islands (Malvinas)
    FK,
    /// Country code of Micronesia
    FM,
    /// Country code of Faroe Islands
    FO,
    /// Country code of France
    FR,
    /// Country code of Gabon
    GA,
    /// Country code of United Kingdom
    GB,
    /// Country code of Grenada
    GD,
    /// Country code of Georgia
    GE,
    /// Country code of French Guiana
    GF,
    /// Country code of Guernsey
    GG,
    /// Country code of Ghana
    GH,
    /// Country code of Gibraltar
    GI,
    /// Country code of Greenland
    GL,
    /// Country code of Gambia
    GM,
    /// Country code of Guinea
    GN,
    /// Country code of Guadeloupe
    GP,
    /// Country code of Equatorial Guinea
    GQ,
    /// Country code of Greece
    GR,
    /// Country code of South Georgia
    GS,
    /// Country code of Guatemala
    GT,
    /// Country code of Guam
    GU,
    /// Country code of Guinea-Bissau
    GW,
    /// Country code of Guyana
    GY,
    /// Country code of Hong Kong
    HK,
    /// Country code of Heard Island and Mcdonald Islands
    HM,
    /// Country code of Honduras
    HN,
    /// Country code of Croatia
    HR,
    /// Country code of Haiti
    HT,
    /// Country code of Hungary
    HU,
    /// Country code of Indonesia
    ID,
    /// Country code of Ireland
    IE,
    /// Country code of Israel
    IL,
    /// Country code of Isle of Man
    IM,
    /// Country code of India
    IN,
    /// Country code of British Indian Ocean Territory
    IO,
    /// Country code of Iraq
    IQ,
    /// Country code of Iran
    IR,
    /// Country code of Iceland
    IS,
    /// Country code of Italy
    IT,
    /// Country code of Jersey
    JE,
    /// Country code of Jamaica
    JM,
    /// Country code of Jordan
    JO,
    /// Country code of Japan
    JP,
    /// Country code of Kenya
    KE,
    /// Country code of Kyrgyzstan
    KG,
    /// Country code of Cambodia
    KH,
    /// Country code of Kiribati
    KI,
    /// Country code of Comoros
    KM,
    /// Country code of Saint Kitts and Nevis
    KN,
    /// Country code of North Korea
    KP,
    /// Country code of South Korea
    KR,
    /// Country code of Kuwait
    KW,
    /// Country code of Cayman Islands
    KY,
    /// Country code of Kazakhstan
    KZ,
    /// Country code of Lao People's Democratic Republic
    LA,
    /// Country code of Lebanon
    LB,
    /// Country code of Saint Lucia
    LC,
    /// Country code of Liechtenstein
    LI,
    /// Country code of Sri Lanka
    LK,
    /// Country code of Liberia
    LR,
    /// Country code of Lesotho
    LS,
    /// Country code of Lithuania
    LT,
    /// Country code of Luxembourg
    LU,
    /// Country code of Latvia
    LV,
    /// Country code of Libya
    LY,
    /// Country code of Morocco
    MA,
    /// Country code of Monaco
    MC,
    /// Country code of Moldova
    MD,
    /// Country code of Montenegro
    ME,
    /// Country code of Saint Martin (French Part)
    MF,
    /// Country code of Madagascar
    MG,
    /// Country code of Marshall Islands
    MH,
    /// Country code of Macedonia
    MK,
    /// Country code of Mali
    ML,
    /// Country code of Myanmar
    MM,
    /// Country code of Mongolia
    MN,
    /// Country code of Macao
    MO,
    /// Country code of Northern Mariana Islands
    MP,
    /// Country code of Martinique
    MQ,
    /// Country code of Mauritania
    MR,
    /// Country code of Montserrat
    MS,
    /// Country code of Malta
    MT,
    /// Country code of Mauritius
    MU,
    /// Country code of Maldives
    MV,
    /// Country code of Malawi
    MW,
    /// Country code of Mexico
    MX,
    /// Country code of Malaysia
    MY,
    /// Country code of Mozambique
    MZ,
    /// Country code of Namibia
    NA,
    /// Country code of New Caledonia
    NC,
    /// Country code of Niger
    NE,
    /// Country code of Norfolk Island
    NF,
    /// Country code of Nigeria
    NG,
    /// Country code of Nicaragua
    NI,
    /// Country code of Netherlands
    NL,
    /// Country code of Norway
    NO,
    /// Country code of Nepal
    NP,
    /// Country code of Nauru
    NR,
    /// Country code of Niue
    NU,
    /// Country code of New Zealand
    NZ,
    /// Country code of Oman
    OM,
    /// Country code of Panama
    PA,
    /// Country code of Peru
    PE,
    /// Country code of French Polynesia
    PF,
    /// Country code of Papua New Guinea
    PG,
    /// Country code of Philippines
    PH,
    /// Country code of Pakistan
    PK,
    /// Country code of Poland
    PL,
    /// Country code of Saint Pierre and Miquelon
    PM,
    /// Country code of Pitcairn
    PN,
    /// Country code of Puerto Rico
    PR,
    /// Country code of Palestinian Territory
    PS,
    /// Country code of Portugal
    PT,
    /// Country code of Palau
    PW,
    /// Country code of Paraguay
    PY,
    /// Country code of Qatar
    QA,
    /// Country code of Réunion
    RE,
    /// Country code of Romania
    RO,
    /// Country code of Serbia
    RS,
    /// Country code of Russia
    RU,
    /// Country code of Rwanda
    RW,
    /// Country code of Saudi Arabia
    SA,
    /// Country code of Solomon Islands
    SB,
    /// Country code of Seychelles
    SC,
    /// Country code of Sudan
    SD,
    /// Country code of Sweden
    SE,
    /// Country code of Singapore
    SG,
    /// Country code of Saint Helena, Ascension and Tristan Da Cunha
    SH,
    /// Country code of Slovenia
    SI,
    /// Country code of Svalbard and Jan Mayen
    SJ,
    /// Country code of Slovakia
    SK,
    /// Country code of Sierra Leone
    SL,
    /// Country code of San Marino
    SM,
    /// Country code of Senegal
    SN,
    /// Country code of Somalia
    SO,
    /// Country code of Suriname
    SR,
    /// Country code of South Sudan
    SS,
    /// Country code of Sao Tome and Principe
    ST,
    /// Country code of El Salvador
    SV,
    /// Country code of Sint Maarten (Dutch Part)
    SX,
    /// Country code of Syrian Arab Republic
    SY,
    /// Country code of Swaziland
    SZ,
    /// Country code of Turks and Caicos Islands
    TC,
    /// Country code of Chad
    TD,
    /// Country code of French Southern Territories
    TF,
    /// Country code of Togo
    TG,
    /// Country code of Thailand
    TH,
    /// Country code of Tajikistan
    TJ,
    /// Country code of Tokelau
    TK,
    /// Country code of Timor-Leste
    TL,
    /// Country code of Turkmenistan
    TM,
    /// Country code of Tunisia
    TN,
    /// Country code of Tonga
    TO,
    /// Country code of Turkey
    TR,
    /// Country code of Trinidad and Tobago
    TT,
    /// Country code of Tuvalu
    TV,
    /// Country code of Taiwan
    TW,
    /// Country code of Tanzania
    TZ,
    /// Country code of Ukraine
    UA,
    /// Country code of Uganda
    UG,
    /// Country code of United States Minor Outlying Islands
    UM,
    /// Country code of United States
    US,
    /// Country code of Uruguay
    UY,
    /// Country code of Uzbekistan
    UZ,
    /// Country code of Vatican City
    VA,
    /// Country code of Saint Vincent and The Grenadines
    VC,
    /// Country code of Venezuela
    VE,
    /// Country code of Virgin Islands, British
    VG,
    /// Country code of Virgin Islands, U.S.
    VI,
    /// Country code of Viet Nam
    VN,
    /// Country code of Vanuatu
    VU,
    /// Country code of Wallis and Futuna
    WF,
    /// Country code of Samoa
    WS,
    /// Country code of Yemen
    YE,
    /// Country code of Mayotte
    YT,
    /// Country code of South Africa
    ZA,
    /// Country code of Zambia
    ZM,
    /// Country code of Zimbabwe
    ZW,
    /// Country code of Kosovo
    XK,
}
