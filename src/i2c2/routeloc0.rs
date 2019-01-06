#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROUTELOC0 {
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
#[doc = "Possible values of the field `SDALOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDALOCR {
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
impl SDALOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDALOCR::LOC0 => 0,
            SDALOCR::LOC1 => 1,
            SDALOCR::LOC2 => 2,
            SDALOCR::LOC3 => 3,
            SDALOCR::LOC4 => 4,
            SDALOCR::LOC5 => 5,
            SDALOCR::LOC6 => 6,
            SDALOCR::LOC7 => 7,
            SDALOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDALOCR {
        match value {
            0 => SDALOCR::LOC0,
            1 => SDALOCR::LOC1,
            2 => SDALOCR::LOC2,
            3 => SDALOCR::LOC3,
            4 => SDALOCR::LOC4,
            5 => SDALOCR::LOC5,
            6 => SDALOCR::LOC6,
            7 => SDALOCR::LOC7,
            i => SDALOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == SDALOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == SDALOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == SDALOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == SDALOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == SDALOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == SDALOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == SDALOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == SDALOCR::LOC7
    }
}
#[doc = "Possible values of the field `SCLLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLLOCR {
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
impl SCLLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCLLOCR::LOC0 => 0,
            SCLLOCR::LOC1 => 1,
            SCLLOCR::LOC2 => 2,
            SCLLOCR::LOC3 => 3,
            SCLLOCR::LOC4 => 4,
            SCLLOCR::LOC5 => 5,
            SCLLOCR::LOC6 => 6,
            SCLLOCR::LOC7 => 7,
            SCLLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCLLOCR {
        match value {
            0 => SCLLOCR::LOC0,
            1 => SCLLOCR::LOC1,
            2 => SCLLOCR::LOC2,
            3 => SCLLOCR::LOC3,
            4 => SCLLOCR::LOC4,
            5 => SCLLOCR::LOC5,
            6 => SCLLOCR::LOC6,
            7 => SCLLOCR::LOC7,
            i => SCLLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == SCLLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == SCLLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == SCLLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == SCLLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == SCLLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == SCLLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == SCLLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == SCLLOCR::LOC7
    }
}
#[doc = "Values that can be written to the field `SDALOC`"]
pub enum SDALOCW {
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
impl SDALOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SDALOCW::LOC0 => 0,
            SDALOCW::LOC1 => 1,
            SDALOCW::LOC2 => 2,
            SDALOCW::LOC3 => 3,
            SDALOCW::LOC4 => 4,
            SDALOCW::LOC5 => 5,
            SDALOCW::LOC6 => 6,
            SDALOCW::LOC7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDALOCW<'a> {
    w: &'a mut W,
}
impl<'a> _SDALOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDALOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(SDALOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(SDALOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(SDALOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(SDALOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(SDALOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(SDALOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(SDALOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(SDALOCW::LOC7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCLLOC`"]
pub enum SCLLOCW {
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
impl SCLLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCLLOCW::LOC0 => 0,
            SCLLOCW::LOC1 => 1,
            SCLLOCW::LOC2 => 2,
            SCLLOCW::LOC3 => 3,
            SCLLOCW::LOC4 => 4,
            SCLLOCW::LOC5 => 5,
            SCLLOCW::LOC6 => 6,
            SCLLOCW::LOC7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCLLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCLLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(SCLLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(SCLLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(SCLLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(SCLLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(SCLLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(SCLLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(SCLLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(SCLLOCW::LOC7)
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
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline]
    pub fn sdaloc(&self) -> SDALOCR {
        SDALOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn sclloc(&self) -> SCLLOCR {
        SCLLOCR::_from({
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
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline]
    pub fn sdaloc(&mut self) -> _SDALOCW {
        _SDALOCW { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn sclloc(&mut self) -> _SCLLOCW {
        _SCLLOCW { w: self }
    }
}
