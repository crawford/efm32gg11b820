#[doc = "Reader of register HFCLKSTATUS"]
pub type R = crate::R<u32, super::HFCLKSTATUS>;
#[doc = "HFCLK Selected\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECTED_A {
    #[doc = "1: HFRCO is selected as HFCLK clock source"]
    HFRCO,
    #[doc = "2: HFXO is selected as HFCLK clock source"]
    HFXO,
    #[doc = "3: LFRCO is selected as HFCLK clock source"]
    LFRCO,
    #[doc = "4: LFXO is selected as HFCLK clock source"]
    LFXO,
    #[doc = "5: HFRCO divided by 2 is selected as HFCLK clock source"]
    HFRCODIV2,
    #[doc = "6: USHFRCO is selected as HFCLK clock source"]
    USHFRCO,
    #[doc = "7: CLKIN0 is selected as HFCLK clock source"]
    CLKIN0,
}
impl From<SELECTED_A> for u8 {
    #[inline(always)]
    fn from(variant: SELECTED_A) -> Self {
        match variant {
            SELECTED_A::HFRCO => 1,
            SELECTED_A::HFXO => 2,
            SELECTED_A::LFRCO => 3,
            SELECTED_A::LFXO => 4,
            SELECTED_A::HFRCODIV2 => 5,
            SELECTED_A::USHFRCO => 6,
            SELECTED_A::CLKIN0 => 7,
        }
    }
}
#[doc = "Reader of field `SELECTED`"]
pub type SELECTED_R = crate::R<u8, SELECTED_A>;
impl SELECTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SELECTED_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SELECTED_A::HFRCO),
            2 => Val(SELECTED_A::HFXO),
            3 => Val(SELECTED_A::LFRCO),
            4 => Val(SELECTED_A::LFXO),
            5 => Val(SELECTED_A::HFRCODIV2),
            6 => Val(SELECTED_A::USHFRCO),
            7 => Val(SELECTED_A::CLKIN0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == SELECTED_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == SELECTED_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == SELECTED_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == SELECTED_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCODIV2`"]
    #[inline(always)]
    pub fn is_hfrcodiv2(&self) -> bool {
        *self == SELECTED_A::HFRCODIV2
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == SELECTED_A::USHFRCO
    }
    #[doc = "Checks if the value of the field is `CLKIN0`"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == SELECTED_A::CLKIN0
    }
}
impl R {
    #[doc = "Bits 0:2 - HFCLK Selected"]
    #[inline(always)]
    pub fn selected(&self) -> SELECTED_R {
        SELECTED_R::new((self.bits & 0x07) as u8)
    }
}
