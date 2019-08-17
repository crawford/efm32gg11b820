#[doc = "Reader of register INITWAITVAL"]
pub type R = crate::R<u32, super::INITWAITVAL>;
#[doc = "Writer for register INITWAITVAL"]
pub type W = crate::W<u32, super::INITWAITVAL>;
#[doc = "Register INITWAITVAL `reset()`'s with value 0xff"]
impl crate::ResetValue for super::INITWAITVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Wait counter value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait counter value"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
