#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ROUTELOC1 {
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
#[doc = "Possible values of the field `TSUEXTCLKLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSUEXTCLKLOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSUEXTCLKLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSUEXTCLKLOCR::LOC0 => 0,
            TSUEXTCLKLOCR::LOC1 => 1,
            TSUEXTCLKLOCR::LOC2 => 2,
            TSUEXTCLKLOCR::LOC3 => 3,
            TSUEXTCLKLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSUEXTCLKLOCR {
        match value {
            0 => TSUEXTCLKLOCR::LOC0,
            1 => TSUEXTCLKLOCR::LOC1,
            2 => TSUEXTCLKLOCR::LOC2,
            3 => TSUEXTCLKLOCR::LOC3,
            i => TSUEXTCLKLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == TSUEXTCLKLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == TSUEXTCLKLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == TSUEXTCLKLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == TSUEXTCLKLOCR::LOC3
    }
}
#[doc = "Possible values of the field `TSUTMRTOGLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSUTMRTOGLOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TSUTMRTOGLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSUTMRTOGLOCR::LOC0 => 0,
            TSUTMRTOGLOCR::LOC1 => 1,
            TSUTMRTOGLOCR::LOC2 => 2,
            TSUTMRTOGLOCR::LOC3 => 3,
            TSUTMRTOGLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSUTMRTOGLOCR {
        match value {
            0 => TSUTMRTOGLOCR::LOC0,
            1 => TSUTMRTOGLOCR::LOC1,
            2 => TSUTMRTOGLOCR::LOC2,
            3 => TSUTMRTOGLOCR::LOC3,
            i => TSUTMRTOGLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == TSUTMRTOGLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == TSUTMRTOGLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == TSUTMRTOGLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == TSUTMRTOGLOCR::LOC3
    }
}
#[doc = "Possible values of the field `MDIOLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDIOLOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MDIOLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MDIOLOCR::LOC0 => 0,
            MDIOLOCR::LOC1 => 1,
            MDIOLOCR::LOC2 => 2,
            MDIOLOCR::LOC3 => 3,
            MDIOLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MDIOLOCR {
        match value {
            0 => MDIOLOCR::LOC0,
            1 => MDIOLOCR::LOC1,
            2 => MDIOLOCR::LOC2,
            3 => MDIOLOCR::LOC3,
            i => MDIOLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == MDIOLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == MDIOLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == MDIOLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == MDIOLOCR::LOC3
    }
}
#[doc = "Possible values of the field `RMIILOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMIILOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RMIILOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RMIILOCR::LOC0 => 0,
            RMIILOCR::LOC1 => 1,
            RMIILOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RMIILOCR {
        match value {
            0 => RMIILOCR::LOC0,
            1 => RMIILOCR::LOC1,
            i => RMIILOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == RMIILOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == RMIILOCR::LOC1
    }
}
#[doc = "Values that can be written to the field `TSUEXTCLKLOC`"]
pub enum TSUEXTCLKLOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
}
impl TSUEXTCLKLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSUEXTCLKLOCW::LOC0 => 0,
            TSUEXTCLKLOCW::LOC1 => 1,
            TSUEXTCLKLOCW::LOC2 => 2,
            TSUEXTCLKLOCW::LOC3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSUEXTCLKLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _TSUEXTCLKLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSUEXTCLKLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(TSUEXTCLKLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(TSUEXTCLKLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(TSUEXTCLKLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(TSUEXTCLKLOCW::LOC3)
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
#[doc = "Values that can be written to the field `TSUTMRTOGLOC`"]
pub enum TSUTMRTOGLOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
}
impl TSUTMRTOGLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSUTMRTOGLOCW::LOC0 => 0,
            TSUTMRTOGLOCW::LOC1 => 1,
            TSUTMRTOGLOCW::LOC2 => 2,
            TSUTMRTOGLOCW::LOC3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSUTMRTOGLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _TSUTMRTOGLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSUTMRTOGLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(TSUTMRTOGLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(TSUTMRTOGLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(TSUTMRTOGLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(TSUTMRTOGLOCW::LOC3)
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
#[doc = "Values that can be written to the field `MDIOLOC`"]
pub enum MDIOLOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
}
impl MDIOLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MDIOLOCW::LOC0 => 0,
            MDIOLOCW::LOC1 => 1,
            MDIOLOCW::LOC2 => 2,
            MDIOLOCW::LOC3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDIOLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _MDIOLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDIOLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(MDIOLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(MDIOLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(MDIOLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(MDIOLOCW::LOC3)
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
#[doc = "Values that can be written to the field `RMIILOC`"]
pub enum RMIILOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
}
impl RMIILOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RMIILOCW::LOC0 => 0,
            RMIILOCW::LOC1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMIILOCW<'a> {
    w: &'a mut W,
}
impl<'a> _RMIILOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMIILOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(RMIILOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(RMIILOCW::LOC1)
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
    pub fn tsuextclkloc(&self) -> TSUEXTCLKLOCR {
        TSUEXTCLKLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn tsutmrtogloc(&self) -> TSUTMRTOGLOCR {
        TSUTMRTOGLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn mdioloc(&self) -> MDIOLOCR {
        MDIOLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn rmiiloc(&self) -> RMIILOCR {
        RMIILOCR::_from({
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
    pub fn tsuextclkloc(&mut self) -> _TSUEXTCLKLOCW {
        _TSUEXTCLKLOCW { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn tsutmrtogloc(&mut self) -> _TSUTMRTOGLOCW {
        _TSUTMRTOGLOCW { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn mdioloc(&mut self) -> _MDIOLOCW {
        _MDIOLOCW { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn rmiiloc(&mut self) -> _RMIILOCW {
        _RMIILOCW { w: self }
    }
}
