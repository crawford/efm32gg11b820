#[doc = "Reader of register EXTILEVEL"]
pub type R = crate::R<u32, super::EXTILEVEL>;
#[doc = "Writer for register EXTILEVEL"]
pub type W = crate::W<u32, super::EXTILEVEL>;
#[doc = "Register EXTILEVEL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTILEVEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EM4WU0`"]
pub type EM4WU0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WU0`"]
pub struct EM4WU0_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `EM4WU1`"]
pub type EM4WU1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WU1`"]
pub struct EM4WU1_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `EM4WU2`"]
pub type EM4WU2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WU2`"]
pub struct EM4WU2_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `EM4WU3`"]
pub type EM4WU3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WU3`"]
pub struct EM4WU3_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `EM4WU4`"]
pub type EM4WU4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WU4`"]
pub struct EM4WU4_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `EM4WU5`"]
pub type EM4WU5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WU5`"]
pub struct EM4WU5_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `EM4WU6`"]
pub type EM4WU6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WU6`"]
pub struct EM4WU6_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `EM4WU7`"]
pub type EM4WU7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WU7`"]
pub struct EM4WU7_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `EM4WU8`"]
pub type EM4WU8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WU8`"]
pub struct EM4WU8_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `EM4WU9`"]
pub type EM4WU9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WU9`"]
pub struct EM4WU9_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - EM4 Wake Up Level for EM4WU0 Pin"]
    #[inline(always)]
    pub fn em4wu0(&self) -> EM4WU0_R {
        EM4WU0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - EM4 Wake Up Level for EM4WU1 Pin"]
    #[inline(always)]
    pub fn em4wu1(&self) -> EM4WU1_R {
        EM4WU1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - EM4 Wake Up Level for EM4WU2 Pin"]
    #[inline(always)]
    pub fn em4wu2(&self) -> EM4WU2_R {
        EM4WU2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - EM4 Wake Up Level for EM4WU3 Pin"]
    #[inline(always)]
    pub fn em4wu3(&self) -> EM4WU3_R {
        EM4WU3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - EM4 Wake Up Level for EM4WU4 Pin"]
    #[inline(always)]
    pub fn em4wu4(&self) -> EM4WU4_R {
        EM4WU4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - EM4 Wake Up Level for EM4WU5 Pin"]
    #[inline(always)]
    pub fn em4wu5(&self) -> EM4WU5_R {
        EM4WU5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - EM4 Wake Up Level for EM4WU6 Pin"]
    #[inline(always)]
    pub fn em4wu6(&self) -> EM4WU6_R {
        EM4WU6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - EM4 Wake Up Level for EM4WU7 Pin"]
    #[inline(always)]
    pub fn em4wu7(&self) -> EM4WU7_R {
        EM4WU7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - EM4 Wake Up Level for EM4WU8 Pin"]
    #[inline(always)]
    pub fn em4wu8(&self) -> EM4WU8_R {
        EM4WU8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - EM4 Wake Up Level for EM4WU9 Pin"]
    #[inline(always)]
    pub fn em4wu9(&self) -> EM4WU9_R {
        EM4WU9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - EM4 Wake Up Level for EM4WU0 Pin"]
    #[inline(always)]
    pub fn em4wu0(&mut self) -> EM4WU0_W {
        EM4WU0_W { w: self }
    }
    #[doc = "Bit 17 - EM4 Wake Up Level for EM4WU1 Pin"]
    #[inline(always)]
    pub fn em4wu1(&mut self) -> EM4WU1_W {
        EM4WU1_W { w: self }
    }
    #[doc = "Bit 18 - EM4 Wake Up Level for EM4WU2 Pin"]
    #[inline(always)]
    pub fn em4wu2(&mut self) -> EM4WU2_W {
        EM4WU2_W { w: self }
    }
    #[doc = "Bit 19 - EM4 Wake Up Level for EM4WU3 Pin"]
    #[inline(always)]
    pub fn em4wu3(&mut self) -> EM4WU3_W {
        EM4WU3_W { w: self }
    }
    #[doc = "Bit 20 - EM4 Wake Up Level for EM4WU4 Pin"]
    #[inline(always)]
    pub fn em4wu4(&mut self) -> EM4WU4_W {
        EM4WU4_W { w: self }
    }
    #[doc = "Bit 21 - EM4 Wake Up Level for EM4WU5 Pin"]
    #[inline(always)]
    pub fn em4wu5(&mut self) -> EM4WU5_W {
        EM4WU5_W { w: self }
    }
    #[doc = "Bit 22 - EM4 Wake Up Level for EM4WU6 Pin"]
    #[inline(always)]
    pub fn em4wu6(&mut self) -> EM4WU6_W {
        EM4WU6_W { w: self }
    }
    #[doc = "Bit 23 - EM4 Wake Up Level for EM4WU7 Pin"]
    #[inline(always)]
    pub fn em4wu7(&mut self) -> EM4WU7_W {
        EM4WU7_W { w: self }
    }
    #[doc = "Bit 24 - EM4 Wake Up Level for EM4WU8 Pin"]
    #[inline(always)]
    pub fn em4wu8(&mut self) -> EM4WU8_W {
        EM4WU8_W { w: self }
    }
    #[doc = "Bit 25 - EM4 Wake Up Level for EM4WU9 Pin"]
    #[inline(always)]
    pub fn em4wu9(&mut self) -> EM4WU9_W {
        EM4WU9_W { w: self }
    }
}
