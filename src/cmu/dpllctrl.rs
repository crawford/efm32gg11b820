#[doc = "Reader of register DPLLCTRL"]
pub type R = crate::R<u32, super::DPLLCTRL>;
#[doc = "Writer for register DPLLCTRL"]
pub type W = crate::W<u32, super::DPLLCTRL>;
#[doc = "Register DPLLCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DPLLCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `EDGESEL`"]
pub type EDGESEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGESEL`"]
pub struct EDGESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGESEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `AUTORECOVER`"]
pub type AUTORECOVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTORECOVER`"]
pub struct AUTORECOVER_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTORECOVER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reference Clock Selection Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: HFXO selected"]
    HFXO = 0,
    #[doc = "1: LFXO selected"]
    LFXO = 1,
    #[doc = "2: USHFRCO selected"]
    USHFRCO = 2,
    #[doc = "3: CLKIN0 selected"]
    CLKIN0 = 3,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::HFXO,
            1 => REFSEL_A::LFXO,
            2 => REFSEL_A::USHFRCO,
            3 => REFSEL_A::CLKIN0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == REFSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == REFSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == REFSEL_A::USHFRCO
    }
    #[doc = "Checks if the value of the field is `CLKIN0`"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == REFSEL_A::CLKIN0
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "HFXO selected"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(REFSEL_A::HFXO)
    }
    #[doc = "LFXO selected"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(REFSEL_A::LFXO)
    }
    #[doc = "USHFRCO selected"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(REFSEL_A::USHFRCO)
    }
    #[doc = "CLKIN0 selected"]
    #[inline(always)]
    pub fn clkin0(self) -> &'a mut W {
        self.variant(REFSEL_A::CLKIN0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `DITHEN`"]
pub type DITHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DITHEN`"]
pub struct DITHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Operating Mode Control"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reference Edge Select"]
    #[inline(always)]
    pub fn edgesel(&self) -> EDGESEL_R {
        EDGESEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Automatic Recovery Ctrl"]
    #[inline(always)]
    pub fn autorecover(&self) -> AUTORECOVER_R {
        AUTORECOVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Reference Clock Selection Control"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Dither Enable Control"]
    #[inline(always)]
    pub fn dithen(&self) -> DITHEN_R {
        DITHEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operating Mode Control"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 1 - Reference Edge Select"]
    #[inline(always)]
    pub fn edgesel(&mut self) -> EDGESEL_W {
        EDGESEL_W { w: self }
    }
    #[doc = "Bit 2 - Automatic Recovery Ctrl"]
    #[inline(always)]
    pub fn autorecover(&mut self) -> AUTORECOVER_W {
        AUTORECOVER_W { w: self }
    }
    #[doc = "Bits 3:4 - Reference Clock Selection Control"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bit 6 - Dither Enable Control"]
    #[inline(always)]
    pub fn dithen(&mut self) -> DITHEN_W {
        DITHEN_W { w: self }
    }
}
