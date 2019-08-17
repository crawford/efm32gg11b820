#[doc = "Reader of register SEQ4"]
pub type R = crate::R<u32, super::SEQ4>;
#[doc = "Writer for register SEQ4"]
pub type W = crate::W<u32, super::SEQ4>;
#[doc = "Register SEQ4 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQ4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INSTR16`"]
pub type INSTR16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR16`"]
pub struct INSTR16_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `INSTR17`"]
pub type INSTR17_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR17`"]
pub struct INSTR17_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `INSTR18`"]
pub type INSTR18_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR18`"]
pub struct INSTR18_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `INSTR19`"]
pub type INSTR19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR19`"]
pub struct INSTR19_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 16"]
    #[inline(always)]
    pub fn instr16(&self) -> INSTR16_R {
        INSTR16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 17"]
    #[inline(always)]
    pub fn instr17(&self) -> INSTR17_R {
        INSTR17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 18"]
    #[inline(always)]
    pub fn instr18(&self) -> INSTR18_R {
        INSTR18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 19"]
    #[inline(always)]
    pub fn instr19(&self) -> INSTR19_R {
        INSTR19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 16"]
    #[inline(always)]
    pub fn instr16(&mut self) -> INSTR16_W {
        INSTR16_W { w: self }
    }
    #[doc = "Bits 8:15 - Sequence Instruction 17"]
    #[inline(always)]
    pub fn instr17(&mut self) -> INSTR17_W {
        INSTR17_W { w: self }
    }
    #[doc = "Bits 16:23 - Sequence Instruction 18"]
    #[inline(always)]
    pub fn instr18(&mut self) -> INSTR18_W {
        INSTR18_W { w: self }
    }
    #[doc = "Bits 24:31 - Sequence Instruction 19"]
    #[inline(always)]
    pub fn instr19(&mut self) -> INSTR19_W {
        INSTR19_W { w: self }
    }
}
