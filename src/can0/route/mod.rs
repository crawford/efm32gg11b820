#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROUTE {
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
pub struct TXPENR {
    bits: bool,
}
impl TXPENR {
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
#[doc = "Possible values of the field `RXLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
    #[doc = "Location 7"]
    LOC7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXLOCR::LOC0 => 0,
            RXLOCR::LOC1 => 1,
            RXLOCR::LOC2 => 2,
            RXLOCR::LOC3 => 3,
            RXLOCR::LOC4 => 4,
            RXLOCR::LOC5 => 5,
            RXLOCR::LOC6 => 6,
            RXLOCR::LOC7 => 7,
            RXLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXLOCR {
        match value {
            0 => RXLOCR::LOC0,
            1 => RXLOCR::LOC1,
            2 => RXLOCR::LOC2,
            3 => RXLOCR::LOC3,
            4 => RXLOCR::LOC4,
            5 => RXLOCR::LOC5,
            6 => RXLOCR::LOC6,
            7 => RXLOCR::LOC7,
            i => RXLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == RXLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == RXLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == RXLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == RXLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == RXLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == RXLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == RXLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == RXLOCR::LOC7
    }
}
#[doc = "Possible values of the field `TXLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
    #[doc = "Location 7"]
    LOC7,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXLOCR::LOC0 => 0,
            TXLOCR::LOC1 => 1,
            TXLOCR::LOC2 => 2,
            TXLOCR::LOC3 => 3,
            TXLOCR::LOC4 => 4,
            TXLOCR::LOC5 => 5,
            TXLOCR::LOC6 => 6,
            TXLOCR::LOC7 => 7,
            TXLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXLOCR {
        match value {
            0 => TXLOCR::LOC0,
            1 => TXLOCR::LOC1,
            2 => TXLOCR::LOC2,
            3 => TXLOCR::LOC3,
            4 => TXLOCR::LOC4,
            5 => TXLOCR::LOC5,
            6 => TXLOCR::LOC6,
            7 => TXLOCR::LOC7,
            i => TXLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == TXLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == TXLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == TXLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == TXLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == TXLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == TXLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == TXLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == TXLOCR::LOC7
    }
}
#[doc = r" Proxy"]
pub struct _TXPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPENW<'a> {
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
#[doc = "Values that can be written to the field `RXLOC`"]
pub enum RXLOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
    #[doc = "Location 7"]
    LOC7,
}
impl RXLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXLOCW::LOC0 => 0,
            RXLOCW::LOC1 => 1,
            RXLOCW::LOC2 => 2,
            RXLOCW::LOC3 => 3,
            RXLOCW::LOC4 => 4,
            RXLOCW::LOC5 => 5,
            RXLOCW::LOC6 => 6,
            RXLOCW::LOC7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _RXLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(RXLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(RXLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(RXLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(RXLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(RXLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(RXLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(RXLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(RXLOCW::LOC7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXLOC`"]
pub enum TXLOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
    #[doc = "Location 7"]
    LOC7,
}
impl TXLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXLOCW::LOC0 => 0,
            TXLOCW::LOC1 => 1,
            TXLOCW::LOC2 => 2,
            TXLOCW::LOC3 => 3,
            TXLOCW::LOC4 => 4,
            TXLOCW::LOC5 => 5,
            TXLOCW::LOC6 => 6,
            TXLOCW::LOC7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _TXLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(TXLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(TXLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(TXLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(TXLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(TXLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(TXLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(TXLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(TXLOCW::LOC7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - TX Pin Enable"]
    #[inline]
    pub fn txpen(&self) -> TXPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXPENR { bits }
    }
    #[doc = "Bits 2:7 - RX Pin Location"]
    #[inline]
    pub fn rxloc(&self) -> RXLOCR {
        RXLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - TX Pin Location"]
    #[inline]
    pub fn txloc(&self) -> TXLOCR {
        TXLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - TX Pin Enable"]
    #[inline]
    pub fn txpen(&mut self) -> _TXPENW {
        _TXPENW { w: self }
    }
    #[doc = "Bits 2:7 - RX Pin Location"]
    #[inline]
    pub fn rxloc(&mut self) -> _RXLOCW {
        _RXLOCW { w: self }
    }
    #[doc = "Bits 8:13 - TX Pin Location"]
    #[inline]
    pub fn txloc(&mut self) -> _TXLOCW {
        _TXLOCW { w: self }
    }
}
