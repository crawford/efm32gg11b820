#[doc = "Reader of register DMBASELINE"]
pub type R = crate::R<u32, super::DMBASELINE>;
#[doc = "Writer for register DMBASELINE"]
pub type W = crate::W<u32, super::DMBASELINE>;
#[doc = "Register DMBASELINE `reset()`'s with value 0"]
impl crate::ResetValue for super::DMBASELINE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BASELINEUP`"]
pub type BASELINEUP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BASELINEUP`"]
pub struct BASELINEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> BASELINEUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `BASELINEDN`"]
pub type BASELINEDN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BASELINEDN`"]
pub struct BASELINEDN_W<'a> {
    w: &'a mut W,
}
impl<'a> BASELINEDN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    pub fn baselineup(&self) -> BASELINEUP_R {
        BASELINEUP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    pub fn baselinedn(&self) -> BASELINEDN_R {
        BASELINEDN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    pub fn baselineup(&mut self) -> BASELINEUP_W {
        BASELINEUP_W { w: self }
    }
    #[doc = "Bits 16:31 - Delta Modulator Integrator Initial Value"]
    #[inline(always)]
    pub fn baselinedn(&mut self) -> BASELINEDN_W {
        BASELINEDN_W { w: self }
    }
}
