#[doc = "Reader of register QSPICTRL"]
pub type R = crate::R<u32, super::QSPICTRL>;
#[doc = "Writer for register QSPICTRL"]
pub type W = crate::W<u32, super::QSPICTRL>;
#[doc = "Register QSPICTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::QSPICTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `QSPI0CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QSPI0CLKSEL_A {
    #[doc = "HFRCO clock is used to clock QSPI0"]
    HFRCO,
    #[doc = "HFXO clock is used to clock QSPI0"]
    HFXO,
    #[doc = "AUXHFRCO is used to clock QSPI0"]
    AUXHFRCO,
    #[doc = "USHFRCO is used to clock QSPI0"]
    USHFRCO,
}
impl crate::ToBits<u8> for QSPI0CLKSEL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            QSPI0CLKSEL_A::HFRCO => 0,
            QSPI0CLKSEL_A::HFXO => 1,
            QSPI0CLKSEL_A::AUXHFRCO => 2,
            QSPI0CLKSEL_A::USHFRCO => 3,
        }
    }
}
#[doc = "Reader of field `QSPI0CLKSEL`"]
pub type QSPI0CLKSEL_R = crate::R<u8, QSPI0CLKSEL_A>;
impl QSPI0CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QSPI0CLKSEL_A {
        match self.bits {
            0 => QSPI0CLKSEL_A::HFRCO,
            1 => QSPI0CLKSEL_A::HFXO,
            2 => QSPI0CLKSEL_A::AUXHFRCO,
            3 => QSPI0CLKSEL_A::USHFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == QSPI0CLKSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == QSPI0CLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == QSPI0CLKSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == QSPI0CLKSEL_A::USHFRCO
    }
}
#[doc = "Write proxy for field `QSPI0CLKSEL`"]
pub struct QSPI0CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI0CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPI0CLKSEL_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "HFRCO clock is used to clock QSPI0"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(QSPI0CLKSEL_A::HFRCO)
    }
    #[doc = "HFXO clock is used to clock QSPI0"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(QSPI0CLKSEL_A::HFXO)
    }
    #[doc = "AUXHFRCO is used to clock QSPI0"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(QSPI0CLKSEL_A::AUXHFRCO)
    }
    #[doc = "USHFRCO is used to clock QSPI0"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(QSPI0CLKSEL_A::USHFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `QSPI0CLKDIS`"]
pub type QSPI0CLKDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QSPI0CLKDIS`"]
pub struct QSPI0CLKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI0CLKDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - QSPI0 Reference Clock Select"]
    #[inline(always)]
    pub fn qspi0clksel(&self) -> QSPI0CLKSEL_R {
        QSPI0CLKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 7 - QSPI0 Reference Clock Disable"]
    #[inline(always)]
    pub fn qspi0clkdis(&self) -> QSPI0CLKDIS_R {
        QSPI0CLKDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - QSPI0 Reference Clock Select"]
    #[inline(always)]
    pub fn qspi0clksel(&mut self) -> QSPI0CLKSEL_W {
        QSPI0CLKSEL_W { w: self }
    }
    #[doc = "Bit 7 - QSPI0 Reference Clock Disable"]
    #[inline(always)]
    pub fn qspi0clkdis(&mut self) -> QSPI0CLKDIS_W {
        QSPI0CLKDIS_W { w: self }
    }
}
