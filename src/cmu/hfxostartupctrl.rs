#[doc = "Reader of register HFXOSTARTUPCTRL"]
pub type R = crate::R<u32, super::HFXOSTARTUPCTRL>;
#[doc = "Writer for register HFXOSTARTUPCTRL"]
pub type W = crate::W<u32, super::HFXOSTARTUPCTRL>;
#[doc = "Register HFXOSTARTUPCTRL `reset()`'s with value 0x0600"]
impl crate::ResetValue for super::HFXOSTARTUPCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0600
    }
}
#[doc = "Reader of field `IBTRIMXOCORE`"]
pub type IBTRIMXOCORE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IBTRIMXOCORE`"]
pub struct IBTRIMXOCORE_W<'a> {
    w: &'a mut W,
}
impl<'a> IBTRIMXOCORE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `CTUNE`"]
pub type CTUNE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTUNE`"]
pub struct CTUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTUNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 11)) | (((value as u32) & 0x01ff) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Sets the Startup Oscillator Core Bias Current"]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&self) -> CTUNE_R {
        CTUNE_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Sets the Startup Oscillator Core Bias Current"]
    #[inline(always)]
    pub fn ibtrimxocore(&mut self) -> IBTRIMXOCORE_W {
        IBTRIMXOCORE_W { w: self }
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&mut self) -> CTUNE_W {
        CTUNE_W { w: self }
    }
}
