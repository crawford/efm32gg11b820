#[doc = "Reader of register SEQ1"]
pub type R = crate::R<u32, super::SEQ1>;
#[doc = "Writer for register SEQ1"]
pub type W = crate::W<u32, super::SEQ1>;
#[doc = "Register SEQ1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQ1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INSTR4`"]
pub type INSTR4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR4`"]
pub struct INSTR4_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `INSTR5`"]
pub type INSTR5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR5`"]
pub struct INSTR5_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `INSTR6`"]
pub type INSTR6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR6`"]
pub struct INSTR6_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `INSTR7`"]
pub type INSTR7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR7`"]
pub struct INSTR7_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 4"]
    #[inline(always)]
    pub fn instr4(&self) -> INSTR4_R {
        INSTR4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 5"]
    #[inline(always)]
    pub fn instr5(&self) -> INSTR5_R {
        INSTR5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 6"]
    #[inline(always)]
    pub fn instr6(&self) -> INSTR6_R {
        INSTR6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 7"]
    #[inline(always)]
    pub fn instr7(&self) -> INSTR7_R {
        INSTR7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 4"]
    #[inline(always)]
    pub fn instr4(&mut self) -> INSTR4_W {
        INSTR4_W { w: self }
    }
    #[doc = "Bits 8:15 - Sequence Instruction 5"]
    #[inline(always)]
    pub fn instr5(&mut self) -> INSTR5_W {
        INSTR5_W { w: self }
    }
    #[doc = "Bits 16:23 - Sequence Instruction 6"]
    #[inline(always)]
    pub fn instr6(&mut self) -> INSTR6_W {
        INSTR6_W { w: self }
    }
    #[doc = "Bits 24:31 - Sequence Instruction 7"]
    #[inline(always)]
    pub fn instr7(&mut self) -> INSTR7_W {
        INSTR7_W { w: self }
    }
}
