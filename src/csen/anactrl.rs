#[doc = "Reader of register ANACTRL"]
pub type R = crate::R<u32, super::ANACTRL>;
#[doc = "Writer for register ANACTRL"]
pub type W = crate::W<u32, super::ANACTRL>;
#[doc = "Register ANACTRL `reset()`'s with value 0x70"]
impl crate::ResetValue for super::ANACTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x70
    }
}
#[doc = "Reader of field `IREFPROG`"]
pub type IREFPROG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IREFPROG`"]
pub struct IREFPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> IREFPROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `IDACIREFS`"]
pub type IDACIREFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDACIREFS`"]
pub struct IDACIREFS_W<'a> {
    w: &'a mut W,
}
impl<'a> IDACIREFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `TRSTPROG`"]
pub type TRSTPROG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRSTPROG`"]
pub struct TRSTPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> TRSTPROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Reference Current Control."]
    #[inline(always)]
    pub fn irefprog(&self) -> IREFPROG_R {
        IREFPROG_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Current DAC and Reference Current Scale"]
    #[inline(always)]
    pub fn idacirefs(&self) -> IDACIREFS_R {
        IDACIREFS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Reset Timing"]
    #[inline(always)]
    pub fn trstprog(&self) -> TRSTPROG_R {
        TRSTPROG_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Reference Current Control."]
    #[inline(always)]
    pub fn irefprog(&mut self) -> IREFPROG_W {
        IREFPROG_W { w: self }
    }
    #[doc = "Bits 8:10 - Current DAC and Reference Current Scale"]
    #[inline(always)]
    pub fn idacirefs(&mut self) -> IDACIREFS_W {
        IDACIREFS_W { w: self }
    }
    #[doc = "Bits 20:22 - Reset Timing"]
    #[inline(always)]
    pub fn trstprog(&mut self) -> TRSTPROG_W {
        TRSTPROG_W { w: self }
    }
}
