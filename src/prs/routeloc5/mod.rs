#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROUTELOC5 {
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
#[doc = "Possible values of the field `CH20LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH20LOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CH20LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH20LOCR::LOC0 => 0,
            CH20LOCR::LOC1 => 1,
            CH20LOCR::LOC2 => 2,
            CH20LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH20LOCR {
        match value {
            0 => CH20LOCR::LOC0,
            1 => CH20LOCR::LOC1,
            2 => CH20LOCR::LOC2,
            i => CH20LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CH20LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CH20LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CH20LOCR::LOC2
    }
}
#[doc = "Possible values of the field `CH21LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH21LOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CH21LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH21LOCR::LOC0 => 0,
            CH21LOCR::LOC1 => 1,
            CH21LOCR::LOC2 => 2,
            CH21LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH21LOCR {
        match value {
            0 => CH21LOCR::LOC0,
            1 => CH21LOCR::LOC1,
            2 => CH21LOCR::LOC2,
            i => CH21LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CH21LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CH21LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CH21LOCR::LOC2
    }
}
#[doc = "Possible values of the field `CH22LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH22LOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CH22LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH22LOCR::LOC0 => 0,
            CH22LOCR::LOC1 => 1,
            CH22LOCR::LOC2 => 2,
            CH22LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH22LOCR {
        match value {
            0 => CH22LOCR::LOC0,
            1 => CH22LOCR::LOC1,
            2 => CH22LOCR::LOC2,
            i => CH22LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CH22LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CH22LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CH22LOCR::LOC2
    }
}
#[doc = "Possible values of the field `CH23LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH23LOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CH23LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH23LOCR::LOC0 => 0,
            CH23LOCR::LOC1 => 1,
            CH23LOCR::LOC2 => 2,
            CH23LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH23LOCR {
        match value {
            0 => CH23LOCR::LOC0,
            1 => CH23LOCR::LOC1,
            2 => CH23LOCR::LOC2,
            i => CH23LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CH23LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CH23LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CH23LOCR::LOC2
    }
}
#[doc = "Values that can be written to the field `CH20LOC`"]
pub enum CH20LOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl CH20LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH20LOCW::LOC0 => 0,
            CH20LOCW::LOC1 => 1,
            CH20LOCW::LOC2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH20LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH20LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH20LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH20LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH20LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH20LOCW::LOC2)
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
#[doc = "Values that can be written to the field `CH21LOC`"]
pub enum CH21LOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl CH21LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH21LOCW::LOC0 => 0,
            CH21LOCW::LOC1 => 1,
            CH21LOCW::LOC2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH21LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH21LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH21LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH21LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH21LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH21LOCW::LOC2)
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
#[doc = "Values that can be written to the field `CH22LOC`"]
pub enum CH22LOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl CH22LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH22LOCW::LOC0 => 0,
            CH22LOCW::LOC1 => 1,
            CH22LOCW::LOC2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH22LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH22LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH22LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH22LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH22LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH22LOCW::LOC2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH23LOC`"]
pub enum CH23LOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl CH23LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH23LOCW::LOC0 => 0,
            CH23LOCW::LOC1 => 1,
            CH23LOCW::LOC2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH23LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH23LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH23LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH23LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH23LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH23LOCW::LOC2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 24;
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
    pub fn ch20loc(&self) -> CH20LOCR {
        CH20LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn ch21loc(&self) -> CH21LOCR {
        CH21LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn ch22loc(&self) -> CH22LOCR {
        CH22LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn ch23loc(&self) -> CH23LOCR {
        CH23LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
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
    pub fn ch20loc(&mut self) -> _CH20LOCW {
        _CH20LOCW { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn ch21loc(&mut self) -> _CH21LOCW {
        _CH21LOCW { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn ch22loc(&mut self) -> _CH22LOCW {
        _CH22LOCW { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn ch23loc(&mut self) -> _CH23LOCW {
        _CH23LOCW { w: self }
    }
}
