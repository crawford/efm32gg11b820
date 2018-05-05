#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTIFCTRL {
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
        R {
            bits: self.register.get(),
        }
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
#[doc = r" Value of the field"]
pub struct ENR {
    bits: bool,
}
impl ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `APORTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APORTSELR {
    #[doc = "APORT0X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0XCH0."]
    APORT0X,
    #[doc = "APORT0Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0YCH0."]
    APORT0Y,
    #[doc = "APORT1X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    APORT1X,
    #[doc = "APORT1Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    APORT1Y,
    #[doc = "APORT1X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    APORT1XY,
    #[doc = "APORT2X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    APORT2X,
    #[doc = "APORT2Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    APORT2Y,
    #[doc = "APORT2Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    APORT2YX,
    #[doc = "APORT3X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    APORT3X,
    #[doc = "APORT3Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    APORT3Y,
    #[doc = "APORT3X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    APORT3XY,
    #[doc = "APORT4X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    APORT4X,
    #[doc = "APORT4Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    APORT4Y,
    #[doc = "APORT4Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    APORT4YX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl APORTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            APORTSELR::APORT0X => 0,
            APORTSELR::APORT0Y => 1,
            APORTSELR::APORT1X => 2,
            APORTSELR::APORT1Y => 3,
            APORTSELR::APORT1XY => 4,
            APORTSELR::APORT2X => 5,
            APORTSELR::APORT2Y => 6,
            APORTSELR::APORT2YX => 7,
            APORTSELR::APORT3X => 8,
            APORTSELR::APORT3Y => 9,
            APORTSELR::APORT3XY => 10,
            APORTSELR::APORT4X => 11,
            APORTSELR::APORT4Y => 12,
            APORTSELR::APORT4YX => 13,
            APORTSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> APORTSELR {
        match value {
            0 => APORTSELR::APORT0X,
            1 => APORTSELR::APORT0Y,
            2 => APORTSELR::APORT1X,
            3 => APORTSELR::APORT1Y,
            4 => APORTSELR::APORT1XY,
            5 => APORTSELR::APORT2X,
            6 => APORTSELR::APORT2Y,
            7 => APORTSELR::APORT2YX,
            8 => APORTSELR::APORT3X,
            9 => APORTSELR::APORT3Y,
            10 => APORTSELR::APORT3XY,
            11 => APORTSELR::APORT4X,
            12 => APORTSELR::APORT4Y,
            13 => APORTSELR::APORT4YX,
            i => APORTSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT0X`"]
    #[inline]
    pub fn is_aport0x(&self) -> bool {
        *self == APORTSELR::APORT0X
    }
    #[doc = "Checks if the value of the field is `APORT0Y`"]
    #[inline]
    pub fn is_aport0y(&self) -> bool {
        *self == APORTSELR::APORT0Y
    }
    #[doc = "Checks if the value of the field is `APORT1X`"]
    #[inline]
    pub fn is_aport1x(&self) -> bool {
        *self == APORTSELR::APORT1X
    }
    #[doc = "Checks if the value of the field is `APORT1Y`"]
    #[inline]
    pub fn is_aport1y(&self) -> bool {
        *self == APORTSELR::APORT1Y
    }
    #[doc = "Checks if the value of the field is `APORT1XY`"]
    #[inline]
    pub fn is_aport1xy(&self) -> bool {
        *self == APORTSELR::APORT1XY
    }
    #[doc = "Checks if the value of the field is `APORT2X`"]
    #[inline]
    pub fn is_aport2x(&self) -> bool {
        *self == APORTSELR::APORT2X
    }
    #[doc = "Checks if the value of the field is `APORT2Y`"]
    #[inline]
    pub fn is_aport2y(&self) -> bool {
        *self == APORTSELR::APORT2Y
    }
    #[doc = "Checks if the value of the field is `APORT2YX`"]
    #[inline]
    pub fn is_aport2yx(&self) -> bool {
        *self == APORTSELR::APORT2YX
    }
    #[doc = "Checks if the value of the field is `APORT3X`"]
    #[inline]
    pub fn is_aport3x(&self) -> bool {
        *self == APORTSELR::APORT3X
    }
    #[doc = "Checks if the value of the field is `APORT3Y`"]
    #[inline]
    pub fn is_aport3y(&self) -> bool {
        *self == APORTSELR::APORT3Y
    }
    #[doc = "Checks if the value of the field is `APORT3XY`"]
    #[inline]
    pub fn is_aport3xy(&self) -> bool {
        *self == APORTSELR::APORT3XY
    }
    #[doc = "Checks if the value of the field is `APORT4X`"]
    #[inline]
    pub fn is_aport4x(&self) -> bool {
        *self == APORTSELR::APORT4X
    }
    #[doc = "Checks if the value of the field is `APORT4Y`"]
    #[inline]
    pub fn is_aport4y(&self) -> bool {
        *self == APORTSELR::APORT4Y
    }
    #[doc = "Checks if the value of the field is `APORT4YX`"]
    #[inline]
    pub fn is_aport4yx(&self) -> bool {
        *self == APORTSELR::APORT4YX
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `APORTSEL`"]
pub enum APORTSELW {
    #[doc = "APORT0X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0XCH0."]
    APORT0X,
    #[doc = "APORT0Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0YCH0."]
    APORT0Y,
    #[doc = "APORT1X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    APORT1X,
    #[doc = "APORT1Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    APORT1Y,
    #[doc = "APORT1X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    APORT1XY,
    #[doc = "APORT2X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    APORT2X,
    #[doc = "APORT2Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    APORT2Y,
    #[doc = "APORT2Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    APORT2YX,
    #[doc = "APORT3X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    APORT3X,
    #[doc = "APORT3Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    APORT3Y,
    #[doc = "APORT3X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    APORT3XY,
    #[doc = "APORT4X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    APORT4X,
    #[doc = "APORT4Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    APORT4Y,
    #[doc = "APORT4Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    APORT4YX,
}
impl APORTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            APORTSELW::APORT0X => 0,
            APORTSELW::APORT0Y => 1,
            APORTSELW::APORT1X => 2,
            APORTSELW::APORT1Y => 3,
            APORTSELW::APORT1XY => 4,
            APORTSELW::APORT2X => 5,
            APORTSELW::APORT2Y => 6,
            APORTSELW::APORT2YX => 7,
            APORTSELW::APORT3X => 8,
            APORTSELW::APORT3Y => 9,
            APORTSELW::APORT3XY => 10,
            APORTSELW::APORT4X => 11,
            APORTSELW::APORT4Y => 12,
            APORTSELW::APORT4YX => 13,
        }
    }
}
#[doc = r" Proxy"]
pub struct _APORTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _APORTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APORTSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "APORT0X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0XCH0."]
    #[inline]
    pub fn aport0x(self) -> &'a mut W {
        self.variant(APORTSELW::APORT0X)
    }
    #[doc = "APORT0Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0YCH0."]
    #[inline]
    pub fn aport0y(self) -> &'a mut W {
        self.variant(APORTSELW::APORT0Y)
    }
    #[doc = "APORT1X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline]
    pub fn aport1x(self) -> &'a mut W {
        self.variant(APORTSELW::APORT1X)
    }
    #[doc = "APORT1Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline]
    pub fn aport1y(self) -> &'a mut W {
        self.variant(APORTSELW::APORT1Y)
    }
    #[doc = "APORT1X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline]
    pub fn aport1xy(self) -> &'a mut W {
        self.variant(APORTSELW::APORT1XY)
    }
    #[doc = "APORT2X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline]
    pub fn aport2x(self) -> &'a mut W {
        self.variant(APORTSELW::APORT2X)
    }
    #[doc = "APORT2Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline]
    pub fn aport2y(self) -> &'a mut W {
        self.variant(APORTSELW::APORT2Y)
    }
    #[doc = "APORT2Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline]
    pub fn aport2yx(self) -> &'a mut W {
        self.variant(APORTSELW::APORT2YX)
    }
    #[doc = "APORT3X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline]
    pub fn aport3x(self) -> &'a mut W {
        self.variant(APORTSELW::APORT3X)
    }
    #[doc = "APORT3Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline]
    pub fn aport3y(self) -> &'a mut W {
        self.variant(APORTSELW::APORT3Y)
    }
    #[doc = "APORT3X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline]
    pub fn aport3xy(self) -> &'a mut W {
        self.variant(APORTSELW::APORT3XY)
    }
    #[doc = "APORT4X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline]
    pub fn aport4x(self) -> &'a mut W {
        self.variant(APORTSELW::APORT4X)
    }
    #[doc = "APORT4Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline]
    pub fn aport4y(self) -> &'a mut W {
        self.variant(APORTSELW::APORT4Y)
    }
    #[doc = "APORT4Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline]
    pub fn aport4yx(self) -> &'a mut W {
        self.variant(APORTSELW::APORT4YX)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Enable External Interface"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bits 4:7 - APORT Selection for External Interface"]
    #[inline]
    pub fn aportsel(&self) -> APORTSELR {
        APORTSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Enable External Interface"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bits 4:7 - APORT Selection for External Interface"]
    #[inline]
    pub fn aportsel(&mut self) -> _APORTSELW {
        _APORTSELW { w: self }
    }
}
