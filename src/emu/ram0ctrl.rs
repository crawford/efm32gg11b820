#[doc = "Reader of register RAM0CTRL"]
pub type R = crate::R<u32, super::RAM0CTRL>;
#[doc = "Writer for register RAM0CTRL"]
pub type W = crate::W<u32, super::RAM0CTRL>;
#[doc = "Register RAM0CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RAM0CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RAM0 Blockset Power-down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RAMPOWERDOWN_A {
    #[doc = "0: None of the RAM blocks powered down"]
    NONE = 0,
    #[doc = "64: Power down RAM block 7 and above"]
    BLK7 = 64,
    #[doc = "96: Power down RAM block 6 and above"]
    BLK6TO7 = 96,
    #[doc = "112: Power down RAM block 5 and above"]
    BLK5TO7 = 112,
    #[doc = "120: Power down RAM blocks 4 and above"]
    BLK4TO7 = 120,
    #[doc = "124: Power down RAM blocks 3 and above"]
    BLK3TO7 = 124,
    #[doc = "126: Power down RAM blocks 2 and above"]
    BLK2TO7 = 126,
    #[doc = "127: Power down RAM blocks 1 and above"]
    BLK1TO7 = 127,
}
impl From<RAMPOWERDOWN_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMPOWERDOWN_A) -> Self {
        variant as _
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
            64 => Val(RAMPOWERDOWN_A::BLK7),
            96 => Val(RAMPOWERDOWN_A::BLK6TO7),
            112 => Val(RAMPOWERDOWN_A::BLK5TO7),
            120 => Val(RAMPOWERDOWN_A::BLK4TO7),
            124 => Val(RAMPOWERDOWN_A::BLK3TO7),
            126 => Val(RAMPOWERDOWN_A::BLK2TO7),
            127 => Val(RAMPOWERDOWN_A::BLK1TO7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWN_A::NONE
    }
    #[doc = "Checks if the value of the field is `BLK7`"]
    #[inline(always)]
    pub fn is_blk7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK7
    }
    #[doc = "Checks if the value of the field is `BLK6TO7`"]
    #[inline(always)]
    pub fn is_blk6to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK6TO7
    }
    #[doc = "Checks if the value of the field is `BLK5TO7`"]
    #[inline(always)]
    pub fn is_blk5to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK5TO7
    }
    #[doc = "Checks if the value of the field is `BLK4TO7`"]
    #[inline(always)]
    pub fn is_blk4to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK4TO7
    }
    #[doc = "Checks if the value of the field is `BLK3TO7`"]
    #[inline(always)]
    pub fn is_blk3to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK3TO7
    }
    #[doc = "Checks if the value of the field is `BLK2TO7`"]
    #[inline(always)]
    pub fn is_blk2to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK2TO7
    }
    #[doc = "Checks if the value of the field is `BLK1TO7`"]
    #[inline(always)]
    pub fn is_blk1to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK1TO7
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
    #[doc = "Power down RAM block 7 and above"]
    #[inline(always)]
    pub fn blk7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK7)
    }
    #[doc = "Power down RAM block 6 and above"]
    #[inline(always)]
    pub fn blk6to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK6TO7)
    }
    #[doc = "Power down RAM block 5 and above"]
    #[inline(always)]
    pub fn blk5to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK5TO7)
    }
    #[doc = "Power down RAM blocks 4 and above"]
    #[inline(always)]
    pub fn blk4to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK4TO7)
    }
    #[doc = "Power down RAM blocks 3 and above"]
    #[inline(always)]
    pub fn blk3to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK3TO7)
    }
    #[doc = "Power down RAM blocks 2 and above"]
    #[inline(always)]
    pub fn blk2to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK2TO7)
    }
    #[doc = "Power down RAM blocks 1 and above"]
    #[inline(always)]
    pub fn blk1to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK1TO7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - RAM0 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RAMPOWERDOWN_R {
        RAMPOWERDOWN_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RAM0 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&mut self) -> RAMPOWERDOWN_W {
        RAMPOWERDOWN_W { w: self }
    }
}
