#[doc = "Reader of register TFTPIXEL1"]
pub type R = crate::R<u32, super::TFTPIXEL1>;
#[doc = "Writer for register TFTPIXEL1"]
pub type W = crate::W<u32, super::TFTPIXEL1>;
#[doc = "Register TFTPIXEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TFTPIXEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - RGB Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - RGB Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
