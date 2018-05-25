#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RAM1CTRL {
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAMPOWERDOWNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAMPOWERDOWNR::NONE => 0,
            RAMPOWERDOWNR::BLK7 => 128,
            RAMPOWERDOWNR::BLK6TO7 => 192,
            RAMPOWERDOWNR::BLK5TO7 => 224,
            RAMPOWERDOWNR::BLK4TO7 => 240,
            RAMPOWERDOWNR::BLK3TO7 => 248,
            RAMPOWERDOWNR::BLK2TO7 => 252,
            RAMPOWERDOWNR::BLK1TO7 => 254,
            RAMPOWERDOWNR::BLK0TO7 => 255,
            RAMPOWERDOWNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAMPOWERDOWNR {
        match value {
            0 => RAMPOWERDOWNR::NONE,
            128 => RAMPOWERDOWNR::BLK7,
            192 => RAMPOWERDOWNR::BLK6TO7,
            224 => RAMPOWERDOWNR::BLK5TO7,
            240 => RAMPOWERDOWNR::BLK4TO7,
            248 => RAMPOWERDOWNR::BLK3TO7,
            252 => RAMPOWERDOWNR::BLK2TO7,
            254 => RAMPOWERDOWNR::BLK1TO7,
            255 => RAMPOWERDOWNR::BLK0TO7,
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
    #[doc = "Checks if the value of the field is `BLK0TO7`"]
    #[inline]
    pub fn is_blk0to7(&self) -> bool {
        *self == RAMPOWERDOWNR::BLK0TO7
    }
}
#[doc = "Values that can be written to the field `RAMPOWERDOWN`"]
pub enum RAMPOWERDOWNW {
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
impl RAMPOWERDOWNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAMPOWERDOWNW::NONE => 0,
            RAMPOWERDOWNW::BLK7 => 128,
            RAMPOWERDOWNW::BLK6TO7 => 192,
            RAMPOWERDOWNW::BLK5TO7 => 224,
            RAMPOWERDOWNW::BLK4TO7 => 240,
            RAMPOWERDOWNW::BLK3TO7 => 248,
            RAMPOWERDOWNW::BLK2TO7 => 252,
            RAMPOWERDOWNW::BLK1TO7 => 254,
            RAMPOWERDOWNW::BLK0TO7 => 255,
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
    #[doc = "Power down RAM block 7 (address range 0x2003C000-0x2003FFFF)"]
    #[inline]
    pub fn blk7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK7)
    }
    #[doc = "Power down RAM blocks 6-7 (address range 0x20038000-0x2003FFFF)"]
    #[inline]
    pub fn blk6to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK6TO7)
    }
    #[doc = "Power down RAM blocks 5-7 (address range 0x20034000-0x2003FFFF)"]
    #[inline]
    pub fn blk5to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK5TO7)
    }
    #[doc = "Power down RAM blocks 4-7 (address range 0x20030000-0x2003FFFF)"]
    #[inline]
    pub fn blk4to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK4TO7)
    }
    #[doc = "Power down RAM blocks 3-7 (address range 0x2002C000-0x2003FFFF)"]
    #[inline]
    pub fn blk3to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK3TO7)
    }
    #[doc = "Power down RAM blocks 2-7 (address range 0x20028000-0x2003FFFF)"]
    #[inline]
    pub fn blk2to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK2TO7)
    }
    #[doc = "Power down RAM blocks 1-7 (address range 0x20024000-0x2003FFFF)"]
    #[inline]
    pub fn blk1to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK1TO7)
    }
    #[doc = "Power down RAM blocks 0-7 (address range 0x20020000-0x2003FFFF)"]
    #[inline]
    pub fn blk0to7(self) -> &'a mut W {
        self.variant(RAMPOWERDOWNW::BLK0TO7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - RAM1 Blockset Power-down"]
    #[inline]
    pub fn rampowerdown(&self) -> RAMPOWERDOWNR {
        RAMPOWERDOWNR::_from({
            const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - RAM1 Blockset Power-down"]
    #[inline]
    pub fn rampowerdown(&mut self) -> _RAMPOWERDOWNW {
        _RAMPOWERDOWNW { w: self }
    }
}
