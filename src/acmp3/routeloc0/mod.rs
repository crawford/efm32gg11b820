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
#[doc = "Possible values of the field `OUTLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTLOCR {
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
impl OUTLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OUTLOCR::LOC0 => 0,
            OUTLOCR::LOC1 => 1,
            OUTLOCR::LOC2 => 2,
            OUTLOCR::LOC3 => 3,
            OUTLOCR::LOC4 => 4,
            OUTLOCR::LOC5 => 5,
            OUTLOCR::LOC6 => 6,
            OUTLOCR::LOC7 => 7,
            OUTLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OUTLOCR {
        match value {
            0 => OUTLOCR::LOC0,
            1 => OUTLOCR::LOC1,
            2 => OUTLOCR::LOC2,
            3 => OUTLOCR::LOC3,
            4 => OUTLOCR::LOC4,
            5 => OUTLOCR::LOC5,
            6 => OUTLOCR::LOC6,
            7 => OUTLOCR::LOC7,
            i => OUTLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == OUTLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == OUTLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == OUTLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == OUTLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == OUTLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == OUTLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == OUTLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == OUTLOCR::LOC7
    }
}
#[doc = "Values that can be written to the field `OUTLOC`"]
pub enum OUTLOCW {
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
impl OUTLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUTLOCW::LOC0 => 0,
            OUTLOCW::LOC1 => 1,
            OUTLOCW::LOC2 => 2,
            OUTLOCW::LOC3 => 3,
            OUTLOCW::LOC4 => 4,
            OUTLOCW::LOC5 => 5,
            OUTLOCW::LOC6 => 6,
            OUTLOCW::LOC7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OUTLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _OUTLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(OUTLOCW::LOC7)
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
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline]
    pub fn outloc(&self) -> OUTLOCR {
        OUTLOCR::_from({
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
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline]
    pub fn outloc(&mut self) -> _OUTLOCW {
        _OUTLOCW { w: self }
    }
}
