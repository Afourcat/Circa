use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tristate {
    Low,
    High,
    Floating,
    Error,
}

impl Tristate {
    pub fn merge(self, rhs: Tristate) -> Tristate {
        if self == rhs {
            self
        } else if self == Tristate::Error || rhs == Tristate::Error {
            Tristate::Error
        } else if self == Tristate::Floating {
            rhs
        } else if rhs == Tristate::Floating {
            self
        } else {
            Tristate::Error
        }
    }
}

impl BitAnd for Tristate {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        if self == Tristate::Error || rhs == Tristate::Error {
            Tristate::Error
        } else if self == Tristate::Floating || rhs == Tristate::Floating {
            Tristate::Floating
        } else if self == Tristate::High && rhs == Tristate::High {
            Tristate::High
        } else {
            Tristate::Low
        }
    }
}

impl BitOr for Tristate {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        if self == Tristate::Error || rhs == Tristate::Error {
            Tristate::Error
        } else if self == Tristate::Floating || rhs == Tristate::Floating {
            Tristate::Floating
        } else if self == Tristate::High || rhs == Tristate::High {
            Tristate::High
        } else {
            Tristate::Low
        }
    }
}

impl BitXor for Tristate {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        if self == Tristate::Error || rhs == Tristate::Error {
            Tristate::Error
        } else if self == Tristate::Floating || rhs == Tristate::Floating {
            Tristate::Floating
        } else if self != rhs {
            Tristate::High
        } else {
            Tristate::Low
        }
    }
}

impl Not for Tristate {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Tristate::Low      => Tristate::High,
            Tristate::High     => Tristate::Low,
            x                  => x,
        }
    }
}

impl Default for Tristate {
    fn default() -> Self {
        Tristate::Floating
    }
}
