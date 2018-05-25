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
#[doc = "Possible values of the field `CMDLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDLOCR {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMDLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDLOCR::LOC0 => 0,
            CMDLOCR::LOC1 => 1,
            CMDLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDLOCR {
        match value {
            0 => CMDLOCR::LOC0,
            1 => CMDLOCR::LOC1,
            i => CMDLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CMDLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CMDLOCR::LOC1
    }
}
#[doc = "Values that can be written to the field `CMDLOC`"]
pub enum CMDLOCW {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
}
impl CMDLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDLOCW::LOC0 => 0,
            CMDLOCW::LOC1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CMDLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CMDLOCW::LOC1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - I/O Location for CMD Pin"]
    #[inline]
    pub fn cmdloc(&self) -> CMDLOCR {
        CMDLOCR::_from({
            const MASK: u8 = 63;
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
    #[doc = "Bits 0:5 - I/O Location for CMD Pin"]
    #[inline]
    pub fn cmdloc(&mut self) -> _CMDLOCW {
        _CMDLOCW { w: self }
    }
}
