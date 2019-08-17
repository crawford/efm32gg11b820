#[doc = "Reader of register MIR0_ARB"]
pub type R = crate::R<u32, super::MIR0_ARB>;
#[doc = "Writer for register MIR0_ARB"]
pub type W = crate::W<u32, super::MIR0_ARB>;
#[doc = "Register MIR0_ARB `reset()`'s with value 0"]
impl crate::ResetValue for super::MIR0_ARB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ID`"]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `XTD`"]
pub type XTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTD`"]
pub struct XTD_W<'a> {
    w: &'a mut W,
}
impl<'a> XTD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `MSGVAL`"]
pub type MSGVAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSGVAL`"]
pub struct MSGVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MSGVAL_W<'a> {
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
    #[doc = "Bits 0:28 - Message Identifier"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 29 - Message Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&self) -> XTD_R {
        XTD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Message Valid"]
    #[inline(always)]
    pub fn msgval(&self) -> MSGVAL_R {
        MSGVAL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Message Identifier"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Bit 29 - Message Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&mut self) -> XTD_W {
        XTD_W { w: self }
    }
    #[doc = "Bit 31 - Message Valid"]
    #[inline(always)]
    pub fn msgval(&mut self) -> MSGVAL_W {
        MSGVAL_W { w: self }
    }
}
