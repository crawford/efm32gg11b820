#[doc = "Reader of register HFXOSTEADYSTATECTRL"]
pub type R = crate::R<u32, super::HFXOSTEADYSTATECTRL>;
#[doc = "Writer for register HFXOSTEADYSTATECTRL"]
pub type W = crate::W<u32, super::HFXOSTEADYSTATECTRL>;
#[doc = "Register HFXOSTEADYSTATECTRL `reset()`'s with value 0x0800_0100"]
impl crate::ResetValue for super::HFXOSTEADYSTATECTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800_0100
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
#[doc = "Reader of field `PEAKDETEN`"]
pub type PEAKDETEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEAKDETEN`"]
pub struct PEAKDETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAKDETEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `PEAKMONEN`"]
pub type PEAKMONEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEAKMONEN`"]
pub struct PEAKMONEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAKMONEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&self) -> CTUNE_R {
        CTUNE_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    pub fn peakdeten(&self) -> PEAKDETEN_R {
        PEAKDETEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
    #[inline(always)]
    pub fn peakmonen(&self) -> PEAKMONEN_R {
        PEAKMONEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Sets the Steady State Oscillator Core Bias Current."]
    #[inline(always)]
    pub fn ibtrimxocore(&mut self) -> IBTRIMXOCORE_W {
        IBTRIMXOCORE_W { w: self }
    }
    #[doc = "Bits 11:19 - Sets Oscillator Tuning Capacitance"]
    #[inline(always)]
    pub fn ctune(&mut self) -> CTUNE_W {
        CTUNE_W { w: self }
    }
    #[doc = "Bit 26 - Enables Oscillator Peak Detectors"]
    #[inline(always)]
    pub fn peakdeten(&mut self) -> PEAKDETEN_W {
        PEAKDETEN_W { w: self }
    }
    #[doc = "Bit 27 - Automatically Perform Peak Monitoring Algorithm on Every Rising Edge of ULFRCO"]
    #[inline(always)]
    pub fn peakmonen(&mut self) -> PEAKMONEN_W {
        PEAKMONEN_W { w: self }
    }
}
