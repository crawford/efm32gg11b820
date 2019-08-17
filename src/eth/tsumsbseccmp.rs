#[doc = "Reader of register TSUMSBSECCMP"]
pub type R = crate::R<u32, super::TSUMSBSECCMP>;
#[doc = "Writer for register TSUMSBSECCMP"]
pub type W = crate::W<u32, super::TSUMSBSECCMP>;
#[doc = "Register TSUMSBSECCMP `reset()`'s with value 0"]
impl crate::ResetValue for super::TSUMSBSECCMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPVAL`"]
pub type COMPVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMPVAL`"]
pub struct COMPVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TSU timer comparison value (s)"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TSU timer comparison value (s)"]
    #[inline(always)]
    pub fn compval(&mut self) -> COMPVAL_W {
        COMPVAL_W { w: self }
    }
}
