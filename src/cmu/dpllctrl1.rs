#[doc = "Reader of register DPLLCTRL1"]
pub type R = crate::R<u32, super::DPLLCTRL1>;
#[doc = "Writer for register DPLLCTRL1"]
pub type W = crate::W<u32, super::DPLLCTRL1>;
#[doc = "Register DPLLCTRL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DPLLCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `M`"]
pub type M_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `M`"]
pub struct M_W<'a> {
    w: &'a mut W,
}
impl<'a> M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `N`"]
pub type N_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `N`"]
pub struct N_W<'a> {
    w: &'a mut W,
}
impl<'a> N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Factor M"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Factor N"]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Factor M"]
    #[inline(always)]
    pub fn m(&mut self) -> M_W {
        M_W { w: self }
    }
    #[doc = "Bits 16:27 - Factor N"]
    #[inline(always)]
    pub fn n(&mut self) -> N_W {
        N_W { w: self }
    }
}
