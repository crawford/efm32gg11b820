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
#[doc = "Possible values of the field `S0INLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0INLOCR {
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
impl S0INLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S0INLOCR::LOC0 => 0,
            S0INLOCR::LOC1 => 1,
            S0INLOCR::LOC2 => 2,
            S0INLOCR::LOC3 => 3,
            S0INLOCR::LOC4 => 4,
            S0INLOCR::LOC5 => 5,
            S0INLOCR::LOC6 => 6,
            S0INLOCR::LOC7 => 7,
            S0INLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S0INLOCR {
        match value {
            0 => S0INLOCR::LOC0,
            1 => S0INLOCR::LOC1,
            2 => S0INLOCR::LOC2,
            3 => S0INLOCR::LOC3,
            4 => S0INLOCR::LOC4,
            5 => S0INLOCR::LOC5,
            6 => S0INLOCR::LOC6,
            7 => S0INLOCR::LOC7,
            i => S0INLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == S0INLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == S0INLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == S0INLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == S0INLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == S0INLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == S0INLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == S0INLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == S0INLOCR::LOC7
    }
}
#[doc = "Possible values of the field `S1INLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1INLOCR {
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
impl S1INLOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            S1INLOCR::LOC0 => 0,
            S1INLOCR::LOC1 => 1,
            S1INLOCR::LOC2 => 2,
            S1INLOCR::LOC3 => 3,
            S1INLOCR::LOC4 => 4,
            S1INLOCR::LOC5 => 5,
            S1INLOCR::LOC6 => 6,
            S1INLOCR::LOC7 => 7,
            S1INLOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> S1INLOCR {
        match value {
            0 => S1INLOCR::LOC0,
            1 => S1INLOCR::LOC1,
            2 => S1INLOCR::LOC2,
            3 => S1INLOCR::LOC3,
            4 => S1INLOCR::LOC4,
            5 => S1INLOCR::LOC5,
            6 => S1INLOCR::LOC6,
            7 => S1INLOCR::LOC7,
            i => S1INLOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == S1INLOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == S1INLOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == S1INLOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == S1INLOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == S1INLOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == S1INLOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == S1INLOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == S1INLOCR::LOC7
    }
}
#[doc = "Values that can be written to the field `S0INLOC`"]
pub enum S0INLOCW {
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
impl S0INLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S0INLOCW::LOC0 => 0,
            S0INLOCW::LOC1 => 1,
            S0INLOCW::LOC2 => 2,
            S0INLOCW::LOC3 => 3,
            S0INLOCW::LOC4 => 4,
            S0INLOCW::LOC5 => 5,
            S0INLOCW::LOC6 => 6,
            S0INLOCW::LOC7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0INLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _S0INLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0INLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(S0INLOCW::LOC7)
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
#[doc = "Values that can be written to the field `S1INLOC`"]
pub enum S1INLOCW {
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
impl S1INLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            S1INLOCW::LOC0 => 0,
            S1INLOCW::LOC1 => 1,
            S1INLOCW::LOC2 => 2,
            S1INLOCW::LOC3 => 3,
            S1INLOCW::LOC4 => 4,
            S1INLOCW::LOC5 => 5,
            S1INLOCW::LOC6 => 6,
            S1INLOCW::LOC7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1INLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _S1INLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1INLOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(S1INLOCW::LOC7)
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
    pub fn s0inloc(&self) -> S0INLOCR {
        S0INLOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn s1inloc(&self) -> S1INLOCR {
        S1INLOCR::_from({
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
    pub fn s0inloc(&mut self) -> _S0INLOCW {
        _S0INLOCW { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn s1inloc(&mut self) -> _S1INLOCW {
        _S1INLOCW { w: self }
    }
}
