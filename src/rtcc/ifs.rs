#[doc = "Writer for register IFS"]
pub type W = crate::W<u32, super::IFS>;
#[doc = "Register IFS `reset()`'s with value 0"]
impl crate::ResetValue for super::IFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
impl W {
    #[doc = "Bit 0 - Set OF Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W { w: self }
    }
    #[doc = "Bit 1 - Set CC0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&mut self) -> CC0_W {
        CC0_W { w: self }
    }
    #[doc = "Bit 2 - Set CC1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&mut self) -> CC1_W {
        CC1_W { w: self }
    }
    #[doc = "Bit 3 - Set CC2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&mut self) -> CC2_W {
        CC2_W { w: self }
    }
    #[doc = "Bit 4 - Set OSCFAIL Interrupt Flag"]
    #[inline(always)]
    pub fn oscfail(&mut self) -> OSCFAIL_W {
        OSCFAIL_W { w: self }
    }
    #[doc = "Bit 5 - Set CNTTICK Interrupt Flag"]
    #[inline(always)]
    pub fn cnttick(&mut self) -> CNTTICK_W {
        CNTTICK_W { w: self }
    }
    #[doc = "Bit 6 - Set MINTICK Interrupt Flag"]
    #[inline(always)]
    pub fn mintick(&mut self) -> MINTICK_W {
        MINTICK_W { w: self }
    }
    #[doc = "Bit 7 - Set HOURTICK Interrupt Flag"]
    #[inline(always)]
    pub fn hourtick(&mut self) -> HOURTICK_W {
        HOURTICK_W { w: self }
    }
    #[doc = "Bit 8 - Set DAYTICK Interrupt Flag"]
    #[inline(always)]
    pub fn daytick(&mut self) -> DAYTICK_W {
        DAYTICK_W { w: self }
    }
    #[doc = "Bit 9 - Set DAYOWOF Interrupt Flag"]
    #[inline(always)]
    pub fn dayowof(&mut self) -> DAYOWOF_W {
        DAYOWOF_W { w: self }
    }
    #[doc = "Bit 10 - Set MONTHTICK Interrupt Flag"]
    #[inline(always)]
    pub fn monthtick(&mut self) -> MONTHTICK_W {
        MONTHTICK_W { w: self }
    }
}
