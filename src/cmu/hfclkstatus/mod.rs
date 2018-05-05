#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HFCLKSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `SELECTED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECTEDR {
    #[doc = "HFRCO is selected as HFCLK clock source"]
    HFRCO,
    #[doc = "HFXO is selected as HFCLK clock source"]
    HFXO,
    #[doc = "LFRCO is selected as HFCLK clock source"]
    LFRCO,
    #[doc = "LFXO is selected as HFCLK clock source"]
    LFXO,
    #[doc = "HFRCO divided by 2 is selected as HFCLK clock source"]
    HFRCODIV2,
    #[doc = "USHFRCO is selected as HFCLK clock source"]
    USHFRCO,
    #[doc = "CLKIN0 is selected as HFCLK clock source"]
    CLKIN0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SELECTEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SELECTEDR::HFRCO => 1,
            SELECTEDR::HFXO => 2,
            SELECTEDR::LFRCO => 3,
            SELECTEDR::LFXO => 4,
            SELECTEDR::HFRCODIV2 => 5,
            SELECTEDR::USHFRCO => 6,
            SELECTEDR::CLKIN0 => 7,
            SELECTEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SELECTEDR {
        match value {
            1 => SELECTEDR::HFRCO,
            2 => SELECTEDR::HFXO,
            3 => SELECTEDR::LFRCO,
            4 => SELECTEDR::LFXO,
            5 => SELECTEDR::HFRCODIV2,
            6 => SELECTEDR::USHFRCO,
            7 => SELECTEDR::CLKIN0,
            i => SELECTEDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline]
    pub fn is_hfrco(&self) -> bool {
        *self == SELECTEDR::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == SELECTEDR::HFXO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline]
    pub fn is_lfrco(&self) -> bool {
        *self == SELECTEDR::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline]
    pub fn is_lfxo(&self) -> bool {
        *self == SELECTEDR::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCODIV2`"]
    #[inline]
    pub fn is_hfrcodiv2(&self) -> bool {
        *self == SELECTEDR::HFRCODIV2
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline]
    pub fn is_ushfrco(&self) -> bool {
        *self == SELECTEDR::USHFRCO
    }
    #[doc = "Checks if the value of the field is `CLKIN0`"]
    #[inline]
    pub fn is_clkin0(&self) -> bool {
        *self == SELECTEDR::CLKIN0
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - HFCLK Selected"]
    #[inline]
    pub fn selected(&self) -> SELECTEDR {
        SELECTEDR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
