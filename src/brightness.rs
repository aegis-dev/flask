
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Brightness {
    Normal  = 0,
    Dim     = 1,
    Dark    = 2,
    Black   = 3,
}

impl Brightness {
    #[inline(always)]
    pub fn is_lighter(&self, other: Brightness) -> bool {
        (*self as u8) < (other as u8)
    }
    
    #[inline(always)]
    pub fn is_darker(&self, other: Brightness) -> bool {
        !self.is_lighter(other)
    }
}