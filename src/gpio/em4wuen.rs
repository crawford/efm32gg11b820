#[doc = "Reader of register EM4WUEN"]
pub type R = crate::R<u32, super::EM4WUEN>;
#[doc = "Writer for register EM4WUEN"]
pub type W = crate::W<u32, super::EM4WUEN>;
#[doc = "Register EM4WUEN `reset()`'s with value 0"]
impl crate::ResetValue for super::EM4WUEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EM4WUEN`"]
pub type EM4WUEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EM4WUEN`"]
pub struct EM4WUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WUEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - EM4 Wake Up Enable"]
    #[inline(always)]
    pub fn em4wuen(&self) -> EM4WUEN_R {
        EM4WUEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - EM4 Wake Up Enable"]
    #[inline(always)]
    pub fn em4wuen(&mut self) -> EM4WUEN_W {
        EM4WUEN_W { w: self }
    }
}
