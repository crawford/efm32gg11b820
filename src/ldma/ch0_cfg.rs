#[doc = "Reader of register CH0_CFG"]
pub type R = crate::R<u32, super::CH0_CFG>;
#[doc = "Writer for register CH0_CFG"]
pub type W = crate::W<u32, super::CH0_CFG>;
#[doc = "Register CH0_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CH0_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Arbitration Slot Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ARBSLOTS_A {
    #[doc = "0: One arbitration slot selected"]
    ONE = 0,
    #[doc = "1: Two arbitration slots selected"]
    TWO = 1,
    #[doc = "2: Four arbitration slots selected"]
    FOUR = 2,
    #[doc = "3: Eight arbitration slots selected"]
    EIGHT = 3,
}
impl From<ARBSLOTS_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBSLOTS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ARBSLOTS`"]
pub type ARBSLOTS_R = crate::R<u8, ARBSLOTS_A>;
impl ARBSLOTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBSLOTS_A {
        match self.bits {
            0 => ARBSLOTS_A::ONE,
            1 => ARBSLOTS_A::TWO,
            2 => ARBSLOTS_A::FOUR,
            3 => ARBSLOTS_A::EIGHT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ARBSLOTS_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == ARBSLOTS_A::TWO
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == ARBSLOTS_A::FOUR
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == ARBSLOTS_A::EIGHT
    }
}
#[doc = "Write proxy for field `ARBSLOTS`"]
pub struct ARBSLOTS_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBSLOTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBSLOTS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "One arbitration slot selected"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ARBSLOTS_A::ONE)
    }
    #[doc = "Two arbitration slots selected"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(ARBSLOTS_A::TWO)
    }
    #[doc = "Four arbitration slots selected"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(ARBSLOTS_A::FOUR)
    }
    #[doc = "Eight arbitration slots selected"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(ARBSLOTS_A::EIGHT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SRCINCSIGN`"]
pub type SRCINCSIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRCINCSIGN`"]
pub struct SRCINCSIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCINCSIGN_W<'a> {
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
#[doc = "Reader of field `DSTINCSIGN`"]
pub type DSTINCSIGN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSTINCSIGN`"]
pub struct DSTINCSIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTINCSIGN_W<'a> {
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
impl R {
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline(always)]
    pub fn arbslots(&self) -> ARBSLOTS_R {
        ARBSLOTS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline(always)]
    pub fn srcincsign(&self) -> SRCINCSIGN_R {
        SRCINCSIGN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline(always)]
    pub fn dstincsign(&self) -> DSTINCSIGN_R {
        DSTINCSIGN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Arbitration Slot Number Select"]
    #[inline(always)]
    pub fn arbslots(&mut self) -> ARBSLOTS_W {
        ARBSLOTS_W { w: self }
    }
    #[doc = "Bit 20 - Source Address Increment Sign"]
    #[inline(always)]
    pub fn srcincsign(&mut self) -> SRCINCSIGN_W {
        SRCINCSIGN_W { w: self }
    }
    #[doc = "Bit 21 - Destination Address Increment Sign"]
    #[inline(always)]
    pub fn dstincsign(&mut self) -> DSTINCSIGN_W {
        DSTINCSIGN_W { w: self }
    }
}
