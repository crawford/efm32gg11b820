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
#[doc = "Possible values of the field `ADLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADLOCR::LOC0 => 0,
            ADLOCR::LOC1 => 1,
            ADLOCR::LOC2 => 2,
            ADLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADLOCR {
        match value {
            0 => ADLOCR::LOC0,
            1 => ADLOCR::LOC1,
            2 => ADLOCR::LOC2,
            i => ADLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == ADLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == ADLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == ADLOCR::LOC2
    }
}
#[doc = "Possible values of the field `ALOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALOCR {
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
impl ALOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALOCR::LOC0 => 0,
            ALOCR::LOC1 => 1,
            ALOCR::LOC2 => 2,
            ALOCR::LOC3 => 3,
            ALOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALOCR {
        match value {
            0 => ALOCR::LOC0,
            1 => ALOCR::LOC1,
            2 => ALOCR::LOC2,
            3 => ALOCR::LOC3,
            i => ALOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == ALOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == ALOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == ALOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == ALOCR::LOC3
    }
}
#[doc = "Possible values of the field `RDYLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDYLOCR {
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RDYLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RDYLOCR::LOC0 => 0,
            RDYLOCR::LOC1 => 1,
            RDYLOCR::LOC2 => 2,
            RDYLOCR::LOC3 => 3,
            RDYLOCR::LOC4 => 4,
            RDYLOCR::LOC5 => 5,
            RDYLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RDYLOCR {
        match value {
            0 => RDYLOCR::LOC0,
            1 => RDYLOCR::LOC1,
            2 => RDYLOCR::LOC2,
            3 => RDYLOCR::LOC3,
            4 => RDYLOCR::LOC4,
            5 => RDYLOCR::LOC5,
            i => RDYLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == RDYLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == RDYLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == RDYLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == RDYLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == RDYLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == RDYLOCR::LOC5
    }
}
#[doc = "Values that can be written to the field `ADLOC`"]
pub enum ADLOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl ADLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADLOCW::LOC0 => 0,
            ADLOCW::LOC1 => 1,
            ADLOCW::LOC2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ADLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ADLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ADLOCW::LOC2)
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
#[doc = "Values that can be written to the field `ALOC`"]
pub enum ALOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
}
impl ALOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ALOCW::LOC0 => 0,
            ALOCW::LOC1 => 1,
            ALOCW::LOC2 => 2,
            ALOCW::LOC3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALOCW<'a> {
    w: &'a mut W,
}
impl<'a> _ALOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ALOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ALOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ALOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ALOCW::LOC3)
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
#[doc = "Values that can be written to the field `RDYLOC`"]
pub enum RDYLOCW {
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
}
impl RDYLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RDYLOCW::LOC0 => 0,
            RDYLOCW::LOC1 => 1,
            RDYLOCW::LOC2 => 2,
            RDYLOCW::LOC3 => 3,
            RDYLOCW::LOC4 => 4,
            RDYLOCW::LOC5 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDYLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _RDYLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDYLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(RDYLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(RDYLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(RDYLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(RDYLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(RDYLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(RDYLOCW::LOC5)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline]
    pub fn adloc(&self) -> ADLOCR {
        ADLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn aloc(&self) -> ALOCR {
        ALOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn rdyloc(&self) -> RDYLOCR {
        RDYLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
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
    pub fn adloc(&mut self) -> _ADLOCW {
        _ADLOCW { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn aloc(&mut self) -> _ALOCW {
        _ALOCW { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn rdyloc(&mut self) -> _RDYLOCW {
        _RDYLOCW { w: self }
    }
}
