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
#[doc = "Possible values of the field `CC0LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC0LOCR {
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
impl CC0LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC0LOCR::LOC0 => 0,
            CC0LOCR::LOC1 => 1,
            CC0LOCR::LOC2 => 2,
            CC0LOCR::LOC3 => 3,
            CC0LOCR::LOC4 => 4,
            CC0LOCR::LOC5 => 5,
            CC0LOCR::LOC6 => 6,
            CC0LOCR::LOC7 => 7,
            CC0LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC0LOCR {
        match value {
            0 => CC0LOCR::LOC0,
            1 => CC0LOCR::LOC1,
            2 => CC0LOCR::LOC2,
            3 => CC0LOCR::LOC3,
            4 => CC0LOCR::LOC4,
            5 => CC0LOCR::LOC5,
            6 => CC0LOCR::LOC6,
            7 => CC0LOCR::LOC7,
            i => CC0LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CC0LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CC0LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CC0LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CC0LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CC0LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CC0LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CC0LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CC0LOCR::LOC7
    }
}
#[doc = "Possible values of the field `CC1LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1LOCR {
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
impl CC1LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC1LOCR::LOC0 => 0,
            CC1LOCR::LOC1 => 1,
            CC1LOCR::LOC2 => 2,
            CC1LOCR::LOC3 => 3,
            CC1LOCR::LOC4 => 4,
            CC1LOCR::LOC5 => 5,
            CC1LOCR::LOC6 => 6,
            CC1LOCR::LOC7 => 7,
            CC1LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC1LOCR {
        match value {
            0 => CC1LOCR::LOC0,
            1 => CC1LOCR::LOC1,
            2 => CC1LOCR::LOC2,
            3 => CC1LOCR::LOC3,
            4 => CC1LOCR::LOC4,
            5 => CC1LOCR::LOC5,
            6 => CC1LOCR::LOC6,
            7 => CC1LOCR::LOC7,
            i => CC1LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CC1LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CC1LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CC1LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CC1LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CC1LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CC1LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CC1LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CC1LOCR::LOC7
    }
}
#[doc = "Possible values of the field `CC2LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2LOCR {
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
impl CC2LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC2LOCR::LOC0 => 0,
            CC2LOCR::LOC1 => 1,
            CC2LOCR::LOC2 => 2,
            CC2LOCR::LOC3 => 3,
            CC2LOCR::LOC4 => 4,
            CC2LOCR::LOC5 => 5,
            CC2LOCR::LOC6 => 6,
            CC2LOCR::LOC7 => 7,
            CC2LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC2LOCR {
        match value {
            0 => CC2LOCR::LOC0,
            1 => CC2LOCR::LOC1,
            2 => CC2LOCR::LOC2,
            3 => CC2LOCR::LOC3,
            4 => CC2LOCR::LOC4,
            5 => CC2LOCR::LOC5,
            6 => CC2LOCR::LOC6,
            7 => CC2LOCR::LOC7,
            i => CC2LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CC2LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CC2LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CC2LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CC2LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CC2LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CC2LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CC2LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CC2LOCR::LOC7
    }
}
#[doc = "Possible values of the field `CC3LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3LOCR {
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
impl CC3LOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CC3LOCR::LOC0 => 0,
            CC3LOCR::LOC1 => 1,
            CC3LOCR::LOC2 => 2,
            CC3LOCR::LOC3 => 3,
            CC3LOCR::LOC4 => 4,
            CC3LOCR::LOC5 => 5,
            CC3LOCR::LOC6 => 6,
            CC3LOCR::LOC7 => 7,
            CC3LOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CC3LOCR {
        match value {
            0 => CC3LOCR::LOC0,
            1 => CC3LOCR::LOC1,
            2 => CC3LOCR::LOC2,
            3 => CC3LOCR::LOC3,
            4 => CC3LOCR::LOC4,
            5 => CC3LOCR::LOC5,
            6 => CC3LOCR::LOC6,
            7 => CC3LOCR::LOC7,
            i => CC3LOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline]
    pub fn is_loc0(&self) -> bool {
        *self == CC3LOCR::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline]
    pub fn is_loc1(&self) -> bool {
        *self == CC3LOCR::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline]
    pub fn is_loc2(&self) -> bool {
        *self == CC3LOCR::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline]
    pub fn is_loc3(&self) -> bool {
        *self == CC3LOCR::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline]
    pub fn is_loc4(&self) -> bool {
        *self == CC3LOCR::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline]
    pub fn is_loc5(&self) -> bool {
        *self == CC3LOCR::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline]
    pub fn is_loc6(&self) -> bool {
        *self == CC3LOCR::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline]
    pub fn is_loc7(&self) -> bool {
        *self == CC3LOCR::LOC7
    }
}
#[doc = "Values that can be written to the field `CC0LOC`"]
pub enum CC0LOCW {
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
impl CC0LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC0LOCW::LOC0 => 0,
            CC0LOCW::LOC1 => 1,
            CC0LOCW::LOC2 => 2,
            CC0LOCW::LOC3 => 3,
            CC0LOCW::LOC4 => 4,
            CC0LOCW::LOC5 => 5,
            CC0LOCW::LOC6 => 6,
            CC0LOCW::LOC7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC0LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CC0LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC0LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CC0LOCW::LOC7)
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
#[doc = "Values that can be written to the field `CC1LOC`"]
pub enum CC1LOCW {
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
impl CC1LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC1LOCW::LOC0 => 0,
            CC1LOCW::LOC1 => 1,
            CC1LOCW::LOC2 => 2,
            CC1LOCW::LOC3 => 3,
            CC1LOCW::LOC4 => 4,
            CC1LOCW::LOC5 => 5,
            CC1LOCW::LOC6 => 6,
            CC1LOCW::LOC7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC1LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC1LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CC1LOCW::LOC7)
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
#[doc = "Values that can be written to the field `CC2LOC`"]
pub enum CC2LOCW {
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
impl CC2LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC2LOCW::LOC0 => 0,
            CC2LOCW::LOC1 => 1,
            CC2LOCW::LOC2 => 2,
            CC2LOCW::LOC3 => 3,
            CC2LOCW::LOC4 => 4,
            CC2LOCW::LOC5 => 5,
            CC2LOCW::LOC6 => 6,
            CC2LOCW::LOC7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC2LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CC2LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC2LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CC2LOCW::LOC7)
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
#[doc = "Values that can be written to the field `CC3LOC`"]
pub enum CC3LOCW {
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
impl CC3LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CC3LOCW::LOC0 => 0,
            CC3LOCW::LOC1 => 1,
            CC3LOCW::LOC2 => 2,
            CC3LOCW::LOC3 => 3,
            CC3LOCW::LOC4 => 4,
            CC3LOCW::LOC5 => 5,
            CC3LOCW::LOC6 => 6,
            CC3LOCW::LOC7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CC3LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CC3LOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC0)
    }
    #[doc = "Location 1"]
    #[inline]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC1)
    }
    #[doc = "Location 2"]
    #[inline]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC2)
    }
    #[doc = "Location 3"]
    #[inline]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC3)
    }
    #[doc = "Location 4"]
    #[inline]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC4)
    }
    #[doc = "Location 5"]
    #[inline]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC5)
    }
    #[doc = "Location 6"]
    #[inline]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC6)
    }
    #[doc = "Location 7"]
    #[inline]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CC3LOCW::LOC7)
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
    pub fn cc0loc(&self) -> CC0LOCR {
        CC0LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn cc1loc(&self) -> CC1LOCR {
        CC1LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn cc2loc(&self) -> CC2LOCR {
        CC2LOCR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn cc3loc(&self) -> CC3LOCR {
        CC3LOCR::_from({
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
    pub fn cc0loc(&mut self) -> _CC0LOCW {
        _CC0LOCW { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline]
    pub fn cc1loc(&mut self) -> _CC1LOCW {
        _CC1LOCW { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline]
    pub fn cc2loc(&mut self) -> _CC2LOCW {
        _CC2LOCW { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline]
    pub fn cc3loc(&mut self) -> _CC3LOCW {
        _CC3LOCW { w: self }
    }
}
