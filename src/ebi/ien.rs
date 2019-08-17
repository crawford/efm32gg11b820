#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VSYNC`"]
pub type VSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSYNC`"]
pub struct VSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_W<'a> {
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
#[doc = "Reader of field `HSYNC`"]
pub type HSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSYNC`"]
pub struct HSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNC_W<'a> {
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
#[doc = "Reader of field `VBPORCH`"]
pub type VBPORCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBPORCH`"]
pub struct VBPORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> VBPORCH_W<'a> {
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
#[doc = "Reader of field `VFPORCH`"]
pub type VFPORCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VFPORCH`"]
pub struct VFPORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> VFPORCH_W<'a> {
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
#[doc = "Reader of field `DDEMPTY`"]
pub type DDEMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDEMPTY`"]
pub struct DDEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> DDEMPTY_W<'a> {
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
#[doc = "Reader of field `DDJIT`"]
pub type DDJIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDJIT`"]
pub struct DDJIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DDJIT_W<'a> {
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
#[doc = "Reader of field `TFTPIXEL0EMPTY`"]
pub type TFTPIXEL0EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFTPIXEL0EMPTY`"]
pub struct TFTPIXEL0EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TFTPIXEL0EMPTY_W<'a> {
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
#[doc = "Reader of field `TFTPIXEL1EMPTY`"]
pub type TFTPIXEL1EMPTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFTPIXEL1EMPTY`"]
pub struct TFTPIXEL1EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TFTPIXEL1EMPTY_W<'a> {
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
#[doc = "Reader of field `TFTPIXELFULL`"]
pub type TFTPIXELFULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFTPIXELFULL`"]
pub struct TFTPIXELFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> TFTPIXELFULL_W<'a> {
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
#[doc = "Reader of field `TFTPIXELOF`"]
pub type TFTPIXELOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFTPIXELOF`"]
pub struct TFTPIXELOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TFTPIXELOF_W<'a> {
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
    #[doc = "Bit 0 - Vertical Sync Interrupt Enable"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Enable"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Enable"]
    #[inline(always)]
    pub fn vbporch(&self) -> VBPORCH_R {
        VBPORCH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Enable"]
    #[inline(always)]
    pub fn vfporch(&self) -> VFPORCH_R {
        VFPORCH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Enable"]
    #[inline(always)]
    pub fn ddempty(&self) -> DDEMPTY_R {
        DDEMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Enable"]
    #[inline(always)]
    pub fn ddjit(&self) -> DDJIT_R {
        DDJIT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EBI_TFTPIXEL0 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixel0empty(&self) -> TFTPIXEL0EMPTY_R {
        TFTPIXEL0EMPTY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EBI_TFTPIXEL1 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixel1empty(&self) -> TFTPIXEL1EMPTY_R {
        TFTPIXEL1EMPTY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL Full Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixelfull(&self) -> TFTPIXELFULL_R {
        TFTPIXELFULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixelof(&self) -> TFTPIXELOF_R {
        TFTPIXELOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Vertical Sync Interrupt Enable"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VSYNC_W {
        VSYNC_W { w: self }
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Enable"]
    #[inline(always)]
    pub fn hsync(&mut self) -> HSYNC_W {
        HSYNC_W { w: self }
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Enable"]
    #[inline(always)]
    pub fn vbporch(&mut self) -> VBPORCH_W {
        VBPORCH_W { w: self }
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Enable"]
    #[inline(always)]
    pub fn vfporch(&mut self) -> VFPORCH_W {
        VFPORCH_W { w: self }
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Enable"]
    #[inline(always)]
    pub fn ddempty(&mut self) -> DDEMPTY_W {
        DDEMPTY_W { w: self }
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Enable"]
    #[inline(always)]
    pub fn ddjit(&mut self) -> DDJIT_W {
        DDJIT_W { w: self }
    }
    #[doc = "Bit 6 - EBI_TFTPIXEL0 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixel0empty(&mut self) -> TFTPIXEL0EMPTY_W {
        TFTPIXEL0EMPTY_W { w: self }
    }
    #[doc = "Bit 7 - EBI_TFTPIXEL1 Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixel1empty(&mut self) -> TFTPIXEL1EMPTY_W {
        TFTPIXEL1EMPTY_W { w: self }
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL Full Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixelfull(&mut self) -> TFTPIXELFULL_W {
        TFTPIXELFULL_W { w: self }
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn tftpixelof(&mut self) -> TFTPIXELOF_W {
        TFTPIXELOF_W { w: self }
    }
}
