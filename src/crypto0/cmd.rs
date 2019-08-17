#[doc = "Reader of register CMD"]
pub type R = crate::R<u32, super::CMD>;
#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INSTR`"]
pub type INSTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTR`"]
pub struct INSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Write proxy for field `SEQSTART`"]
pub struct SEQSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQSTART_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `SEQSTOP`"]
pub struct SEQSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQSTOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `SEQSTEP`"]
pub struct SEQSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQSTEP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Execute Instruction"]
    #[inline(always)]
    pub fn instr(&self) -> INSTR_R {
        INSTR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Execute Instruction"]
    #[inline(always)]
    pub fn instr(&mut self) -> INSTR_W {
        INSTR_W { w: self }
    }
    #[doc = "Bit 9 - Encryption/Decryption SEQUENCE Start"]
    #[inline(always)]
    pub fn seqstart(&mut self) -> SEQSTART_W {
        SEQSTART_W { w: self }
    }
    #[doc = "Bit 10 - Sequence Stop"]
    #[inline(always)]
    pub fn seqstop(&mut self) -> SEQSTOP_W {
        SEQSTOP_W { w: self }
    }
    #[doc = "Bit 11 - Sequence Step"]
    #[inline(always)]
    pub fn seqstep(&mut self) -> SEQSTEP_W {
        SEQSTEP_W { w: self }
    }
}
