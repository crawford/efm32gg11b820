#[doc = "Reader of register TFTVPORCH"]
pub type R = crate::R<u32, super::TFTVPORCH>;
#[doc = "Writer for register TFTVPORCH"]
pub type W = crate::W<u32, super::TFTVPORCH>;
#[doc = "Register TFTVPORCH `reset()`'s with value 0"]
impl crate::ResetValue for super::TFTVPORCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VSYNC`"]
pub type VSYNC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSYNC`"]
pub struct VSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `VFPORCH`"]
pub type VFPORCH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VFPORCH`"]
pub struct VFPORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> VFPORCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | (((value as u32) & 0x0fff) << 8);
        self.w
    }
}
#[doc = "Reader of field `VBPORCH`"]
pub type VBPORCH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VBPORCH`"]
pub struct VBPORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> VBPORCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 20)) | (((value as u32) & 0x0fff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Vertical Synchronization Pulse Width"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:19 - Vertical Front Porch Size"]
    #[inline(always)]
    pub fn vfporch(&self) -> VFPORCH_R {
        VFPORCH_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - Vertical Back Porch Size"]
    #[inline(always)]
    pub fn vbporch(&self) -> VBPORCH_R {
        VBPORCH_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Vertical Synchronization Pulse Width"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VSYNC_W {
        VSYNC_W { w: self }
    }
    #[doc = "Bits 8:19 - Vertical Front Porch Size"]
    #[inline(always)]
    pub fn vfporch(&mut self) -> VFPORCH_W {
        VFPORCH_W { w: self }
    }
    #[doc = "Bits 20:31 - Vertical Back Porch Size"]
    #[inline(always)]
    pub fn vbporch(&mut self) -> VBPORCH_W {
        VBPORCH_W { w: self }
    }
}
