#[doc = "Reader of register MASKADD1TOP"]
pub type R = crate::R<u32, super::MASKADD1TOP>;
#[doc = "Writer for register MASKADD1TOP"]
pub type W = crate::W<u32, super::MASKADD1TOP>;
#[doc = "Register MASKADD1TOP `reset()`'s with value 0"]
impl crate::ResetValue for super::MASKADD1TOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDRMASK`"]
pub type ADDRMASK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDRMASK`"]
pub struct ADDRMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Specific Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> ADDRMASK_R {
        ADDRMASK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific Address Mask"]
    #[inline(always)]
    pub fn addrmask(&mut self) -> ADDRMASK_W {
        ADDRMASK_W { w: self }
    }
}
