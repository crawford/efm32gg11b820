#[doc = "Reader of register CMDARG1"]
pub type R = crate::R<u32, super::CMDARG1>;
#[doc = "Writer for register CMDARG1"]
pub type W = crate::W<u32, super::CMDARG1>;
#[doc = "Register CMDARG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMDARG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMDARG1`"]
pub type CMDARG1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CMDARG1`"]
pub struct CMDARG1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDARG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Command Argument 1"]
    #[inline(always)]
    pub fn cmdarg1(&self) -> CMDARG1_R {
        CMDARG1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument 1"]
    #[inline(always)]
    pub fn cmdarg1(&mut self) -> CMDARG1_W {
        CMDARG1_W { w: self }
    }
}
