#[doc = "Reader of register RAM1CTRL"]
pub type R = crate::R<u32, super::RAM1CTRL>;
#[doc = "Writer for register RAM1CTRL"]
pub type W = crate::W<u32, super::RAM1CTRL>;
#[doc = "Register RAM1CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::RAM1CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `RAMPOWERDOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMPOWERDOWN_A {
    #[doc = "None of the RAM blocks powered down"]
    NONE,
    #[doc = "Power down RAM block 7 (address range 0x2003C000-0x2003FFFF)"]
    BLK7,
    #[doc = "Power down RAM blocks 6-7 (address range 0x20038000-0x2003FFFF)"]
    BLK6TO7,
    #[doc = "Power down RAM blocks 5-7 (address range 0x20034000-0x2003FFFF)"]
    BLK5TO7,
    #[doc = "Power down RAM blocks 4-7 (address range 0x20030000-0x2003FFFF)"]
    BLK4TO7,
    #[doc = "Power down RAM blocks 3-7 (address range 0x2002C000-0x2003FFFF)"]
    BLK3TO7,
    #[doc = "Power down RAM blocks 2-7 (address range 0x20028000-0x2003FFFF)"]
    BLK2TO7,
    #[doc = "Power down RAM blocks 1-7 (address range 0x20024000-0x2003FFFF)"]
    BLK1TO7,
    #[doc = "Power down RAM blocks 0-7 (address range 0x20020000-0x2003FFFF)"]
    BLK0TO7,
}
impl crate::ToBits<u8> for RAMPOWERDOWN_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RAMPOWERDOWN_A::NONE => 0,
            RAMPOWERDOWN_A::BLK7 => 128,
            RAMPOWERDOWN_A::BLK6TO7 => 192,
            RAMPOWERDOWN_A::BLK5TO7 => 224,
            RAMPOWERDOWN_A::BLK4TO7 => 240,
            RAMPOWERDOWN_A::BLK3TO7 => 248,
            RAMPOWERDOWN_A::BLK2TO7 => 252,
            RAMPOWERDOWN_A::BLK1TO7 => 254,
            RAMPOWERDOWN_A::BLK0TO7 => 255,
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
            128 => Val(RAMPOWERDOWN_A::BLK7),
            192 => Val(RAMPOWERDOWN_A::BLK6TO7),
            224 => Val(RAMPOWERDOWN_A::BLK5TO7),
            240 => Val(RAMPOWERDOWN_A::BLK4TO7),
            248 => Val(RAMPOWERDOWN_A::BLK3TO7),
            252 => Val(RAMPOWERDOWN_A::BLK2TO7),
            254 => Val(RAMPOWERDOWN_A::BLK1TO7),
            255 => Val(RAMPOWERDOWN_A::BLK0TO7),
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
    #[doc = "Checks if the value of the field is `BLK0TO7`"]
    #[inline(always)]
    pub fn is_blk0to7(&self) -> bool {
        *self == RAMPOWERDOWN_A::BLK0TO7
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
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::NONE)
    }
    #[doc = "Power down RAM block 7 (address range 0x2003C000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK7)
    }
    #[doc = "Power down RAM blocks 6-7 (address range 0x20038000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk6to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK6TO7)
    }
    #[doc = "Power down RAM blocks 5-7 (address range 0x20034000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk5to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK5TO7)
    }
    #[doc = "Power down RAM blocks 4-7 (address range 0x20030000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk4to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK4TO7)
    }
    #[doc = "Power down RAM blocks 3-7 (address range 0x2002C000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk3to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK3TO7)
    }
    #[doc = "Power down RAM blocks 2-7 (address range 0x20028000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk2to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK2TO7)
    }
    #[doc = "Power down RAM blocks 1-7 (address range 0x20024000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk1to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK1TO7)
    }
    #[doc = "Power down RAM blocks 0-7 (address range 0x20020000-0x2003FFFF)"]
    #[inline(always)]
    pub fn blk0to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWN_A::BLK0TO7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - RAM1 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&self) -> RAMPOWERDOWN_R {
        RAMPOWERDOWN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RAM1 Blockset Power-down"]
    #[inline(always)]
    pub fn rampowerdown(&mut self) -> RAMPOWERDOWN_W {
        RAMPOWERDOWN_W { w: self }
    }
}
