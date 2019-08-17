#[doc = "Reader of register NOOFPOLLSBEFEXP"]
pub type R = crate::R<u32, super::NOOFPOLLSBEFEXP>;
#[doc = "Writer for register NOOFPOLLSBEFEXP"]
pub type W = crate::W<u32, super::NOOFPOLLSBEFEXP>;
#[doc = "Register NOOFPOLLSBEFEXP `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::NOOFPOLLSBEFEXP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `NOOFPOLLSBEFEXP`"]
pub type NOOFPOLLSBEFEXP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NOOFPOLLSBEFEXP`"]
pub struct NOOFPOLLSBEFEXP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOOFPOLLSBEFEXP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of Polls Cycles Before Expiration"]
    #[inline(always)]
    pub fn noofpollsbefexp(&self) -> NOOFPOLLSBEFEXP_R {
        NOOFPOLLSBEFEXP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of Polls Cycles Before Expiration"]
    #[inline(always)]
    pub fn noofpollsbefexp(&mut self) -> NOOFPOLLSBEFEXP_W {
        NOOFPOLLSBEFEXP_W { w: self }
    }
}
