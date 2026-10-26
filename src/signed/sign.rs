use core::{fmt, ops::Neg};

/// The sign of an [`Int`](crate::Int).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum Sign {
    /// Less than zero.
    Negative = -1,
    /// Greater than or equal to zero.
    Positive = 1,
}

impl Sign {
    /// Returns `true` if the sign is [`Sign::Positive`].
    #[inline]
    #[must_use]
    pub const fn is_positive(self) -> bool {
        matches!(self, Self::Positive)
    }

    /// Returns `true` if the sign is [`Sign::Negative`].
    #[inline]
    #[must_use]
    pub const fn is_negative(self) -> bool {
        matches!(self, Self::Negative)
    }
}

impl Neg for Sign {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
        }
    }
}

impl fmt::Display for Sign {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Positive => "+",
            Self::Negative => "-",
        })
    }
}
