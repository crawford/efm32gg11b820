#[doc = "Reader of register BUF13_DATA"]
pub type R = crate::R<u32, super::BUF13_DATA>;
#[doc = "Writer for register BUF13_DATA"]
pub type W = crate::W<u32, super::BUF13_DATA>;
#[doc = "Register BUF13_DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::BUF13_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DATASRC`"]
pub type DATASRC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Scan Result Buffer"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Result Data Source"]
    #[inline(always)]
    pub fn datasrc(&self) -> DATASRC_R {
        DATASRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan Result Buffer"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
