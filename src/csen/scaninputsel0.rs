#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCANINPUTSEL0 {
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
#[doc = "Possible values of the field `INPUT0TO7SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT0TO7SELR {
    #[doc = "undocumented"]
    APORT1CH0TO7,
    #[doc = "undocumented"]
    APORT1CH8TO15,
    #[doc = "undocumented"]
    APORT1CH16TO23,
    #[doc = "undocumented"]
    APORT1CH24TO31,
    #[doc = "undocumented"]
    APORT3CH0TO7,
    #[doc = "undocumented"]
    APORT3CH8TO15,
    #[doc = "undocumented"]
    APORT3CH16TO23,
    #[doc = "undocumented"]
    APORT3CH24TO31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT0TO7SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT0TO7SELR::APORT1CH0TO7 => 4,
            INPUT0TO7SELR::APORT1CH8TO15 => 5,
            INPUT0TO7SELR::APORT1CH16TO23 => 6,
            INPUT0TO7SELR::APORT1CH24TO31 => 7,
            INPUT0TO7SELR::APORT3CH0TO7 => 12,
            INPUT0TO7SELR::APORT3CH8TO15 => 13,
            INPUT0TO7SELR::APORT3CH16TO23 => 14,
            INPUT0TO7SELR::APORT3CH24TO31 => 15,
            INPUT0TO7SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT0TO7SELR {
        match value {
            4 => INPUT0TO7SELR::APORT1CH0TO7,
            5 => INPUT0TO7SELR::APORT1CH8TO15,
            6 => INPUT0TO7SELR::APORT1CH16TO23,
            7 => INPUT0TO7SELR::APORT1CH24TO31,
            12 => INPUT0TO7SELR::APORT3CH0TO7,
            13 => INPUT0TO7SELR::APORT3CH8TO15,
            14 => INPUT0TO7SELR::APORT3CH16TO23,
            15 => INPUT0TO7SELR::APORT3CH24TO31,
            i => INPUT0TO7SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT0TO7SELR::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT0TO7SELR::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT0TO7SELR::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT0TO7SELR::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT0TO7SELR::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT0TO7SELR::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT0TO7SELR::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT0TO7SELR::APORT3CH24TO31
    }
}
#[doc = "Possible values of the field `INPUT8TO15SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT8TO15SELR {
    #[doc = "undocumented"]
    APORT1CH0TO7,
    #[doc = "undocumented"]
    APORT1CH8TO15,
    #[doc = "undocumented"]
    APORT1CH16TO23,
    #[doc = "undocumented"]
    APORT1CH24TO31,
    #[doc = "undocumented"]
    APORT3CH0TO7,
    #[doc = "undocumented"]
    APORT3CH8TO15,
    #[doc = "undocumented"]
    APORT3CH16TO23,
    #[doc = "undocumented"]
    APORT3CH24TO31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT8TO15SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT8TO15SELR::APORT1CH0TO7 => 4,
            INPUT8TO15SELR::APORT1CH8TO15 => 5,
            INPUT8TO15SELR::APORT1CH16TO23 => 6,
            INPUT8TO15SELR::APORT1CH24TO31 => 7,
            INPUT8TO15SELR::APORT3CH0TO7 => 12,
            INPUT8TO15SELR::APORT3CH8TO15 => 13,
            INPUT8TO15SELR::APORT3CH16TO23 => 14,
            INPUT8TO15SELR::APORT3CH24TO31 => 15,
            INPUT8TO15SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT8TO15SELR {
        match value {
            4 => INPUT8TO15SELR::APORT1CH0TO7,
            5 => INPUT8TO15SELR::APORT1CH8TO15,
            6 => INPUT8TO15SELR::APORT1CH16TO23,
            7 => INPUT8TO15SELR::APORT1CH24TO31,
            12 => INPUT8TO15SELR::APORT3CH0TO7,
            13 => INPUT8TO15SELR::APORT3CH8TO15,
            14 => INPUT8TO15SELR::APORT3CH16TO23,
            15 => INPUT8TO15SELR::APORT3CH24TO31,
            i => INPUT8TO15SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT8TO15SELR::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT8TO15SELR::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT8TO15SELR::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT8TO15SELR::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT8TO15SELR::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT8TO15SELR::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT8TO15SELR::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT8TO15SELR::APORT3CH24TO31
    }
}
#[doc = "Possible values of the field `INPUT16TO23SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT16TO23SELR {
    #[doc = "undocumented"]
    APORT1CH0TO7,
    #[doc = "undocumented"]
    APORT1CH8TO15,
    #[doc = "undocumented"]
    APORT1CH16TO23,
    #[doc = "undocumented"]
    APORT1CH24TO31,
    #[doc = "undocumented"]
    APORT3CH0TO7,
    #[doc = "undocumented"]
    APORT3CH8TO15,
    #[doc = "undocumented"]
    APORT3CH16TO23,
    #[doc = "undocumented"]
    APORT3CH24TO31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT16TO23SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT16TO23SELR::APORT1CH0TO7 => 4,
            INPUT16TO23SELR::APORT1CH8TO15 => 5,
            INPUT16TO23SELR::APORT1CH16TO23 => 6,
            INPUT16TO23SELR::APORT1CH24TO31 => 7,
            INPUT16TO23SELR::APORT3CH0TO7 => 12,
            INPUT16TO23SELR::APORT3CH8TO15 => 13,
            INPUT16TO23SELR::APORT3CH16TO23 => 14,
            INPUT16TO23SELR::APORT3CH24TO31 => 15,
            INPUT16TO23SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT16TO23SELR {
        match value {
            4 => INPUT16TO23SELR::APORT1CH0TO7,
            5 => INPUT16TO23SELR::APORT1CH8TO15,
            6 => INPUT16TO23SELR::APORT1CH16TO23,
            7 => INPUT16TO23SELR::APORT1CH24TO31,
            12 => INPUT16TO23SELR::APORT3CH0TO7,
            13 => INPUT16TO23SELR::APORT3CH8TO15,
            14 => INPUT16TO23SELR::APORT3CH16TO23,
            15 => INPUT16TO23SELR::APORT3CH24TO31,
            i => INPUT16TO23SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT16TO23SELR::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT16TO23SELR::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT16TO23SELR::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT16TO23SELR::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT16TO23SELR::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT16TO23SELR::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT16TO23SELR::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT16TO23SELR::APORT3CH24TO31
    }
}
#[doc = "Possible values of the field `INPUT24TO31SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT24TO31SELR {
    #[doc = "undocumented"]
    APORT1CH0TO7,
    #[doc = "undocumented"]
    APORT1CH8TO15,
    #[doc = "undocumented"]
    APORT1CH16TO23,
    #[doc = "undocumented"]
    APORT1CH24TO31,
    #[doc = "undocumented"]
    APORT3CH0TO7,
    #[doc = "undocumented"]
    APORT3CH8TO15,
    #[doc = "undocumented"]
    APORT3CH16TO23,
    #[doc = "undocumented"]
    APORT3CH24TO31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUT24TO31SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUT24TO31SELR::APORT1CH0TO7 => 4,
            INPUT24TO31SELR::APORT1CH8TO15 => 5,
            INPUT24TO31SELR::APORT1CH16TO23 => 6,
            INPUT24TO31SELR::APORT1CH24TO31 => 7,
            INPUT24TO31SELR::APORT3CH0TO7 => 12,
            INPUT24TO31SELR::APORT3CH8TO15 => 13,
            INPUT24TO31SELR::APORT3CH16TO23 => 14,
            INPUT24TO31SELR::APORT3CH24TO31 => 15,
            INPUT24TO31SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUT24TO31SELR {
        match value {
            4 => INPUT24TO31SELR::APORT1CH0TO7,
            5 => INPUT24TO31SELR::APORT1CH8TO15,
            6 => INPUT24TO31SELR::APORT1CH16TO23,
            7 => INPUT24TO31SELR::APORT1CH24TO31,
            12 => INPUT24TO31SELR::APORT3CH0TO7,
            13 => INPUT24TO31SELR::APORT3CH8TO15,
            14 => INPUT24TO31SELR::APORT3CH16TO23,
            15 => INPUT24TO31SELR::APORT3CH24TO31,
            i => INPUT24TO31SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT24TO31SELR::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT24TO31SELR::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT24TO31SELR::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT24TO31SELR::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT24TO31SELR::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT24TO31SELR::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT24TO31SELR::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT24TO31SELR::APORT3CH24TO31
    }
}
#[doc = "Values that can be written to the field `INPUT0TO7SEL`"]
pub enum INPUT0TO7SELW {
    #[doc = "`100`"]
    APORT1CH0TO7,
    #[doc = "`101`"]
    APORT1CH8TO15,
    #[doc = "`110`"]
    APORT1CH16TO23,
    #[doc = "`111`"]
    APORT1CH24TO31,
    #[doc = "`1100`"]
    APORT3CH0TO7,
    #[doc = "`1101`"]
    APORT3CH8TO15,
    #[doc = "`1110`"]
    APORT3CH16TO23,
    #[doc = "`1111`"]
    APORT3CH24TO31,
}
impl INPUT0TO7SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT0TO7SELW::APORT1CH0TO7 => 4,
            INPUT0TO7SELW::APORT1CH8TO15 => 5,
            INPUT0TO7SELW::APORT1CH16TO23 => 6,
            INPUT0TO7SELW::APORT1CH24TO31 => 7,
            INPUT0TO7SELW::APORT3CH0TO7 => 12,
            INPUT0TO7SELW::APORT3CH8TO15 => 13,
            INPUT0TO7SELW::APORT3CH16TO23 => 14,
            INPUT0TO7SELW::APORT3CH24TO31 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT0TO7SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT0TO7SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT0TO7SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`100`"]
    #[inline]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT1CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SELW::APORT3CH24TO31)
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
#[doc = "Values that can be written to the field `INPUT8TO15SEL`"]
pub enum INPUT8TO15SELW {
    #[doc = "`100`"]
    APORT1CH0TO7,
    #[doc = "`101`"]
    APORT1CH8TO15,
    #[doc = "`110`"]
    APORT1CH16TO23,
    #[doc = "`111`"]
    APORT1CH24TO31,
    #[doc = "`1100`"]
    APORT3CH0TO7,
    #[doc = "`1101`"]
    APORT3CH8TO15,
    #[doc = "`1110`"]
    APORT3CH16TO23,
    #[doc = "`1111`"]
    APORT3CH24TO31,
}
impl INPUT8TO15SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT8TO15SELW::APORT1CH0TO7 => 4,
            INPUT8TO15SELW::APORT1CH8TO15 => 5,
            INPUT8TO15SELW::APORT1CH16TO23 => 6,
            INPUT8TO15SELW::APORT1CH24TO31 => 7,
            INPUT8TO15SELW::APORT3CH0TO7 => 12,
            INPUT8TO15SELW::APORT3CH8TO15 => 13,
            INPUT8TO15SELW::APORT3CH16TO23 => 14,
            INPUT8TO15SELW::APORT3CH24TO31 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT8TO15SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT8TO15SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT8TO15SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`100`"]
    #[inline]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT1CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SELW::APORT3CH24TO31)
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
#[doc = "Values that can be written to the field `INPUT16TO23SEL`"]
pub enum INPUT16TO23SELW {
    #[doc = "`100`"]
    APORT1CH0TO7,
    #[doc = "`101`"]
    APORT1CH8TO15,
    #[doc = "`110`"]
    APORT1CH16TO23,
    #[doc = "`111`"]
    APORT1CH24TO31,
    #[doc = "`1100`"]
    APORT3CH0TO7,
    #[doc = "`1101`"]
    APORT3CH8TO15,
    #[doc = "`1110`"]
    APORT3CH16TO23,
    #[doc = "`1111`"]
    APORT3CH24TO31,
}
impl INPUT16TO23SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT16TO23SELW::APORT1CH0TO7 => 4,
            INPUT16TO23SELW::APORT1CH8TO15 => 5,
            INPUT16TO23SELW::APORT1CH16TO23 => 6,
            INPUT16TO23SELW::APORT1CH24TO31 => 7,
            INPUT16TO23SELW::APORT3CH0TO7 => 12,
            INPUT16TO23SELW::APORT3CH8TO15 => 13,
            INPUT16TO23SELW::APORT3CH16TO23 => 14,
            INPUT16TO23SELW::APORT3CH24TO31 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT16TO23SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT16TO23SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT16TO23SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`100`"]
    #[inline]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT1CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SELW::APORT3CH24TO31)
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
#[doc = "Values that can be written to the field `INPUT24TO31SEL`"]
pub enum INPUT24TO31SELW {
    #[doc = "`100`"]
    APORT1CH0TO7,
    #[doc = "`101`"]
    APORT1CH8TO15,
    #[doc = "`110`"]
    APORT1CH16TO23,
    #[doc = "`111`"]
    APORT1CH24TO31,
    #[doc = "`1100`"]
    APORT3CH0TO7,
    #[doc = "`1101`"]
    APORT3CH8TO15,
    #[doc = "`1110`"]
    APORT3CH16TO23,
    #[doc = "`1111`"]
    APORT3CH24TO31,
}
impl INPUT24TO31SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUT24TO31SELW::APORT1CH0TO7 => 4,
            INPUT24TO31SELW::APORT1CH8TO15 => 5,
            INPUT24TO31SELW::APORT1CH16TO23 => 6,
            INPUT24TO31SELW::APORT1CH24TO31 => 7,
            INPUT24TO31SELW::APORT3CH0TO7 => 12,
            INPUT24TO31SELW::APORT3CH8TO15 => 13,
            INPUT24TO31SELW::APORT3CH16TO23 => 14,
            INPUT24TO31SELW::APORT3CH24TO31 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUT24TO31SELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUT24TO31SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUT24TO31SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`100`"]
    #[inline]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT1CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SELW::APORT3CH24TO31)
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
    #[doc = "Bits 0:3 - CSEN_INPUT0-7 Select"]
    #[inline]
    pub fn input0to7sel(&self) -> INPUT0TO7SELR {
        INPUT0TO7SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - CSEN_INPUT8-15 Select"]
    #[inline]
    pub fn input8to15sel(&self) -> INPUT8TO15SELR {
        INPUT8TO15SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - CSEN_INPUT16-23 Select"]
    #[inline]
    pub fn input16to23sel(&self) -> INPUT16TO23SELR {
        INPUT16TO23SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - CSEN_INPUT24-31 Select"]
    #[inline]
    pub fn input24to31sel(&self) -> INPUT24TO31SELR {
        INPUT24TO31SELR::_from({
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
    #[doc = "Bits 0:3 - CSEN_INPUT0-7 Select"]
    #[inline]
    pub fn input0to7sel(&mut self) -> _INPUT0TO7SELW {
        _INPUT0TO7SELW { w: self }
    }
    #[doc = "Bits 8:11 - CSEN_INPUT8-15 Select"]
    #[inline]
    pub fn input8to15sel(&mut self) -> _INPUT8TO15SELW {
        _INPUT8TO15SELW { w: self }
    }
    #[doc = "Bits 16:19 - CSEN_INPUT16-23 Select"]
    #[inline]
    pub fn input16to23sel(&mut self) -> _INPUT16TO23SELW {
        _INPUT16TO23SELW { w: self }
    }
    #[doc = "Bits 24:27 - CSEN_INPUT24-31 Select"]
    #[inline]
    pub fn input24to31sel(&mut self) -> _INPUT24TO31SELW {
        _INPUT24TO31SELW { w: self }
    }
}
