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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `OSCFAIL`"]
pub type OSCFAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSCFAIL`"]
pub struct OSCFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCFAIL_W<'a> {
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
#[doc = "Reader of field `CNTTICK`"]
pub type CNTTICK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNTTICK`"]
pub struct CNTTICK_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTTICK_W<'a> {
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
#[doc = "Reader of field `MINTICK`"]
pub type MINTICK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MINTICK`"]
pub struct MINTICK_W<'a> {
    w: &'a mut W,
}
impl<'a> MINTICK_W<'a> {
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
#[doc = "Reader of field `HOURTICK`"]
pub type HOURTICK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOURTICK`"]
pub struct HOURTICK_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURTICK_W<'a> {
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
#[doc = "Reader of field `DAYTICK`"]
pub type DAYTICK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAYTICK`"]
pub struct DAYTICK_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYTICK_W<'a> {
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
#[doc = "Reader of field `DAYOWOF`"]
pub type DAYOWOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DAYOWOF`"]
pub struct DAYOWOF_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYOWOF_W<'a> {
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
#[doc = "Reader of field `MONTHTICK`"]
pub type MONTHTICK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONTHTICK`"]
pub struct MONTHTICK_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTHTICK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OSCFAIL Interrupt Enable"]
    #[inline(always)]
    pub fn oscfail(&self) -> OSCFAIL_R {
        OSCFAIL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CNTTICK Interrupt Enable"]
    #[inline(always)]
    pub fn cnttick(&self) -> CNTTICK_R {
        CNTTICK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MINTICK Interrupt Enable"]
    #[inline(always)]
    pub fn mintick(&self) -> MINTICK_R {
        MINTICK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HOURTICK Interrupt Enable"]
    #[inline(always)]
    pub fn hourtick(&self) -> HOURTICK_R {
        HOURTICK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DAYTICK Interrupt Enable"]
    #[inline(always)]
    pub fn daytick(&self) -> DAYTICK_R {
        DAYTICK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DAYOWOF Interrupt Enable"]
    #[inline(always)]
    pub fn dayowof(&self) -> DAYOWOF_R {
        DAYOWOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MONTHTICK Interrupt Enable"]
    #[inline(always)]
    pub fn monthtick(&self) -> MONTHTICK_R {
        MONTHTICK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W { w: self }
    }
    #[doc = "Bit 1 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&mut self) -> CC0_W {
        CC0_W { w: self }
    }
    #[doc = "Bit 2 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&mut self) -> CC1_W {
        CC1_W { w: self }
    }
    #[doc = "Bit 3 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&mut self) -> CC2_W {
        CC2_W { w: self }
    }
    #[doc = "Bit 4 - OSCFAIL Interrupt Enable"]
    #[inline(always)]
    pub fn oscfail(&mut self) -> OSCFAIL_W {
        OSCFAIL_W { w: self }
    }
    #[doc = "Bit 5 - CNTTICK Interrupt Enable"]
    #[inline(always)]
    pub fn cnttick(&mut self) -> CNTTICK_W {
        CNTTICK_W { w: self }
    }
    #[doc = "Bit 6 - MINTICK Interrupt Enable"]
    #[inline(always)]
    pub fn mintick(&mut self) -> MINTICK_W {
        MINTICK_W { w: self }
    }
    #[doc = "Bit 7 - HOURTICK Interrupt Enable"]
    #[inline(always)]
    pub fn hourtick(&mut self) -> HOURTICK_W {
        HOURTICK_W { w: self }
    }
    #[doc = "Bit 8 - DAYTICK Interrupt Enable"]
    #[inline(always)]
    pub fn daytick(&mut self) -> DAYTICK_W {
        DAYTICK_W { w: self }
    }
    #[doc = "Bit 9 - DAYOWOF Interrupt Enable"]
    #[inline(always)]
    pub fn dayowof(&mut self) -> DAYOWOF_W {
        DAYOWOF_W { w: self }
    }
    #[doc = "Bit 10 - MONTHTICK Interrupt Enable"]
    #[inline(always)]
    pub fn monthtick(&mut self) -> MONTHTICK_W {
        MONTHTICK_W { w: self }
    }
}
