#[doc = "Reader of register RDTIMING2"]
pub type R = crate::R<u32, super::RDTIMING2>;
#[doc = "Writer for register RDTIMING2"]
pub type W = crate::W<u32, super::RDTIMING2>;
#[doc = "Register RDTIMING2 `reset()`'s with value 0x0007_7f07"]
impl crate::ResetValue for super::RDTIMING2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0007_7f07
    }
}
#[doc = "Reader of field `RDSETUP`"]
pub type RDSETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDSETUP`"]
pub struct RDSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `RDSTRB`"]
pub type RDSTRB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDSTRB`"]
pub struct RDSTRB_W<'a> {
    w: &'a mut W,
}
impl<'a> RDSTRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `RDHOLD`"]
pub type RDHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDHOLD`"]
pub struct RDHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RDHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `HALFRE`"]
pub type HALFRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALFRE`"]
pub struct HALFRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HALFRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `PREFETCH`"]
pub type PREFETCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREFETCH`"]
pub struct PREFETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFETCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `PAGEMODE`"]
pub type PAGEMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAGEMODE`"]
pub struct PAGEMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGEMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Read Setup Time"]
    #[inline(always)]
    pub fn rdsetup(&self) -> RDSETUP_R {
        RDSETUP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:14 - Read Strobe Time"]
    #[inline(always)]
    pub fn rdstrb(&self) -> RDSTRB_R {
        RDSTRB_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - Read Hold Time"]
    #[inline(always)]
    pub fn rdhold(&self) -> RDHOLD_R {
        RDHOLD_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Half Cycle REn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfre(&self) -> HALFRE_R {
        HALFRE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Prefetch Enable"]
    #[inline(always)]
    pub fn prefetch(&self) -> PREFETCH_R {
        PREFETCH_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Page Mode Access Enable"]
    #[inline(always)]
    pub fn pagemode(&self) -> PAGEMODE_R {
        PAGEMODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read Setup Time"]
    #[inline(always)]
    pub fn rdsetup(&mut self) -> RDSETUP_W {
        RDSETUP_W { w: self }
    }
    #[doc = "Bits 8:14 - Read Strobe Time"]
    #[inline(always)]
    pub fn rdstrb(&mut self) -> RDSTRB_W {
        RDSTRB_W { w: self }
    }
    #[doc = "Bits 16:18 - Read Hold Time"]
    #[inline(always)]
    pub fn rdhold(&mut self) -> RDHOLD_W {
        RDHOLD_W { w: self }
    }
    #[doc = "Bit 28 - Half Cycle REn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfre(&mut self) -> HALFRE_W {
        HALFRE_W { w: self }
    }
    #[doc = "Bit 29 - Prefetch Enable"]
    #[inline(always)]
    pub fn prefetch(&mut self) -> PREFETCH_W {
        PREFETCH_W { w: self }
    }
    #[doc = "Bit 30 - Page Mode Access Enable"]
    #[inline(always)]
    pub fn pagemode(&mut self) -> PAGEMODE_W {
        PAGEMODE_W { w: self }
    }
}
