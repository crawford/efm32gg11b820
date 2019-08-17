#[doc = "Reader of register MIR0_MASK"]
pub type R = crate::R<u32, super::MIR0_MASK>;
#[doc = "Writer for register MIR0_MASK"]
pub type W = crate::W<u32, super::MIR0_MASK>;
#[doc = "Register MIR0_MASK `reset()`'s with value 0xdfff_ffff"]
impl crate::ResetValue for super::MIR0_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xdfff_ffff
    }
}
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `MDIR`"]
pub type MDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDIR`"]
pub struct MDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIR_W<'a> {
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
#[doc = "Reader of field `MXTD`"]
pub type MXTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MXTD`"]
pub struct MXTD_W<'a> {
    w: &'a mut W,
}
impl<'a> MXTD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - Identifier Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 30 - Mask Message Direction"]
    #[inline(always)]
    pub fn mdir(&self) -> MDIR_R {
        MDIR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Mask Extended Identifier"]
    #[inline(always)]
    pub fn mxtd(&self) -> MXTD_R {
        MXTD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Identifier Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bit 30 - Mask Message Direction"]
    #[inline(always)]
    pub fn mdir(&mut self) -> MDIR_W {
        MDIR_W { w: self }
    }
    #[doc = "Bit 31 - Mask Extended Identifier"]
    #[inline(always)]
    pub fn mxtd(&mut self) -> MXTD_W {
        MXTD_W { w: self }
    }
}
