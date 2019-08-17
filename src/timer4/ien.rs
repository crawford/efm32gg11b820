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
#[doc = "Reader of field `OF`"]
pub type OF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OF`"]
pub struct OF_W<'a> {
    w: &'a mut W,
}
impl<'a> OF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DIRCHG`"]
pub type DIRCHG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRCHG`"]
pub struct DIRCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCHG_W<'a> {
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
#[doc = "Reader of field `CC0`"]
pub type CC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC0`"]
pub struct CC0_W<'a> {
    w: &'a mut W,
}
impl<'a> CC0_W<'a> {
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
#[doc = "Reader of field `CC1`"]
pub type CC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC1`"]
pub struct CC1_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CC2`"]
pub type CC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC2`"]
pub struct CC2_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CC3`"]
pub type CC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC3`"]
pub struct CC3_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ICBOF0`"]
pub type ICBOF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICBOF0`"]
pub struct ICBOF0_W<'a> {
    w: &'a mut W,
}
impl<'a> ICBOF0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ICBOF1`"]
pub type ICBOF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICBOF1`"]
pub struct ICBOF1_W<'a> {
    w: &'a mut W,
}
impl<'a> ICBOF1_W<'a> {
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
#[doc = "Reader of field `ICBOF2`"]
pub type ICBOF2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICBOF2`"]
pub struct ICBOF2_W<'a> {
    w: &'a mut W,
}
impl<'a> ICBOF2_W<'a> {
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
#[doc = "Reader of field `ICBOF3`"]
pub type ICBOF3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICBOF3`"]
pub struct ICBOF3_W<'a> {
    w: &'a mut W,
}
impl<'a> ICBOF3_W<'a> {
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
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DIRCHG Interrupt Enable"]
    #[inline(always)]
    pub fn dirchg(&self) -> DIRCHG_R {
        DIRCHG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CC3 Interrupt Enable"]
    #[inline(always)]
    pub fn cc3(&self) -> CC3_R {
        CC3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ICBOF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof0(&self) -> ICBOF0_R {
        ICBOF0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ICBOF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof1(&self) -> ICBOF1_R {
        ICBOF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ICBOF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof2(&self) -> ICBOF2_R {
        ICBOF2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ICBOF3 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof3(&self) -> ICBOF3_R {
        ICBOF3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W { w: self }
    }
    #[doc = "Bit 1 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W {
        UF_W { w: self }
    }
    #[doc = "Bit 2 - DIRCHG Interrupt Enable"]
    #[inline(always)]
    pub fn dirchg(&mut self) -> DIRCHG_W {
        DIRCHG_W { w: self }
    }
    #[doc = "Bit 4 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&mut self) -> CC0_W {
        CC0_W { w: self }
    }
    #[doc = "Bit 5 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&mut self) -> CC1_W {
        CC1_W { w: self }
    }
    #[doc = "Bit 6 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&mut self) -> CC2_W {
        CC2_W { w: self }
    }
    #[doc = "Bit 7 - CC3 Interrupt Enable"]
    #[inline(always)]
    pub fn cc3(&mut self) -> CC3_W {
        CC3_W { w: self }
    }
    #[doc = "Bit 8 - ICBOF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof0(&mut self) -> ICBOF0_W {
        ICBOF0_W { w: self }
    }
    #[doc = "Bit 9 - ICBOF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof1(&mut self) -> ICBOF1_W {
        ICBOF1_W { w: self }
    }
    #[doc = "Bit 10 - ICBOF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof2(&mut self) -> ICBOF2_W {
        ICBOF2_W { w: self }
    }
    #[doc = "Bit 11 - ICBOF3 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof3(&mut self) -> ICBOF3_W {
        ICBOF3_W { w: self }
    }
}
