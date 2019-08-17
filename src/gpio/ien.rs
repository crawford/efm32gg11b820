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
#[doc = "Reader of field `EXT`"]
pub type EXT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EXT`"]
pub struct EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `EM4WU`"]
pub type EM4WU_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EM4WU`"]
pub struct EM4WU_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - EXT Interrupt Enable"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - EM4WU Interrupt Enable"]
    #[inline(always)]
    pub fn em4wu(&self) -> EM4WU_R {
        EM4WU_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - EXT Interrupt Enable"]
    #[inline(always)]
    pub fn ext(&mut self) -> EXT_W {
        EXT_W { w: self }
    }
    #[doc = "Bits 16:31 - EM4WU Interrupt Enable"]
    #[inline(always)]
    pub fn em4wu(&mut self) -> EM4WU_W {
        EM4WU_W { w: self }
    }
}
