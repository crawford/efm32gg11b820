#[doc = "Reader of register OPA1_TIMER"]
pub type R = crate::R<u32, super::OPA1_TIMER>;
#[doc = "Writer for register OPA1_TIMER"]
pub type W = crate::W<u32, super::OPA1_TIMER>;
#[doc = "Register OPA1_TIMER `reset()`'s with value 0x0001_0700"]
impl crate::ResetValue for super::OPA1_TIMER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0700
    }
}
#[doc = "Reader of field `STARTUPDLY`"]
pub type STARTUPDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STARTUPDLY`"]
pub struct STARTUPDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUPDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `WARMUPTIME`"]
pub type WARMUPTIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WARMUPTIME`"]
pub struct WARMUPTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> WARMUPTIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SETTLETIME`"]
pub type SETTLETIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SETTLETIME`"]
pub struct SETTLETIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SETTLETIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - OPAx Startup Delay Count Value"]
    #[inline(always)]
    pub fn startupdly(&self) -> STARTUPDLY_R {
        STARTUPDLY_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - OPAx Warmup Time Count Value"]
    #[inline(always)]
    pub fn warmuptime(&self) -> WARMUPTIME_R {
        WARMUPTIME_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:25 - OPAx Output Settling Timeout Value"]
    #[inline(always)]
    pub fn settletime(&self) -> SETTLETIME_R {
        SETTLETIME_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - OPAx Startup Delay Count Value"]
    #[inline(always)]
    pub fn startupdly(&mut self) -> STARTUPDLY_W {
        STARTUPDLY_W { w: self }
    }
    #[doc = "Bits 8:14 - OPAx Warmup Time Count Value"]
    #[inline(always)]
    pub fn warmuptime(&mut self) -> WARMUPTIME_W {
        WARMUPTIME_W { w: self }
    }
    #[doc = "Bits 16:25 - OPAx Output Settling Timeout Value"]
    #[inline(always)]
    pub fn settletime(&mut self) -> SETTLETIME_W {
        SETTLETIME_W { w: self }
    }
}
