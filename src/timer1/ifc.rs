#[doc = "Writer for register IFC"]
pub type W = crate::W<u32, super::IFC>;
#[doc = "Register IFC `reset()`'s with value 0"]
impl crate::ResetValue for super::IFC {
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
impl W {
    #[doc = "Bit 0 - Clear OF Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W { w: self }
    }
    #[doc = "Bit 1 - Clear UF Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W {
        UF_W { w: self }
    }
    #[doc = "Bit 2 - Clear DIRCHG Interrupt Flag"]
    #[inline(always)]
    pub fn dirchg(&mut self) -> DIRCHG_W {
        DIRCHG_W { w: self }
    }
    #[doc = "Bit 4 - Clear CC0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&mut self) -> CC0_W {
        CC0_W { w: self }
    }
    #[doc = "Bit 5 - Clear CC1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&mut self) -> CC1_W {
        CC1_W { w: self }
    }
    #[doc = "Bit 6 - Clear CC2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&mut self) -> CC2_W {
        CC2_W { w: self }
    }
    #[doc = "Bit 7 - Clear CC3 Interrupt Flag"]
    #[inline(always)]
    pub fn cc3(&mut self) -> CC3_W {
        CC3_W { w: self }
    }
    #[doc = "Bit 8 - Clear ICBOF0 Interrupt Flag"]
    #[inline(always)]
    pub fn icbof0(&mut self) -> ICBOF0_W {
        ICBOF0_W { w: self }
    }
    #[doc = "Bit 9 - Clear ICBOF1 Interrupt Flag"]
    #[inline(always)]
    pub fn icbof1(&mut self) -> ICBOF1_W {
        ICBOF1_W { w: self }
    }
    #[doc = "Bit 10 - Clear ICBOF2 Interrupt Flag"]
    #[inline(always)]
    pub fn icbof2(&mut self) -> ICBOF2_W {
        ICBOF2_W { w: self }
    }
    #[doc = "Bit 11 - Clear ICBOF3 Interrupt Flag"]
    #[inline(always)]
    pub fn icbof3(&mut self) -> ICBOF3_W {
        ICBOF3_W { w: self }
    }
}
