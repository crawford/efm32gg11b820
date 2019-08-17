#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0010_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_0000
    }
}
#[doc = "Possible values of the field `CLKOUTSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSEL0_A {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "ULFRCO (directly from oscillator)"]
    ULFRCO,
    #[doc = "LFRCO (directly from oscillator)"]
    LFRCO,
    #[doc = "LFXO (directly from oscillator)"]
    LFXO,
    #[doc = "HFXO (directly from oscillator)"]
    HFXO,
    #[doc = "HFEXPCLK"]
    HFEXPCLK,
    #[doc = "ULFRCO (qualified)"]
    ULFRCOQ,
    #[doc = "LFRCO (qualified)"]
    LFRCOQ,
    #[doc = "LFXO (qualified)"]
    LFXOQ,
    #[doc = "HFRCO (qualified)"]
    HFRCOQ,
    #[doc = "AUXHFRCO (qualified)"]
    AUXHFRCOQ,
    #[doc = "HFXO (qualified)"]
    HFXOQ,
    #[doc = "HFSRCCLK"]
    HFSRCCLK,
    #[doc = "USHFRCO (qualified)"]
    USHFRCOQ,
}
impl crate::ToBits<u8> for CLKOUTSEL0_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSEL0_A::DISABLED => 0,
            CLKOUTSEL0_A::ULFRCO => 1,
            CLKOUTSEL0_A::LFRCO => 2,
            CLKOUTSEL0_A::LFXO => 3,
            CLKOUTSEL0_A::HFXO => 6,
            CLKOUTSEL0_A::HFEXPCLK => 7,
            CLKOUTSEL0_A::ULFRCOQ => 9,
            CLKOUTSEL0_A::LFRCOQ => 10,
            CLKOUTSEL0_A::LFXOQ => 11,
            CLKOUTSEL0_A::HFRCOQ => 12,
            CLKOUTSEL0_A::AUXHFRCOQ => 13,
            CLKOUTSEL0_A::HFXOQ => 14,
            CLKOUTSEL0_A::HFSRCCLK => 15,
            CLKOUTSEL0_A::USHFRCOQ => 18,
        }
    }
}
#[doc = "Reader of field `CLKOUTSEL0`"]
pub type CLKOUTSEL0_R = crate::R<u8, CLKOUTSEL0_A>;
impl CLKOUTSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKOUTSEL0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKOUTSEL0_A::DISABLED),
            1 => Val(CLKOUTSEL0_A::ULFRCO),
            2 => Val(CLKOUTSEL0_A::LFRCO),
            3 => Val(CLKOUTSEL0_A::LFXO),
            6 => Val(CLKOUTSEL0_A::HFXO),
            7 => Val(CLKOUTSEL0_A::HFEXPCLK),
            9 => Val(CLKOUTSEL0_A::ULFRCOQ),
            10 => Val(CLKOUTSEL0_A::LFRCOQ),
            11 => Val(CLKOUTSEL0_A::LFXOQ),
            12 => Val(CLKOUTSEL0_A::HFRCOQ),
            13 => Val(CLKOUTSEL0_A::AUXHFRCOQ),
            14 => Val(CLKOUTSEL0_A::HFXOQ),
            15 => Val(CLKOUTSEL0_A::HFSRCCLK),
            18 => Val(CLKOUTSEL0_A::USHFRCOQ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL0_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL0_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL0_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFEXPCLK`"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL0_A::HFEXPCLK
    }
    #[doc = "Checks if the value of the field is `ULFRCOQ`"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::ULFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFRCOQ`"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::LFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFXOQ`"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL0_A::LFXOQ
    }
    #[doc = "Checks if the value of the field is `HFRCOQ`"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::HFRCOQ
    }
    #[doc = "Checks if the value of the field is `AUXHFRCOQ`"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::AUXHFRCOQ
    }
    #[doc = "Checks if the value of the field is `HFXOQ`"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL0_A::HFXOQ
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL0_A::HFSRCCLK
    }
    #[doc = "Checks if the value of the field is `USHFRCOQ`"]
    #[inline(always)]
    pub fn is_ushfrcoq(&self) -> bool {
        *self == CLKOUTSEL0_A::USHFRCOQ
    }
}
#[doc = "Write proxy for field `CLKOUTSEL0`"]
pub struct CLKOUTSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL0_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::DISABLED)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::ULFRCO)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::LFXO)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFXO)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFEXPCLK)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::ULFRCOQ)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::LFRCOQ)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::LFXOQ)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::AUXHFRCOQ)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFXOQ)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::HFSRCCLK)
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn ushfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL0_A::USHFRCOQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Possible values of the field `CLKOUTSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSEL1_A {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "ULFRCO (directly from oscillator)"]
    ULFRCO,
    #[doc = "LFRCO (directly from oscillator)"]
    LFRCO,
    #[doc = "LFXO (directly from oscillator)"]
    LFXO,
    #[doc = "HFXO (directly from oscillator)"]
    HFXO,
    #[doc = "HFEXPCLK"]
    HFEXPCLK,
    #[doc = "ULFRCO (qualified)"]
    ULFRCOQ,
    #[doc = "LFRCO (qualified)"]
    LFRCOQ,
    #[doc = "LFXO (qualified)"]
    LFXOQ,
    #[doc = "HFRCO (qualified)"]
    HFRCOQ,
    #[doc = "AUXHFRCO (qualified)"]
    AUXHFRCOQ,
    #[doc = "HFXO (qualified)"]
    HFXOQ,
    #[doc = "HFSRCCLK"]
    HFSRCCLK,
    #[doc = "USHFRCO (qualified)"]
    USHFRCOQ,
}
impl crate::ToBits<u8> for CLKOUTSEL1_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSEL1_A::DISABLED => 0,
            CLKOUTSEL1_A::ULFRCO => 1,
            CLKOUTSEL1_A::LFRCO => 2,
            CLKOUTSEL1_A::LFXO => 3,
            CLKOUTSEL1_A::HFXO => 6,
            CLKOUTSEL1_A::HFEXPCLK => 7,
            CLKOUTSEL1_A::ULFRCOQ => 9,
            CLKOUTSEL1_A::LFRCOQ => 10,
            CLKOUTSEL1_A::LFXOQ => 11,
            CLKOUTSEL1_A::HFRCOQ => 12,
            CLKOUTSEL1_A::AUXHFRCOQ => 13,
            CLKOUTSEL1_A::HFXOQ => 14,
            CLKOUTSEL1_A::HFSRCCLK => 15,
            CLKOUTSEL1_A::USHFRCOQ => 18,
        }
    }
}
#[doc = "Reader of field `CLKOUTSEL1`"]
pub type CLKOUTSEL1_R = crate::R<u8, CLKOUTSEL1_A>;
impl CLKOUTSEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKOUTSEL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKOUTSEL1_A::DISABLED),
            1 => Val(CLKOUTSEL1_A::ULFRCO),
            2 => Val(CLKOUTSEL1_A::LFRCO),
            3 => Val(CLKOUTSEL1_A::LFXO),
            6 => Val(CLKOUTSEL1_A::HFXO),
            7 => Val(CLKOUTSEL1_A::HFEXPCLK),
            9 => Val(CLKOUTSEL1_A::ULFRCOQ),
            10 => Val(CLKOUTSEL1_A::LFRCOQ),
            11 => Val(CLKOUTSEL1_A::LFXOQ),
            12 => Val(CLKOUTSEL1_A::HFRCOQ),
            13 => Val(CLKOUTSEL1_A::AUXHFRCOQ),
            14 => Val(CLKOUTSEL1_A::HFXOQ),
            15 => Val(CLKOUTSEL1_A::HFSRCCLK),
            18 => Val(CLKOUTSEL1_A::USHFRCOQ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL1_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL1_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL1_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL1_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFEXPCLK`"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL1_A::HFEXPCLK
    }
    #[doc = "Checks if the value of the field is `ULFRCOQ`"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::ULFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFRCOQ`"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::LFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFXOQ`"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL1_A::LFXOQ
    }
    #[doc = "Checks if the value of the field is `HFRCOQ`"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::HFRCOQ
    }
    #[doc = "Checks if the value of the field is `AUXHFRCOQ`"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::AUXHFRCOQ
    }
    #[doc = "Checks if the value of the field is `HFXOQ`"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL1_A::HFXOQ
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL1_A::HFSRCCLK
    }
    #[doc = "Checks if the value of the field is `USHFRCOQ`"]
    #[inline(always)]
    pub fn is_ushfrcoq(&self) -> bool {
        *self == CLKOUTSEL1_A::USHFRCOQ
    }
}
#[doc = "Write proxy for field `CLKOUTSEL1`"]
pub struct CLKOUTSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL1_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::DISABLED)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::ULFRCO)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFXO)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFXO)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFEXPCLK)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::ULFRCOQ)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFRCOQ)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::LFXOQ)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::AUXHFRCOQ)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFXOQ)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::HFSRCCLK)
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn ushfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL1_A::USHFRCOQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `CLKOUTSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTSEL2_A {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "ULFRCO (directly from oscillator)"]
    ULFRCO,
    #[doc = "LFRCO (directly from oscillator)"]
    LFRCO,
    #[doc = "LFXO (directly from oscillator)"]
    LFXO,
    #[doc = "HFXO divided by two (qualified)"]
    HFXODIV2Q,
    #[doc = "HFXO (directly from oscillator)"]
    HFXO,
    #[doc = "HFEXPCLK"]
    HFEXPCLK,
    #[doc = "HFXO doubler (qualified) (doubling activated by HFXOX2EN=1)"]
    HFXOX2Q,
    #[doc = "ULFRCO (qualified)"]
    ULFRCOQ,
    #[doc = "LFRCO (qualified)"]
    LFRCOQ,
    #[doc = "LFXO (qualified)"]
    LFXOQ,
    #[doc = "HFRCO (qualified)"]
    HFRCOQ,
    #[doc = "AUXHFRCO (qualified)"]
    AUXHFRCOQ,
    #[doc = "HFXO (qualified)"]
    HFXOQ,
    #[doc = "HFSRCCLK"]
    HFSRCCLK,
    #[doc = "USHFRCO (qualified)"]
    USHFRCOQ,
}
impl crate::ToBits<u8> for CLKOUTSEL2_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CLKOUTSEL2_A::DISABLED => 0,
            CLKOUTSEL2_A::ULFRCO => 1,
            CLKOUTSEL2_A::LFRCO => 2,
            CLKOUTSEL2_A::LFXO => 3,
            CLKOUTSEL2_A::HFXODIV2Q => 5,
            CLKOUTSEL2_A::HFXO => 6,
            CLKOUTSEL2_A::HFEXPCLK => 7,
            CLKOUTSEL2_A::HFXOX2Q => 8,
            CLKOUTSEL2_A::ULFRCOQ => 9,
            CLKOUTSEL2_A::LFRCOQ => 10,
            CLKOUTSEL2_A::LFXOQ => 11,
            CLKOUTSEL2_A::HFRCOQ => 12,
            CLKOUTSEL2_A::AUXHFRCOQ => 13,
            CLKOUTSEL2_A::HFXOQ => 14,
            CLKOUTSEL2_A::HFSRCCLK => 15,
            CLKOUTSEL2_A::USHFRCOQ => 18,
        }
    }
}
#[doc = "Reader of field `CLKOUTSEL2`"]
pub type CLKOUTSEL2_R = crate::R<u8, CLKOUTSEL2_A>;
impl CLKOUTSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKOUTSEL2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKOUTSEL2_A::DISABLED),
            1 => Val(CLKOUTSEL2_A::ULFRCO),
            2 => Val(CLKOUTSEL2_A::LFRCO),
            3 => Val(CLKOUTSEL2_A::LFXO),
            5 => Val(CLKOUTSEL2_A::HFXODIV2Q),
            6 => Val(CLKOUTSEL2_A::HFXO),
            7 => Val(CLKOUTSEL2_A::HFEXPCLK),
            8 => Val(CLKOUTSEL2_A::HFXOX2Q),
            9 => Val(CLKOUTSEL2_A::ULFRCOQ),
            10 => Val(CLKOUTSEL2_A::LFRCOQ),
            11 => Val(CLKOUTSEL2_A::LFXOQ),
            12 => Val(CLKOUTSEL2_A::HFRCOQ),
            13 => Val(CLKOUTSEL2_A::AUXHFRCOQ),
            14 => Val(CLKOUTSEL2_A::HFXOQ),
            15 => Val(CLKOUTSEL2_A::HFSRCCLK),
            18 => Val(CLKOUTSEL2_A::USHFRCOQ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKOUTSEL2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == CLKOUTSEL2_A::ULFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == CLKOUTSEL2_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == CLKOUTSEL2_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFXODIV2Q`"]
    #[inline(always)]
    pub fn is_hfxodiv2q(&self) -> bool {
        *self == CLKOUTSEL2_A::HFXODIV2Q
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == CLKOUTSEL2_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFEXPCLK`"]
    #[inline(always)]
    pub fn is_hfexpclk(&self) -> bool {
        *self == CLKOUTSEL2_A::HFEXPCLK
    }
    #[doc = "Checks if the value of the field is `HFXOX2Q`"]
    #[inline(always)]
    pub fn is_hfxox2q(&self) -> bool {
        *self == CLKOUTSEL2_A::HFXOX2Q
    }
    #[doc = "Checks if the value of the field is `ULFRCOQ`"]
    #[inline(always)]
    pub fn is_ulfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::ULFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFRCOQ`"]
    #[inline(always)]
    pub fn is_lfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::LFRCOQ
    }
    #[doc = "Checks if the value of the field is `LFXOQ`"]
    #[inline(always)]
    pub fn is_lfxoq(&self) -> bool {
        *self == CLKOUTSEL2_A::LFXOQ
    }
    #[doc = "Checks if the value of the field is `HFRCOQ`"]
    #[inline(always)]
    pub fn is_hfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::HFRCOQ
    }
    #[doc = "Checks if the value of the field is `AUXHFRCOQ`"]
    #[inline(always)]
    pub fn is_auxhfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::AUXHFRCOQ
    }
    #[doc = "Checks if the value of the field is `HFXOQ`"]
    #[inline(always)]
    pub fn is_hfxoq(&self) -> bool {
        *self == CLKOUTSEL2_A::HFXOQ
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == CLKOUTSEL2_A::HFSRCCLK
    }
    #[doc = "Checks if the value of the field is `USHFRCOQ`"]
    #[inline(always)]
    pub fn is_ushfrcoq(&self) -> bool {
        *self == CLKOUTSEL2_A::USHFRCOQ
    }
}
#[doc = "Write proxy for field `CLKOUTSEL2`"]
pub struct CLKOUTSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL2_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::DISABLED)
    }
    #[doc = "ULFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::ULFRCO)
    }
    #[doc = "LFRCO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::LFRCO)
    }
    #[doc = "LFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::LFXO)
    }
    #[doc = "HFXO divided by two (qualified)"]
    #[inline(always)]
    pub fn hfxodiv2q(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFXODIV2Q)
    }
    #[doc = "HFXO (directly from oscillator)"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFXO)
    }
    #[doc = "HFEXPCLK"]
    #[inline(always)]
    pub fn hfexpclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFEXPCLK)
    }
    #[doc = "HFXO doubler (qualified) (doubling activated by HFXOX2EN=1)"]
    #[inline(always)]
    pub fn hfxox2q(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFXOX2Q)
    }
    #[doc = "ULFRCO (qualified)"]
    #[inline(always)]
    pub fn ulfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::ULFRCOQ)
    }
    #[doc = "LFRCO (qualified)"]
    #[inline(always)]
    pub fn lfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::LFRCOQ)
    }
    #[doc = "LFXO (qualified)"]
    #[inline(always)]
    pub fn lfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::LFXOQ)
    }
    #[doc = "HFRCO (qualified)"]
    #[inline(always)]
    pub fn hfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFRCOQ)
    }
    #[doc = "AUXHFRCO (qualified)"]
    #[inline(always)]
    pub fn auxhfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::AUXHFRCOQ)
    }
    #[doc = "HFXO (qualified)"]
    #[inline(always)]
    pub fn hfxoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFXOQ)
    }
    #[doc = "HFSRCCLK"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::HFSRCCLK)
    }
    #[doc = "USHFRCO (qualified)"]
    #[inline(always)]
    pub fn ushfrcoq(self) -> &'a mut W {
        self.variant(CLKOUTSEL2_A::USHFRCOQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `WSHFLE`"]
pub type WSHFLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WSHFLE`"]
pub struct WSHFLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WSHFLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `HFPERCLKEN`"]
pub type HFPERCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFPERCLKEN`"]
pub struct HFPERCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFPERCLKEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&self) -> CLKOUTSEL0_R {
        CLKOUTSEL0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&self) -> CLKOUTSEL1_R {
        CLKOUTSEL1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Clock Output Select 2"]
    #[inline(always)]
    pub fn clkoutsel2(&self) -> CLKOUTSEL2_R {
        CLKOUTSEL2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Wait State for High-Frequency LE Interface"]
    #[inline(always)]
    pub fn wshfle(&self) -> WSHFLE_R {
        WSHFLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&self) -> HFPERCLKEN_R {
        HFPERCLKEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clock Output Select 0"]
    #[inline(always)]
    pub fn clkoutsel0(&mut self) -> CLKOUTSEL0_W {
        CLKOUTSEL0_W { w: self }
    }
    #[doc = "Bits 5:9 - Clock Output Select 1"]
    #[inline(always)]
    pub fn clkoutsel1(&mut self) -> CLKOUTSEL1_W {
        CLKOUTSEL1_W { w: self }
    }
    #[doc = "Bits 10:14 - Clock Output Select 2"]
    #[inline(always)]
    pub fn clkoutsel2(&mut self) -> CLKOUTSEL2_W {
        CLKOUTSEL2_W { w: self }
    }
    #[doc = "Bit 16 - Wait State for High-Frequency LE Interface"]
    #[inline(always)]
    pub fn wshfle(&mut self) -> WSHFLE_W {
        WSHFLE_W { w: self }
    }
    #[doc = "Bit 20 - HFPERCLK Enable"]
    #[inline(always)]
    pub fn hfperclken(&mut self) -> HFPERCLKEN_W {
        HFPERCLKEN_W { w: self }
    }
}
