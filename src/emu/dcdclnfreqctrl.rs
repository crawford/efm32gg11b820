#[doc = "Reader of register DCDCLNFREQCTRL"]
pub type R = crate::R<u32, super::DCDCLNFREQCTRL>;
#[doc = "Writer for register DCDCLNFREQCTRL"]
pub type W = crate::W<u32, super::DCDCLNFREQCTRL>;
#[doc = "Register DCDCLNFREQCTRL `reset()`'s with value 0x1000_0000"]
impl crate::ResetValue for super::DCDCLNFREQCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000_0000
    }
}
#[doc = "Reader of field `RCOBAND`"]
pub type RCOBAND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOBAND`"]
pub struct RCOBAND_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOBAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `RCOTRIM`"]
pub type RCOTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOTRIM`"]
pub struct RCOTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - LN Mode RCO Frequency Band Selection"]
    #[inline(always)]
    pub fn rcoband(&self) -> RCOBAND_R {
        RCOBAND_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 24:28 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn rcotrim(&self) -> RCOTRIM_R {
        RCOTRIM_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LN Mode RCO Frequency Band Selection"]
    #[inline(always)]
    pub fn rcoband(&mut self) -> RCOBAND_W {
        RCOBAND_W { w: self }
    }
    #[doc = "Bits 24:28 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn rcotrim(&mut self) -> RCOTRIM_W {
        RCOTRIM_W { w: self }
    }
}
