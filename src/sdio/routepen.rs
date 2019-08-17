#[doc = "Reader of register ROUTEPEN"]
pub type R = crate::R<u32, super::ROUTEPEN>;
#[doc = "Writer for register ROUTEPEN"]
pub type W = crate::W<u32, super::ROUTEPEN>;
#[doc = "Register ROUTEPEN `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTEPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLKPEN`"]
pub type CLKPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKPEN`"]
pub struct CLKPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPEN_W<'a> {
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
#[doc = "Reader of field `CMDPEN`"]
pub type CMDPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDPEN`"]
pub struct CMDPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDPEN_W<'a> {
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
#[doc = "Reader of field `D0PEN`"]
pub type D0PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D0PEN`"]
pub struct D0PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D0PEN_W<'a> {
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
#[doc = "Reader of field `D1PEN`"]
pub type D1PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D1PEN`"]
pub struct D1PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D1PEN_W<'a> {
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
#[doc = "Reader of field `D2PEN`"]
pub type D2PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D2PEN`"]
pub struct D2PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D2PEN_W<'a> {
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
#[doc = "Reader of field `D3PEN`"]
pub type D3PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D3PEN`"]
pub struct D3PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D3PEN_W<'a> {
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
#[doc = "Reader of field `D4PEN`"]
pub type D4PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D4PEN`"]
pub struct D4PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D4PEN_W<'a> {
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
#[doc = "Reader of field `D5PEN`"]
pub type D5PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D5PEN`"]
pub struct D5PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D5PEN_W<'a> {
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
#[doc = "Reader of field `D6PEN`"]
pub type D6PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D6PEN`"]
pub struct D6PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D6PEN_W<'a> {
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
#[doc = "Reader of field `D7PEN`"]
pub type D7PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `D7PEN`"]
pub struct D7PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> D7PEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CLK I/O Enable"]
    #[inline(always)]
    pub fn clkpen(&self) -> CLKPEN_R {
        CLKPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CMD I/O Enable"]
    #[inline(always)]
    pub fn cmdpen(&self) -> CMDPEN_R {
        CMDPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Dat0 I/O Enable"]
    #[inline(always)]
    pub fn d0pen(&self) -> D0PEN_R {
        D0PEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Dat1 I/O Enable"]
    #[inline(always)]
    pub fn d1pen(&self) -> D1PEN_R {
        D1PEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Dat2 I/O Enable"]
    #[inline(always)]
    pub fn d2pen(&self) -> D2PEN_R {
        D2PEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Dat3 I/O Enable"]
    #[inline(always)]
    pub fn d3pen(&self) -> D3PEN_R {
        D3PEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Dat4 I/O Enable"]
    #[inline(always)]
    pub fn d4pen(&self) -> D4PEN_R {
        D4PEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Dat5 Enable"]
    #[inline(always)]
    pub fn d5pen(&self) -> D5PEN_R {
        D5PEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Dat6 Enable"]
    #[inline(always)]
    pub fn d6pen(&self) -> D6PEN_R {
        D6PEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Data7 I/O Enable"]
    #[inline(always)]
    pub fn d7pen(&self) -> D7PEN_R {
        D7PEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLK I/O Enable"]
    #[inline(always)]
    pub fn clkpen(&mut self) -> CLKPEN_W {
        CLKPEN_W { w: self }
    }
    #[doc = "Bit 1 - CMD I/O Enable"]
    #[inline(always)]
    pub fn cmdpen(&mut self) -> CMDPEN_W {
        CMDPEN_W { w: self }
    }
    #[doc = "Bit 2 - Dat0 I/O Enable"]
    #[inline(always)]
    pub fn d0pen(&mut self) -> D0PEN_W {
        D0PEN_W { w: self }
    }
    #[doc = "Bit 3 - Dat1 I/O Enable"]
    #[inline(always)]
    pub fn d1pen(&mut self) -> D1PEN_W {
        D1PEN_W { w: self }
    }
    #[doc = "Bit 4 - Dat2 I/O Enable"]
    #[inline(always)]
    pub fn d2pen(&mut self) -> D2PEN_W {
        D2PEN_W { w: self }
    }
    #[doc = "Bit 5 - Dat3 I/O Enable"]
    #[inline(always)]
    pub fn d3pen(&mut self) -> D3PEN_W {
        D3PEN_W { w: self }
    }
    #[doc = "Bit 6 - Dat4 I/O Enable"]
    #[inline(always)]
    pub fn d4pen(&mut self) -> D4PEN_W {
        D4PEN_W { w: self }
    }
    #[doc = "Bit 7 - Dat5 Enable"]
    #[inline(always)]
    pub fn d5pen(&mut self) -> D5PEN_W {
        D5PEN_W { w: self }
    }
    #[doc = "Bit 8 - Dat6 Enable"]
    #[inline(always)]
    pub fn d6pen(&mut self) -> D6PEN_W {
        D6PEN_W { w: self }
    }
    #[doc = "Bit 9 - Data7 I/O Enable"]
    #[inline(always)]
    pub fn d7pen(&mut self) -> D7PEN_W {
        D7PEN_W { w: self }
    }
}
