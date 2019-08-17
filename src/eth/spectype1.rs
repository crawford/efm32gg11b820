#[doc = "Reader of register SPECTYPE1"]
pub type R = crate::R<u32, super::SPECTYPE1>;
#[doc = "Writer for register SPECTYPE1"]
pub type W = crate::W<u32, super::SPECTYPE1>;
#[doc = "Register SPECTYPE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPECTYPE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MATCH`"]
pub type MATCH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MATCH`"]
pub struct MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ENBCOPY`"]
pub type ENBCOPY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBCOPY`"]
pub struct ENBCOPY_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBCOPY_W<'a> {
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
    #[doc = "Bits 0:15 - Type ID match 1"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 1 matched frames."]
    #[inline(always)]
    pub fn enbcopy(&self) -> ENBCOPY_R {
        ENBCOPY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID match 1"]
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W {
        MATCH_W { w: self }
    }
    #[doc = "Bit 31 - Enable copying of type ID match 1 matched frames."]
    #[inline(always)]
    pub fn enbcopy(&mut self) -> ENBCOPY_W {
        ENBCOPY_W { w: self }
    }
}
