#[doc = "Reader of register MASKADD1BOTTOM"]
pub type R = crate::R<u32, super::MASKADD1BOTTOM>;
#[doc = "Writer for register MASKADD1BOTTOM"]
pub type W = crate::W<u32, super::MASKADD1BOTTOM>;
#[doc = "Register MASKADD1BOTTOM `reset()`'s with value 0"]
impl crate::ResetValue for super::MASKADD1BOTTOM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRMASK`"]
pub type ADDRMASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDRMASK`"]
pub struct ADDRMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Specific Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> ADDRMASK_R {
        ADDRMASK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specific Address Mask"]
    #[inline(always)]
    pub fn addrmask(&mut self) -> ADDRMASK_W {
        ADDRMASK_W { w: self }
    }
}
