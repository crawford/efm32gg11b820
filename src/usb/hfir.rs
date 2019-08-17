#[doc = "Reader of register HFIR"]
pub type R = crate::R<u32, super::HFIR>;
#[doc = "Writer for register HFIR"]
pub type W = crate::W<u32, super::HFIR>;
#[doc = "Register HFIR `reset()`'s with value 0xea60"]
impl crate::ResetValue for super::HFIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xea60
    }
}
#[doc = "Reader of field `FRINT`"]
pub type FRINT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRINT`"]
pub struct FRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `HFIRRLDCTRL`"]
pub type HFIRRLDCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFIRRLDCTRL`"]
pub struct HFIRRLDCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> HFIRRLDCTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn frint(&self) -> FRINT_R {
        FRINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrldctrl(&self) -> HFIRRLDCTRL_R {
        HFIRRLDCTRL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frame Interval"]
    #[inline(always)]
    pub fn frint(&mut self) -> FRINT_W {
        FRINT_W { w: self }
    }
    #[doc = "Bit 16 - Reload Control"]
    #[inline(always)]
    pub fn hfirrldctrl(&mut self) -> HFIRRLDCTRL_W {
        HFIRRLDCTRL_W { w: self }
    }
}
