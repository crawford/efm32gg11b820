#[doc = "Reader of register SEQ3"]
pub type R = crate::R<u32, super::SEQ3>;
#[doc = "Writer for register SEQ3"]
pub type W = crate::W<u32, super::SEQ3>;
#[doc = "Register SEQ3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQ3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INSTR12`"]
pub type INSTR12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR12`"]
pub struct INSTR12_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `INSTR13`"]
pub type INSTR13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR13`"]
pub struct INSTR13_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `INSTR14`"]
pub type INSTR14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR14`"]
pub struct INSTR14_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `INSTR15`"]
pub type INSTR15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR15`"]
pub struct INSTR15_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 12"]
    #[inline(always)]
    pub fn instr12(&self) -> INSTR12_R {
        INSTR12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 13"]
    #[inline(always)]
    pub fn instr13(&self) -> INSTR13_R {
        INSTR13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 14"]
    #[inline(always)]
    pub fn instr14(&self) -> INSTR14_R {
        INSTR14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 15"]
    #[inline(always)]
    pub fn instr15(&self) -> INSTR15_R {
        INSTR15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 12"]
    #[inline(always)]
    pub fn instr12(&mut self) -> INSTR12_W {
        INSTR12_W { w: self }
    }
    #[doc = "Bits 8:15 - Sequence Instruction 13"]
    #[inline(always)]
    pub fn instr13(&mut self) -> INSTR13_W {
        INSTR13_W { w: self }
    }
    #[doc = "Bits 16:23 - Sequence Instruction 14"]
    #[inline(always)]
    pub fn instr14(&mut self) -> INSTR14_W {
        INSTR14_W { w: self }
    }
    #[doc = "Bits 24:31 - Sequence Instruction 15"]
    #[inline(always)]
    pub fn instr15(&mut self) -> INSTR15_W {
        INSTR15_W { w: self }
    }
}
