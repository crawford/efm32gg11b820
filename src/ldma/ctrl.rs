#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x1700_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1700_0000
    }
}
#[doc = "Reader of field `SYNCPRSSETEN`"]
pub type SYNCPRSSETEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNCPRSSETEN`"]
pub struct SYNCPRSSETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCPRSSETEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SYNCPRSCLREN`"]
pub type SYNCPRSCLREN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNCPRSCLREN`"]
pub struct SYNCPRSCLREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCPRSCLREN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `NUMFIXED`"]
pub type NUMFIXED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NUMFIXED`"]
pub struct NUMFIXED_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMFIXED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Synchronization PRS Set Enable"]
    #[inline(always)]
    pub fn syncprsseten(&self) -> SYNCPRSSETEN_R {
        SYNCPRSSETEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Synchronization PRS Clear Enable"]
    #[inline(always)]
    pub fn syncprsclren(&self) -> SYNCPRSCLREN_R {
        SYNCPRSCLREN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Number of Fixed Priority Channels"]
    #[inline(always)]
    pub fn numfixed(&self) -> NUMFIXED_R {
        NUMFIXED_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronization PRS Set Enable"]
    #[inline(always)]
    pub fn syncprsseten(&mut self) -> SYNCPRSSETEN_W {
        SYNCPRSSETEN_W { w: self }
    }
    #[doc = "Bits 8:15 - Synchronization PRS Clear Enable"]
    #[inline(always)]
    pub fn syncprsclren(&mut self) -> SYNCPRSCLREN_W {
        SYNCPRSCLREN_W { w: self }
    }
    #[doc = "Bits 24:28 - Number of Fixed Priority Channels"]
    #[inline(always)]
    pub fn numfixed(&mut self) -> NUMFIXED_W {
        NUMFIXED_W { w: self }
    }
}
