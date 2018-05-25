#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCANINPUTSEL1 {
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
#[doc = "Possible values of the field `INPUT32TO39SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT32TO39SELR {
    #[doc = "\"\""]
    APORT1CH0TO7,
    #[doc = "\"\""]
    APORT1CH8TO15,
    #[doc = "\"\""]
    APORT1CH16TO23,
    #[doc = "\"\""]
    APORT1CH24TO31,
    #[doc = "\"\""]
    APORT3CH0TO7,
    #[doc = "\"\""]
    APORT3CH8TO15,
    #[doc = "\"\""]
    APORT3CH16TO23,
    #[doc = "\"\""]
    APORT3CH24TO31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT32TO39SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT32TO39SELR::APORT1CH0TO7 => 4,
            INPUT32TO39SELR::APORT1CH8TO15 => 5,
            INPUT32TO39SELR::APORT1CH16TO23 => 6,
            INPUT32TO39SELR::APORT1CH24TO31 => 7,
            INPUT32TO39SELR::APORT3CH0TO7 => 12,
            INPUT32TO39SELR::APORT3CH8TO15 => 13,
            INPUT32TO39SELR::APORT3CH16TO23 => 14,
            INPUT32TO39SELR::APORT3CH24TO31 => 15,
            INPUT32TO39SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT32TO39SELR {
        match value {
            4 => INPUT32TO39SELR::APORT1CH0TO7,
            5 => INPUT32TO39SELR::APORT1CH8TO15,
            6 => INPUT32TO39SELR::APORT1CH16TO23,
            7 => INPUT32TO39SELR::APORT1CH24TO31,
            12 => INPUT32TO39SELR::APORT3CH0TO7,
            13 => INPUT32TO39SELR::APORT3CH8TO15,
            14 => INPUT32TO39SELR::APORT3CH16TO23,
            15 => INPUT32TO39SELR::APORT3CH24TO31,
            i => INPUT32TO39SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT32TO39SELR::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT32TO39SELR::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT32TO39SELR::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT32TO39SELR::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT32TO39SELR::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT32TO39SELR::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT32TO39SELR::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT32TO39SELR::APORT3CH24TO31
    }
}
#[doc = "Possible values of the field `INPUT40TO47SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT40TO47SELR {
    #[doc = "\"\""]
    APORT1CH0TO7,
    #[doc = "\"\""]
    APORT1CH8TO15,
    #[doc = "\"\""]
    APORT1CH16TO23,
    #[doc = "\"\""]
    APORT1CH24TO31,
    #[doc = "\"\""]
    APORT3CH0TO7,
    #[doc = "\"\""]
    APORT3CH8TO15,
    #[doc = "\"\""]
    APORT3CH16TO23,
    #[doc = "\"\""]
    APORT3CH24TO31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT40TO47SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT40TO47SELR::APORT1CH0TO7 => 4,
            INPUT40TO47SELR::APORT1CH8TO15 => 5,
            INPUT40TO47SELR::APORT1CH16TO23 => 6,
            INPUT40TO47SELR::APORT1CH24TO31 => 7,
            INPUT40TO47SELR::APORT3CH0TO7 => 12,
            INPUT40TO47SELR::APORT3CH8TO15 => 13,
            INPUT40TO47SELR::APORT3CH16TO23 => 14,
            INPUT40TO47SELR::APORT3CH24TO31 => 15,
            INPUT40TO47SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT40TO47SELR {
        match value {
            4 => INPUT40TO47SELR::APORT1CH0TO7,
            5 => INPUT40TO47SELR::APORT1CH8TO15,
            6 => INPUT40TO47SELR::APORT1CH16TO23,
            7 => INPUT40TO47SELR::APORT1CH24TO31,
            12 => INPUT40TO47SELR::APORT3CH0TO7,
            13 => INPUT40TO47SELR::APORT3CH8TO15,
            14 => INPUT40TO47SELR::APORT3CH16TO23,
            15 => INPUT40TO47SELR::APORT3CH24TO31,
            i => INPUT40TO47SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT40TO47SELR::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT40TO47SELR::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT40TO47SELR::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT40TO47SELR::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT40TO47SELR::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT40TO47SELR::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT40TO47SELR::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT40TO47SELR::APORT3CH24TO31
    }
}
#[doc = "Possible values of the field `INPUT48TO55SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT48TO55SELR {
    #[doc = "\"\""]
    APORT1CH0TO7,
    #[doc = "\"\""]
    APORT1CH8TO15,
    #[doc = "\"\""]
    APORT1CH16TO23,
    #[doc = "\"\""]
    APORT1CH24TO31,
    #[doc = "\"\""]
    APORT3CH0TO7,
    #[doc = "\"\""]
    APORT3CH8TO15,
    #[doc = "\"\""]
    APORT3CH16TO23,
    #[doc = "\"\""]
    APORT3CH24TO31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT48TO55SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT48TO55SELR::APORT1CH0TO7 => 4,
            INPUT48TO55SELR::APORT1CH8TO15 => 5,
            INPUT48TO55SELR::APORT1CH16TO23 => 6,
            INPUT48TO55SELR::APORT1CH24TO31 => 7,
            INPUT48TO55SELR::APORT3CH0TO7 => 12,
            INPUT48TO55SELR::APORT3CH8TO15 => 13,
            INPUT48TO55SELR::APORT3CH16TO23 => 14,
            INPUT48TO55SELR::APORT3CH24TO31 => 15,
            INPUT48TO55SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT48TO55SELR {
        match value {
            4 => INPUT48TO55SELR::APORT1CH0TO7,
            5 => INPUT48TO55SELR::APORT1CH8TO15,
            6 => INPUT48TO55SELR::APORT1CH16TO23,
            7 => INPUT48TO55SELR::APORT1CH24TO31,
            12 => INPUT48TO55SELR::APORT3CH0TO7,
            13 => INPUT48TO55SELR::APORT3CH8TO15,
            14 => INPUT48TO55SELR::APORT3CH16TO23,
            15 => INPUT48TO55SELR::APORT3CH24TO31,
            i => INPUT48TO55SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT48TO55SELR::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT48TO55SELR::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT48TO55SELR::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT48TO55SELR::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT48TO55SELR::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT48TO55SELR::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT48TO55SELR::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT48TO55SELR::APORT3CH24TO31
    }
}
#[doc = "Possible values of the field `INPUT56TO63SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT56TO63SELR {
    #[doc = "\"\""]
    APORT1CH0TO7,
    #[doc = "\"\""]
    APORT1CH8TO15,
    #[doc = "\"\""]
    APORT1CH16TO23,
    #[doc = "\"\""]
    APORT1CH24TO31,
    #[doc = "\"\""]
    APORT3CH0TO7,
    #[doc = "\"\""]
    APORT3CH8TO15,
    #[doc = "\"\""]
    APORT3CH16TO23,
    #[doc = "\"\""]
    APORT3CH24TO31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT56TO63SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT56TO63SELR::APORT1CH0TO7 => 4,
            INPUT56TO63SELR::APORT1CH8TO15 => 5,
            INPUT56TO63SELR::APORT1CH16TO23 => 6,
            INPUT56TO63SELR::APORT1CH24TO31 => 7,
            INPUT56TO63SELR::APORT3CH0TO7 => 12,
            INPUT56TO63SELR::APORT3CH8TO15 => 13,
            INPUT56TO63SELR::APORT3CH16TO23 => 14,
            INPUT56TO63SELR::APORT3CH24TO31 => 15,
            INPUT56TO63SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT56TO63SELR {
        match value {
            4 => INPUT56TO63SELR::APORT1CH0TO7,
            5 => INPUT56TO63SELR::APORT1CH8TO15,
            6 => INPUT56TO63SELR::APORT1CH16TO23,
            7 => INPUT56TO63SELR::APORT1CH24TO31,
            12 => INPUT56TO63SELR::APORT3CH0TO7,
            13 => INPUT56TO63SELR::APORT3CH8TO15,
            14 => INPUT56TO63SELR::APORT3CH16TO23,
            15 => INPUT56TO63SELR::APORT3CH24TO31,
            i => INPUT56TO63SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT56TO63SELR::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT56TO63SELR::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT56TO63SELR::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT56TO63SELR::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT56TO63SELR::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT56TO63SELR::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT56TO63SELR::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT56TO63SELR::APORT3CH24TO31
    }
}
#[doc = "Values that can be written to the field `INPUT32TO39SEL`"]
pub enum INPUT32TO39SELW {
    #[doc = "\"\""]
    APORT1CH0TO7,
    #[doc = "\"\""]
    APORT1CH8TO15,
    #[doc = "\"\""]
    APORT1CH16TO23,
    #[doc = "\"\""]
    APORT1CH24TO31,
    #[doc = "\"\""]
    APORT3CH0TO7,
    #[doc = "\"\""]
    APORT3CH8TO15,
    #[doc = "\"\""]
    APORT3CH16TO23,
    #[doc = "\"\""]
    APORT3CH24TO31,
}
impl INPUT32TO39SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT32TO39SELW::APORT1CH0TO7 => 4,
            INPUT32TO39SELW::APORT1CH8TO15 => 5,
            INPUT32TO39SELW::APORT1CH16TO23 => 6,
            INPUT32TO39SELW::APORT1CH24TO31 => 7,
            INPUT32TO39SELW::APORT3CH0TO7 => 12,
            INPUT32TO39SELW::APORT3CH8TO15 => 13,
            INPUT32TO39SELW::APORT3CH16TO23 => 14,
            INPUT32TO39SELW::APORT3CH24TO31 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT32TO39SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT32TO39SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT32TO39SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT32TO39SELW::APORT1CH0TO7)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT32TO39SELW::APORT1CH8TO15)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT32TO39SELW::APORT1CH16TO23)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT32TO39SELW::APORT1CH24TO31)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT32TO39SELW::APORT3CH0TO7)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT32TO39SELW::APORT3CH8TO15)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT32TO39SELW::APORT3CH16TO23)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT32TO39SELW::APORT3CH24TO31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INPUT40TO47SEL`"]
pub enum INPUT40TO47SELW {
    #[doc = "\"\""]
    APORT1CH0TO7,
    #[doc = "\"\""]
    APORT1CH8TO15,
    #[doc = "\"\""]
    APORT1CH16TO23,
    #[doc = "\"\""]
    APORT1CH24TO31,
    #[doc = "\"\""]
    APORT3CH0TO7,
    #[doc = "\"\""]
    APORT3CH8TO15,
    #[doc = "\"\""]
    APORT3CH16TO23,
    #[doc = "\"\""]
    APORT3CH24TO31,
}
impl INPUT40TO47SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT40TO47SELW::APORT1CH0TO7 => 4,
            INPUT40TO47SELW::APORT1CH8TO15 => 5,
            INPUT40TO47SELW::APORT1CH16TO23 => 6,
            INPUT40TO47SELW::APORT1CH24TO31 => 7,
            INPUT40TO47SELW::APORT3CH0TO7 => 12,
            INPUT40TO47SELW::APORT3CH8TO15 => 13,
            INPUT40TO47SELW::APORT3CH16TO23 => 14,
            INPUT40TO47SELW::APORT3CH24TO31 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT40TO47SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT40TO47SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT40TO47SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT40TO47SELW::APORT1CH0TO7)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT40TO47SELW::APORT1CH8TO15)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT40TO47SELW::APORT1CH16TO23)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT40TO47SELW::APORT1CH24TO31)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT40TO47SELW::APORT3CH0TO7)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT40TO47SELW::APORT3CH8TO15)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT40TO47SELW::APORT3CH16TO23)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT40TO47SELW::APORT3CH24TO31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INPUT48TO55SEL`"]
pub enum INPUT48TO55SELW {
    #[doc = "\"\""]
    APORT1CH0TO7,
    #[doc = "\"\""]
    APORT1CH8TO15,
    #[doc = "\"\""]
    APORT1CH16TO23,
    #[doc = "\"\""]
    APORT1CH24TO31,
    #[doc = "\"\""]
    APORT3CH0TO7,
    #[doc = "\"\""]
    APORT3CH8TO15,
    #[doc = "\"\""]
    APORT3CH16TO23,
    #[doc = "\"\""]
    APORT3CH24TO31,
}
impl INPUT48TO55SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT48TO55SELW::APORT1CH0TO7 => 4,
            INPUT48TO55SELW::APORT1CH8TO15 => 5,
            INPUT48TO55SELW::APORT1CH16TO23 => 6,
            INPUT48TO55SELW::APORT1CH24TO31 => 7,
            INPUT48TO55SELW::APORT3CH0TO7 => 12,
            INPUT48TO55SELW::APORT3CH8TO15 => 13,
            INPUT48TO55SELW::APORT3CH16TO23 => 14,
            INPUT48TO55SELW::APORT3CH24TO31 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT48TO55SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT48TO55SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT48TO55SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT48TO55SELW::APORT1CH0TO7)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT48TO55SELW::APORT1CH8TO15)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT48TO55SELW::APORT1CH16TO23)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT48TO55SELW::APORT1CH24TO31)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT48TO55SELW::APORT3CH0TO7)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT48TO55SELW::APORT3CH8TO15)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT48TO55SELW::APORT3CH16TO23)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT48TO55SELW::APORT3CH24TO31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INPUT56TO63SEL`"]
pub enum INPUT56TO63SELW {
    #[doc = "\"\""]
    APORT1CH0TO7,
    #[doc = "\"\""]
    APORT1CH8TO15,
    #[doc = "\"\""]
    APORT1CH16TO23,
    #[doc = "\"\""]
    APORT1CH24TO31,
    #[doc = "\"\""]
    APORT3CH0TO7,
    #[doc = "\"\""]
    APORT3CH8TO15,
    #[doc = "\"\""]
    APORT3CH16TO23,
    #[doc = "\"\""]
    APORT3CH24TO31,
}
impl INPUT56TO63SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT56TO63SELW::APORT1CH0TO7 => 4,
            INPUT56TO63SELW::APORT1CH8TO15 => 5,
            INPUT56TO63SELW::APORT1CH16TO23 => 6,
            INPUT56TO63SELW::APORT1CH24TO31 => 7,
            INPUT56TO63SELW::APORT3CH0TO7 => 12,
            INPUT56TO63SELW::APORT3CH8TO15 => 13,
            INPUT56TO63SELW::APORT3CH16TO23 => 14,
            INPUT56TO63SELW::APORT3CH24TO31 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT56TO63SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT56TO63SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT56TO63SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT56TO63SELW::APORT1CH0TO7)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT56TO63SELW::APORT1CH8TO15)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT56TO63SELW::APORT1CH16TO23)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT56TO63SELW::APORT1CH24TO31)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT56TO63SELW::APORT3CH0TO7)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT56TO63SELW::APORT3CH8TO15)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT56TO63SELW::APORT3CH16TO23)
    }
    #[doc = "\"\""]
    #[inline]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT56TO63SELW::APORT3CH24TO31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - CSEN_INPUT32-39 Select"]
    #[inline]
    pub fn input32to39sel(&self) -> INPUT32TO39SELR {
        INPUT32TO39SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - CSEN_INPUT40-47 Select"]
    #[inline]
    pub fn input40to47sel(&self) -> INPUT40TO47SELR {
        INPUT40TO47SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - CSEN_INPUT48-55 Select"]
    #[inline]
    pub fn input48to55sel(&self) -> INPUT48TO55SELR {
        INPUT48TO55SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - CSEN_INPUT56-63 Select"]
    #[inline]
    pub fn input56to63sel(&self) -> INPUT56TO63SELR {
        INPUT56TO63SELR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - CSEN_INPUT32-39 Select"]
    #[inline]
    pub fn input32to39sel(&mut self) -> _INPUT32TO39SELW {
        _INPUT32TO39SELW { w: self }
    }
    #[doc = "Bits 8:11 - CSEN_INPUT40-47 Select"]
    #[inline]
    pub fn input40to47sel(&mut self) -> _INPUT40TO47SELW {
        _INPUT40TO47SELW { w: self }
    }
    #[doc = "Bits 16:19 - CSEN_INPUT48-55 Select"]
    #[inline]
    pub fn input48to55sel(&mut self) -> _INPUT48TO55SELW {
        _INPUT48TO55SELW { w: self }
    }
    #[doc = "Bits 24:27 - CSEN_INPUT56-63 Select"]
    #[inline]
    pub fn input56to63sel(&mut self) -> _INPUT56TO63SELW {
        _INPUT56TO63SELW { w: self }
    }
}
