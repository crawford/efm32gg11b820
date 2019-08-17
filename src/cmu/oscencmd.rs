#[doc = "Writer for register OSCENCMD"]
pub type W = crate::W<u32, super::OSCENCMD>;
#[doc = "Register OSCENCMD `reset()`'s with value 0"]
impl crate::ResetValue for super::OSCENCMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `HFRCOEN`"]
pub struct HFRCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRCOEN_W<'a> {
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
#[doc = "Write proxy for field `HFRCODIS`"]
pub struct HFRCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRCODIS_W<'a> {
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
#[doc = "Write proxy for field `HFXOEN`"]
pub struct HFXOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOEN_W<'a> {
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
#[doc = "Write proxy for field `HFXODIS`"]
pub struct HFXODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `AUXHFRCOEN`"]
pub struct AUXHFRCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXHFRCOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `AUXHFRCODIS`"]
pub struct AUXHFRCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXHFRCODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `LFRCOEN`"]
pub struct LFRCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFRCOEN_W<'a> {
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
#[doc = "Write proxy for field `LFRCODIS`"]
pub struct LFRCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LFRCODIS_W<'a> {
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
#[doc = "Write proxy for field `LFXOEN`"]
pub struct LFXOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `LFXODIS`"]
pub struct LFXODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `USHFRCOEN`"]
pub struct USHFRCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USHFRCOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `USHFRCODIS`"]
pub struct USHFRCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> USHFRCODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `DPLLEN`"]
pub struct DPLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `DPLLDIS`"]
pub struct DPLLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DPLLDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - HFRCO Enable"]
    #[inline(always)]
    pub fn hfrcoen(&mut self) -> HFRCOEN_W {
        HFRCOEN_W { w: self }
    }
    #[doc = "Bit 1 - HFRCO Disable"]
    #[inline(always)]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W {
        HFRCODIS_W { w: self }
    }
    #[doc = "Bit 2 - HFXO Enable"]
    #[inline(always)]
    pub fn hfxoen(&mut self) -> HFXOEN_W {
        HFXOEN_W { w: self }
    }
    #[doc = "Bit 3 - HFXO Disable"]
    #[inline(always)]
    pub fn hfxodis(&mut self) -> HFXODIS_W {
        HFXODIS_W { w: self }
    }
    #[doc = "Bit 4 - AUXHFRCO Enable"]
    #[inline(always)]
    pub fn auxhfrcoen(&mut self) -> AUXHFRCOEN_W {
        AUXHFRCOEN_W { w: self }
    }
    #[doc = "Bit 5 - AUXHFRCO Disable"]
    #[inline(always)]
    pub fn auxhfrcodis(&mut self) -> AUXHFRCODIS_W {
        AUXHFRCODIS_W { w: self }
    }
    #[doc = "Bit 6 - LFRCO Enable"]
    #[inline(always)]
    pub fn lfrcoen(&mut self) -> LFRCOEN_W {
        LFRCOEN_W { w: self }
    }
    #[doc = "Bit 7 - LFRCO Disable"]
    #[inline(always)]
    pub fn lfrcodis(&mut self) -> LFRCODIS_W {
        LFRCODIS_W { w: self }
    }
    #[doc = "Bit 8 - LFXO Enable"]
    #[inline(always)]
    pub fn lfxoen(&mut self) -> LFXOEN_W {
        LFXOEN_W { w: self }
    }
    #[doc = "Bit 9 - LFXO Disable"]
    #[inline(always)]
    pub fn lfxodis(&mut self) -> LFXODIS_W {
        LFXODIS_W { w: self }
    }
    #[doc = "Bit 10 - USHFRCO Enable"]
    #[inline(always)]
    pub fn ushfrcoen(&mut self) -> USHFRCOEN_W {
        USHFRCOEN_W { w: self }
    }
    #[doc = "Bit 11 - USHFRCO Disable"]
    #[inline(always)]
    pub fn ushfrcodis(&mut self) -> USHFRCODIS_W {
        USHFRCODIS_W { w: self }
    }
    #[doc = "Bit 12 - DPLL Enable"]
    #[inline(always)]
    pub fn dpllen(&mut self) -> DPLLEN_W {
        DPLLEN_W { w: self }
    }
    #[doc = "Bit 13 - DPLL Disable"]
    #[inline(always)]
    pub fn dplldis(&mut self) -> DPLLDIS_W {
        DPLLDIS_W { w: self }
    }
}
