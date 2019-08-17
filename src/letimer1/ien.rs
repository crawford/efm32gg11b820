#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMP0`"]
pub type COMP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP0`"]
pub struct COMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `COMP1`"]
pub type COMP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP1`"]
pub struct COMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `UF`"]
pub type UF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UF`"]
pub struct UF_W<'a> {
    w: &'a mut W,
}
impl<'a> UF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `REP0`"]
pub type REP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REP0`"]
pub struct REP0_W<'a> {
    w: &'a mut W,
}
impl<'a> REP0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `REP1`"]
pub type REP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REP1`"]
pub struct REP1_W<'a> {
    w: &'a mut W,
}
impl<'a> REP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - COMP0 Interrupt Enable"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - COMP1 Interrupt Enable"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - REP0 Interrupt Enable"]
    #[inline(always)]
    pub fn rep0(&self) -> REP0_R {
        REP0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - REP1 Interrupt Enable"]
    #[inline(always)]
    pub fn rep1(&self) -> REP1_R {
        REP1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP0 Interrupt Enable"]
    #[inline(always)]
    pub fn comp0(&mut self) -> COMP0_W {
        COMP0_W { w: self }
    }
    #[doc = "Bit 1 - COMP1 Interrupt Enable"]
    #[inline(always)]
    pub fn comp1(&mut self) -> COMP1_W {
        COMP1_W { w: self }
    }
    #[doc = "Bit 2 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W {
        UF_W { w: self }
    }
    #[doc = "Bit 3 - REP0 Interrupt Enable"]
    #[inline(always)]
    pub fn rep0(&mut self) -> REP0_W {
        REP0_W { w: self }
    }
    #[doc = "Bit 4 - REP1 Interrupt Enable"]
    #[inline(always)]
    pub fn rep1(&mut self) -> REP1_W {
        REP1_W { w: self }
    }
}
