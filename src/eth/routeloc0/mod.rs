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
#[doc = "Possible values of the field `MIITXLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIITXLOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MIITXLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MIITXLOCR::LOC0 => 0,
            MIITXLOCR::LOC1 => 1,
            MIITXLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MIITXLOCR {
        match value {
            0 => MIITXLOCR::LOC0,
            1 => MIITXLOCR::LOC1,
            i => MIITXLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == MIITXLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == MIITXLOCR::LOC1
    }
}
#[doc = "Possible values of the field `MIIRXLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIIRXLOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MIIRXLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MIIRXLOCR::LOC0 => 0,
            MIIRXLOCR::LOC1 => 1,
            MIIRXLOCR::LOC2 => 2,
            MIIRXLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MIIRXLOCR {
        match value {
            0 => MIIRXLOCR::LOC0,
            1 => MIIRXLOCR::LOC1,
            2 => MIIRXLOCR::LOC2,
            i => MIIRXLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == MIIRXLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == MIIRXLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == MIIRXLOCR::LOC2
    }
}
#[doc = "Possible values of the field `MIICRSLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIICRSLOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MIICRSLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MIICRSLOCR::LOC0 => 0,
            MIICRSLOCR::LOC1 => 1,
            MIICRSLOCR::LOC2 => 2,
            MIICRSLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MIICRSLOCR {
        match value {
            0 => MIICRSLOCR::LOC0,
            1 => MIICRSLOCR::LOC1,
            2 => MIICRSLOCR::LOC2,
            i => MIICRSLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == MIICRSLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == MIICRSLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == MIICRSLOCR::LOC2
    }
}
#[doc = "Possible values of the field `MIICOLLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIICOLLOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MIICOLLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MIICOLLOCR::LOC0 => 0,
            MIICOLLOCR::LOC1 => 1,
            MIICOLLOCR::LOC2 => 2,
            MIICOLLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MIICOLLOCR {
        match value {
            0 => MIICOLLOCR::LOC0,
            1 => MIICOLLOCR::LOC1,
            2 => MIICOLLOCR::LOC2,
            i => MIICOLLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == MIICOLLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == MIICOLLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == MIICOLLOCR::LOC2
    }
}
#[doc = "Values that can be written to the field `MIITXLOC`"]
pub enum MIITXLOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
}
impl MIITXLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MIITXLOCW::LOC0 => 0,
            MIITXLOCW::LOC1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MIITXLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _MIITXLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MIITXLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(MIITXLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(MIITXLOCW::LOC1)
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
#[doc = "Values that can be written to the field `MIIRXLOC`"]
pub enum MIIRXLOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl MIIRXLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MIIRXLOCW::LOC0 => 0,
            MIIRXLOCW::LOC1 => 1,
            MIIRXLOCW::LOC2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MIIRXLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _MIIRXLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MIIRXLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(MIIRXLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(MIIRXLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(MIIRXLOCW::LOC2)
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
#[doc = "Values that can be written to the field `MIICRSLOC`"]
pub enum MIICRSLOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl MIICRSLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MIICRSLOCW::LOC0 => 0,
            MIICRSLOCW::LOC1 => 1,
            MIICRSLOCW::LOC2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MIICRSLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _MIICRSLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MIICRSLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(MIICRSLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(MIICRSLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(MIICRSLOCW::LOC2)
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
#[doc = "Values that can be written to the field `MIICOLLOC`"]
pub enum MIICOLLOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl MIICOLLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MIICOLLOCW::LOC0 => 0,
            MIICOLLOCW::LOC1 => 1,
            MIICOLLOCW::LOC2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MIICOLLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _MIICOLLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MIICOLLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(MIICOLLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(MIICOLLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(MIICOLLOCW::LOC2)
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
    pub fn miitxloc(&self) -> MIITXLOCR {
        MIITXLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn miirxloc(&self) -> MIIRXLOCR {
        MIIRXLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn miicrsloc(&self) -> MIICRSLOCR {
        MIICRSLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn miicolloc(&self) -> MIICOLLOCR {
        MIICOLLOCR::_from({
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
    pub fn miitxloc(&mut self) -> _MIITXLOCW {
        _MIITXLOCW { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn miirxloc(&mut self) -> _MIIRXLOCW {
        _MIIRXLOCW { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn miicrsloc(&mut self) -> _MIICRSLOCW {
        _MIICRSLOCW { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn miicolloc(&mut self) -> _MIICOLLOCW {
        _MIICOLLOCW { w: self }
    }
}
