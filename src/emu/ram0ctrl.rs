#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RAM0CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `RAMPOWERDOWN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMPOWERDOWNR {
    #[doc = "None of the RAM blocks powered down"]
    NONE,
    #[doc = "Power down RAM block 7 and above"]
    BLK7,
    #[doc = "Power down RAM block 6 and above"]
    BLK6TO7,
    #[doc = "Power down RAM block 5 and above"]
    BLK5TO7,
    #[doc = "Power down RAM blocks 4 and above"]
    BLK4TO7,
    #[doc = "Power down RAM blocks 3 and above"]
    BLK3TO7,
    #[doc = "Power down RAM blocks 2 and above"]
    BLK2TO7,
    #[doc = "Power down RAM blocks 1 and above"]
    BLK1TO7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAMPOWERDOWNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMPOWERDOWNR::NONE => 0,
            RAMPOWERDOWNR::BLK7 => 64,
            RAMPOWERDOWNR::BLK6TO7 => 96,
            RAMPOWERDOWNR::BLK5TO7 => 112,
            RAMPOWERDOWNR::BLK4TO7 => 120,
            RAMPOWERDOWNR::BLK3TO7 => 124,
            RAMPOWERDOWNR::BLK2TO7 => 126,
            RAMPOWERDOWNR::BLK1TO7 => 127,
            RAMPOWERDOWNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMPOWERDOWNR {
        match value {
            0 => RAMPOWERDOWNR::NONE,
            64 => RAMPOWERDOWNR::BLK7,
            96 => RAMPOWERDOWNR::BLK6TO7,
            112 => RAMPOWERDOWNR::BLK5TO7,
            120 => RAMPOWERDOWNR::BLK4TO7,
            124 => RAMPOWERDOWNR::BLK3TO7,
            126 => RAMPOWERDOWNR::BLK2TO7,
            127 => RAMPOWERDOWNR::BLK1TO7,
            i => RAMPOWERDOWNR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == RAMPOWERDOWNR::NONE
    }
    #[doc = "Checks if the value of the field is `BLK7`"]
    #[inline]
    pub fn is_blk7(&self) -> bool {
        *self == RAMPOWERDOWNR::BLK7
    }
    #[doc = "Checks if the value of the field is `BLK6TO7`"]
    #[inline]
    pub fn is_blk6to7(&self) -> bool {
        *self == RAMPOWERDOWNR::BLK6TO7
    }
    #[doc = "Checks if the value of the field is `BLK5TO7`"]
    #[inline]
    pub fn is_blk5to7(&self) -> bool {
        *self == RAMPOWERDOWNR::BLK5TO7
    }
    #[doc = "Checks if the value of the field is `BLK4TO7`"]
    #[inline]
    pub fn is_blk4to7(&self) -> bool {
        *self == RAMPOWERDOWNR::BLK4TO7
    }
    #[doc = "Checks if the value of the field is `BLK3TO7`"]
    #[inline]
    pub fn is_blk3to7(&self) -> bool {
        *self == RAMPOWERDOWNR::BLK3TO7
    }
    #[doc = "Checks if the value of the field is `BLK2TO7`"]
    #[inline]
    pub fn is_blk2to7(&self) -> bool {
        *self == RAMPOWERDOWNR::BLK2TO7
    }
    #[doc = "Checks if the value of the field is `BLK1TO7`"]
    #[inline]
    pub fn is_blk1to7(&self) -> bool {
        *self == RAMPOWERDOWNR::BLK1TO7
    }
}
#[doc = "Values that can be written to the field `RAMPOWERDOWN`"]
pub enum RAMPOWERDOWNW {
    #[doc = "None of the RAM blocks powered down"]
    NONE,
    #[doc = "Power down RAM block 7 and above"]
    BLK7,
    #[doc = "Power down RAM block 6 and above"]
    BLK6TO7,
    #[doc = "Power down RAM block 5 and above"]
    BLK5TO7,
    #[doc = "Power down RAM blocks 4 and above"]
    BLK4TO7,
    #[doc = "Power down RAM blocks 3 and above"]
    BLK3TO7,
    #[doc = "Power down RAM blocks 2 and above"]
    BLK2TO7,
    #[doc = "Power down RAM blocks 1 and above"]
    BLK1TO7,
}
impl RAMPOWERDOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAMPOWERDOWNW::NONE => 0,
            RAMPOWERDOWNW::BLK7 => 64,
            RAMPOWERDOWNW::BLK6TO7 => 96,
            RAMPOWERDOWNW::BLK5TO7 => 112,
            RAMPOWERDOWNW::BLK4TO7 => 120,
            RAMPOWERDOWNW::BLK3TO7 => 124,
            RAMPOWERDOWNW::BLK2TO7 => 126,
            RAMPOWERDOWNW::BLK1TO7 => 127,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAMPOWERDOWNW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMPOWERDOWNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAMPOWERDOWNW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "None of the RAM blocks powered down"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::NONE)
    }
    #[doc = "Power down RAM block 7 and above"]
    #[inline]
    pub fn blk7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK7)
    }
    #[doc = "Power down RAM block 6 and above"]
    #[inline]
    pub fn blk6to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK6TO7)
    }
    #[doc = "Power down RAM block 5 and above"]
    #[inline]
    pub fn blk5to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK5TO7)
    }
    #[doc = "Power down RAM blocks 4 and above"]
    #[inline]
    pub fn blk4to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK4TO7)
    }
    #[doc = "Power down RAM blocks 3 and above"]
    #[inline]
    pub fn blk3to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK3TO7)
    }
    #[doc = "Power down RAM blocks 2 and above"]
    #[inline]
    pub fn blk2to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK2TO7)
    }
    #[doc = "Power down RAM blocks 1 and above"]
    #[inline]
    pub fn blk1to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK1TO7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - RAM0 Blockset Power-down"]
    #[inline]
    pub fn rampowerdown(&self) -> RAMPOWERDOWNR {
        RAMPOWERDOWNR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - RAM0 Blockset Power-down"]
    #[inline]
    pub fn rampowerdown(&mut self) -> _RAMPOWERDOWNW {
        _RAMPOWERDOWNW { w: self }
    }
}
