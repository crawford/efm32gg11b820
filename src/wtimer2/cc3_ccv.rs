#[doc = "Reader of register CC3_CCV"]
pub type R = crate::R<u32, super::CC3_CCV>;
#[doc = "Writer for register CC3_CCV"]
pub type W = crate::W<u32, super::CC3_CCV>;
#[doc = "Register CC3_CCV `reset()`'s with value 0"]
impl crate::ResetValue for super::CC3_CCV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCV`"]
pub type CCV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CCV`"]
pub struct CCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CCV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&mut self) -> CCV_W {
        CCV_W { w: self }
    }
}
