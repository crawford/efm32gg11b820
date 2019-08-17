#[doc = "Reader of register TSUTIMERADJUST"]
pub type R = crate::R<u32, super::TSUTIMERADJUST>;
#[doc = "Writer for register TSUTIMERADJUST"]
pub type W = crate::W<u32, super::TSUTIMERADJUST>;
#[doc = "Register TSUTIMERADJUST `reset()`'s with value 0"]
impl crate::ResetValue for super::TSUTIMERADJUST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INCREMENTVAL`"]
pub type INCREMENTVAL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INCREMENTVAL`"]
pub struct INCREMENTVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INCREMENTVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `ADDSUBTRACT`"]
pub type ADDSUBTRACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADDSUBTRACT`"]
pub struct ADDSUBTRACT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDSUBTRACT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Timer increment value"]
    #[inline(always)]
    pub fn incrementval(&self) -> INCREMENTVAL_R {
        INCREMENTVAL_R::new((self.bits & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Write as one to subtract from the 1588 timer"]
    #[inline(always)]
    pub fn addsubtract(&self) -> ADDSUBTRACT_R {
        ADDSUBTRACT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer increment value"]
    #[inline(always)]
    pub fn incrementval(&mut self) -> INCREMENTVAL_W {
        INCREMENTVAL_W { w: self }
    }
    #[doc = "Bit 31 - Write as one to subtract from the 1588 timer"]
    #[inline(always)]
    pub fn addsubtract(&mut self) -> ADDSUBTRACT_W {
        ADDSUBTRACT_W { w: self }
    }
}
