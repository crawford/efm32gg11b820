#[doc = "Reader of register TSUSECCMP"]
pub type R = crate::R<u32, super::TSUSECCMP>;
#[doc = "Writer for register TSUSECCMP"]
pub type W = crate::W<u32, super::TSUSECCMP>;
#[doc = "Register TSUSECCMP `reset()`'s with value 0"]
impl crate::ResetValue for super::TSUSECCMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMPVAL`"]
pub type COMPVAL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COMPVAL`"]
pub struct COMPVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TSU timer comparison value (s)"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSU timer comparison value (s)"]
    #[inline(always)]
    pub fn compval(&mut self) -> COMPVAL_W {
        COMPVAL_W { w: self }
    }
}
