#[doc = "Reader of register SEQ2"]
pub type R = crate::R<u32, super::SEQ2>;
#[doc = "Writer for register SEQ2"]
pub type W = crate::W<u32, super::SEQ2>;
#[doc = "Register SEQ2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQ2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INSTR8`"]
pub type INSTR8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR8`"]
pub struct INSTR8_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `INSTR9`"]
pub type INSTR9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR9`"]
pub struct INSTR9_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `INSTR10`"]
pub type INSTR10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR10`"]
pub struct INSTR10_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `INSTR11`"]
pub type INSTR11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR11`"]
pub struct INSTR11_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 8"]
    #[inline(always)]
    pub fn instr8(&self) -> INSTR8_R {
        INSTR8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 9"]
    #[inline(always)]
    pub fn instr9(&self) -> INSTR9_R {
        INSTR9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 10"]
    #[inline(always)]
    pub fn instr10(&self) -> INSTR10_R {
        INSTR10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 11"]
    #[inline(always)]
    pub fn instr11(&self) -> INSTR11_R {
        INSTR11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 8"]
    #[inline(always)]
    pub fn instr8(&mut self) -> INSTR8_W {
        INSTR8_W { w: self }
    }
    #[doc = "Bits 8:15 - Sequence Instruction 9"]
    #[inline(always)]
    pub fn instr9(&mut self) -> INSTR9_W {
        INSTR9_W { w: self }
    }
    #[doc = "Bits 16:23 - Sequence Instruction 10"]
    #[inline(always)]
    pub fn instr10(&mut self) -> INSTR10_W {
        INSTR10_W { w: self }
    }
    #[doc = "Bits 24:31 - Sequence Instruction 11"]
    #[inline(always)]
    pub fn instr11(&mut self) -> INSTR11_W {
        INSTR11_W { w: self }
    }
}
