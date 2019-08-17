#[doc = "Reader of register DATA0"]
pub type R = crate::R<u32, super::DATA0>;
#[doc = "Writer for register DATA0"]
pub type W = crate::W<u32, super::DATA0>;
#[doc = "Register DATA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA0`"]
pub type DATA0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATA0`"]
pub struct DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data 0 Access"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data 0 Access"]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W { w: self }
    }
}
