#[doc = "Reader of register TFTHPORCH"]
pub type R = crate::R<u32, super::TFTHPORCH>;
#[doc = "Writer for register TFTHPORCH"]
pub type W = crate::W<u32, super::TFTHPORCH>;
#[doc = "Register TFTHPORCH `reset()`'s with value 0"]
impl crate::ResetValue for super::TFTHPORCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSYNC`"]
pub type HSYNC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSYNC`"]
pub struct HSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `HFPORCH`"]
pub type HFPORCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HFPORCH`"]
pub struct HFPORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> HFPORCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HBPORCH`"]
pub type HBPORCH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HBPORCH`"]
pub struct HBPORCH_W<'a> {
    w: &'a mut W,
}
impl<'a> HBPORCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 18)) | (((value as u32) & 0xff) << 18);
        self.w
    }
}
#[doc = "Reader of field `HSYNCSTART`"]
pub type HSYNCSTART_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSYNCSTART`"]
pub struct HSYNCSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNCSTART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Horizontal Synchronization Pulse Width"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal Front Porch Size"]
    #[inline(always)]
    pub fn hfporch(&self) -> HFPORCH_R {
        HFPORCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Horizontal Back Porch Size"]
    #[inline(always)]
    pub fn hbporch(&self) -> HBPORCH_R {
        HBPORCH_R::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bits 28:29 - HSYNC Start Delay"]
    #[inline(always)]
    pub fn hsyncstart(&self) -> HSYNCSTART_R {
        HSYNCSTART_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Horizontal Synchronization Pulse Width"]
    #[inline(always)]
    pub fn hsync(&mut self) -> HSYNC_W {
        HSYNC_W { w: self }
    }
    #[doc = "Bits 8:15 - Horizontal Front Porch Size"]
    #[inline(always)]
    pub fn hfporch(&mut self) -> HFPORCH_W {
        HFPORCH_W { w: self }
    }
    #[doc = "Bits 18:25 - Horizontal Back Porch Size"]
    #[inline(always)]
    pub fn hbporch(&mut self) -> HBPORCH_W {
        HBPORCH_W { w: self }
    }
    #[doc = "Bits 28:29 - HSYNC Start Delay"]
    #[inline(always)]
    pub fn hsyncstart(&mut self) -> HSYNCSTART_W {
        HSYNCSTART_W { w: self }
    }
}
