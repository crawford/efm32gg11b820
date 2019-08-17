#[doc = "Reader of register SEQ0"]
pub type R = crate::R<u32, super::SEQ0>;
#[doc = "Writer for register SEQ0"]
pub type W = crate::W<u32, super::SEQ0>;
#[doc = "Register SEQ0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQ0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INSTR0`"]
pub type INSTR0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR0`"]
pub struct INSTR0_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `INSTR1`"]
pub type INSTR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR1`"]
pub struct INSTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `INSTR2`"]
pub type INSTR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR2`"]
pub struct INSTR2_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `INSTR3`"]
pub type INSTR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR3`"]
pub struct INSTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 0"]
    #[inline(always)]
    pub fn instr0(&self) -> INSTR0_R {
        INSTR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 1"]
    #[inline(always)]
    pub fn instr1(&self) -> INSTR1_R {
        INSTR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 2"]
    #[inline(always)]
    pub fn instr2(&self) -> INSTR2_R {
        INSTR2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 3"]
    #[inline(always)]
    pub fn instr3(&self) -> INSTR3_R {
        INSTR3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 0"]
    #[inline(always)]
    pub fn instr0(&mut self) -> INSTR0_W {
        INSTR0_W { w: self }
    }
    #[doc = "Bits 8:15 - Sequence Instruction 1"]
    #[inline(always)]
    pub fn instr1(&mut self) -> INSTR1_W {
        INSTR1_W { w: self }
    }
    #[doc = "Bits 16:23 - Sequence Instruction 2"]
    #[inline(always)]
    pub fn instr2(&mut self) -> INSTR2_W {
        INSTR2_W { w: self }
    }
    #[doc = "Bits 24:31 - Sequence Instruction 3"]
    #[inline(always)]
    pub fn instr3(&mut self) -> INSTR3_W {
        INSTR3_W { w: self }
    }
}
