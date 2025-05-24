//! Submodule handling the tokenization of greek letters in molecular formulas.

use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a greek letter in a molecular formula.
///
/// These are not ALL of the greek letters, but only those which are used in
/// mineral formulas, i.e. `'α'`, `'β'`, `'γ'`, `'δ'`, `'φ'`, `'ω'`, `'λ'`,
/// `'μ'`, and `'π'`.
pub enum GreekLetter {
    /// Alpha
    Alpha,
    /// Beta
    Beta,
    /// Gamma
    Gamma,
    /// Delta
    Delta,
    /// Phi
    Phi,
    /// Omega
    Omega,
    /// Lambda
    Lambda,
    /// Mu
    Mu,
    /// Pi
    Pi,
}

impl GreekLetter {
    #[must_use]
    /// Returns whether the provided character is a valid greek letter.
    pub fn is_greek_letter(c: char) -> bool {
        matches!(
            c,
            '\u{03b1}' // α
            | 'β' // β
            | '\u{03b3}' // γ
            | 'δ' // δ
            | 'φ' // φ
            | 'ω' // ω
            | 'λ' // λ
            | 'μ' // μ
            | 'π' // π
        )
    }
}

impl AsRef<char> for GreekLetter {
    fn as_ref(&self) -> &char {
        match self {
            GreekLetter::Alpha => &'\u{03b1}',
            GreekLetter::Beta => &'β',
            GreekLetter::Gamma => &'\u{03b3}',
            GreekLetter::Delta => &'δ',
            GreekLetter::Phi => &'φ',
            GreekLetter::Omega => &'ω',
            GreekLetter::Lambda => &'λ',
            GreekLetter::Mu => &'μ',
            GreekLetter::Pi => &'π',
        }
    }
}

impl From<GreekLetter> for char {
    fn from(letter: GreekLetter) -> Self {
        *letter.as_ref()
    }
}

impl Display for GreekLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl FromStr for GreekLetter {
    type Err = crate::errors::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let candidate =
            chars.next().ok_or_else(|| crate::errors::Error::InvalidGreekLetter(s.to_string()))?;
        if chars.next().is_some() {
            return Err(crate::errors::Error::InvalidGreekLetter(s.to_string()));
        }

        Self::try_from(candidate)
    }
}

impl TryFrom<&str> for GreekLetter {
    type Error = crate::errors::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl TryFrom<String> for GreekLetter {
    type Error = crate::errors::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl TryFrom<char> for GreekLetter {
    type Error = crate::errors::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '\u{03b1}' => Ok(GreekLetter::Alpha),
            'β' => Ok(GreekLetter::Beta),
            '\u{03b3}' => Ok(GreekLetter::Gamma),
            'δ' => Ok(GreekLetter::Delta),
            'φ' => Ok(GreekLetter::Phi),
            'ω' => Ok(GreekLetter::Omega),
            'λ' => Ok(GreekLetter::Lambda),
            'μ' => Ok(GreekLetter::Mu),
            'π' => Ok(GreekLetter::Pi),
            _ => Err(crate::errors::Error::InvalidGreekLetter(value.to_string())),
        }
    }
}
