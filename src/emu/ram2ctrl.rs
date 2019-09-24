#[doc = "Reader of register RAM2CTRL"]
pub type R = crate::R<u32, super::RAM2CTRL>;
#[doc = "Writer for register RAM2CTRL"]
pub type W = crate::W<u32, super::RAM2CTRL>;
#[doc = "Register RAM2CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RAM2CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RAM2 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMPOWERDOWN_A {
    #[doc = "0: None of the RAM blocks powered down"]
    NONE,
    #[doc = "8: Power down RAM block 3"]
    BLK3,
    #[doc = "12: Power down RAM blocks 2-3"]
    BLK2TO3,
    #[doc = "14: Power down RAM blocks 1-3"]
    BLK1TO3,
    #[doc = "15: Power down RAM blocks 0-3"]
    BLK0TO3,
}
impl From<RAMPOWERDOWN_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPOWERDOWN_A) -> Self {
        match variant {
            RAMPOWERDOWN_A::NONE => 0,
            RAMPOWERDOWN_A::BLK3 => 8,
            RAMPOWERDOWN_A::BLK2TO3 => 12,
            RAMPOWERDOWN_A::BLK1TO3 => 14,
            RAMPOWERDOWN_A::BLK0TO3 => 15,
        }
    }
}
#[doc = "Reader of field `RAMPOWERDOWN`"]
pub type RAMPOWERDOWN_R = crate::R<u8, RAMPOWERDOWN_A>;
impl RAMPOWERDOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAMPOWERDOWN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAMPOWERDOWN_A::NONE),
            8 => Val(RAMPOWERDOWN_A::BLK3),
            12 => Val(RAMPOWERDOWN_A::BLK2TO3),
            14 => Val(RAMPOWERDOWN_A::BLK1TO3),
            15 => Val(RAMPOWERDOWN_A::BLK0TO3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWN_A::NONE
    }
    #[doc = "Checks if the value of the field is `BLK3`"]
    #[inline(always)]
    pub fn is_blk3(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK3
    }
    #[doc = "Checks if the value of the field is `BLK2TO3`"]
    #[inline(always)]
    pub fn is_blk2to3(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK2TO3
    }
    #[doc = "Checks if the value of the field is `BLK1TO3`"]
    #[inline(always)]
    pub fn is_blk1to3(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK1TO3
    }
    #[doc = "Checks if the value of the field is `BLK0TO3`"]
    #[inline(always)]
    pub fn is_blk0to3(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK0TO3
    }
}
#[doc = "Write proxy for field `RAMPOWERDOWN`"]
pub struct RAMPOWERDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPOWERDOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAMPOWERDOWN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::NONE)
    }
    #[doc = "Power down RAM block 3"]
    #[inline(always)]
    pub fn blk3(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK3)
    }
    #[doc = "Power down RAM blocks 2-3"]
    #[inline(always)]
    pub fn blk2to3(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK2TO3)
    }
    #[doc = "Power down RAM blocks 1-3"]
    #[inline(always)]
    pub fn blk1to3(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK1TO3)
    }
    #[doc = "Power down RAM blocks 0-3"]
    #[inline(always)]
    pub fn blk0to3(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK0TO3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - RAM2 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RAMPOWERDOWN_R {
        RAMPOWERDOWN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RAM2 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&mut self) -> RAMPOWERDOWN_W {
        RAMPOWERDOWN_W { w: self }
    }
}
