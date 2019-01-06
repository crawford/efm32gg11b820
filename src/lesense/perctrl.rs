#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PERCTRL {
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
#[doc = r" Value of the field"]
pub struct DACCH0ENR {
    bits: bool,
}
impl DACCH0ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DACCH1ENR {
    bits: bool,
}
impl DACCH1ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DACCH0DATAR {
    bits: bool,
}
impl DACCH0DATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DACCH1DATAR {
    bits: bool,
}
impl DACCH1DATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DACSTARTUPR {
    bits: bool,
}
impl DACSTARTUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DACCONVTRIGR {
    bits: bool,
}
impl DACCONVTRIGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `ACMP0MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP0MODER {
    #[doc = "LESENSE does not control ACMP0"]
    DISABLE,
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0"]
    MUX,
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    MUXTHRES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACMP0MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACMP0MODER::DISABLE => 0,
            ACMP0MODER::MUX => 1,
            ACMP0MODER::MUXTHRES => 2,
            ACMP0MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACMP0MODER {
        match value {
            0 => ACMP0MODER::DISABLE,
            1 => ACMP0MODER::MUX,
            2 => ACMP0MODER::MUXTHRES,
            i => ACMP0MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ACMP0MODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `MUX`"]
    #[inline]
    pub fn is_mux(&self) -> bool {
        *self == ACMP0MODER::MUX
    }
    #[doc = "Checks if the value of the field is `MUXTHRES`"]
    #[inline]
    pub fn is_muxthres(&self) -> bool {
        *self == ACMP0MODER::MUXTHRES
    }
}
#[doc = "Possible values of the field `ACMP1MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP1MODER {
    #[doc = "LESENSE does not control ACMP1"]
    DISABLE,
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1"]
    MUX,
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    MUXTHRES,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACMP1MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACMP1MODER::DISABLE => 0,
            ACMP1MODER::MUX => 1,
            ACMP1MODER::MUXTHRES => 2,
            ACMP1MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACMP1MODER {
        match value {
            0 => ACMP1MODER::DISABLE,
            1 => ACMP1MODER::MUX,
            2 => ACMP1MODER::MUXTHRES,
            i => ACMP1MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ACMP1MODER::DISABLE
    }
    #[doc = "Checks if the value of the field is `MUX`"]
    #[inline]
    pub fn is_mux(&self) -> bool {
        *self == ACMP1MODER::MUX
    }
    #[doc = "Checks if the value of the field is `MUXTHRES`"]
    #[inline]
    pub fn is_muxthres(&self) -> bool {
        *self == ACMP1MODER::MUXTHRES
    }
}
#[doc = r" Value of the field"]
pub struct ACMP0INVR {
    bits: bool,
}
impl ACMP0INVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ACMP1INVR {
    bits: bool,
}
impl ACMP1INVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ACMP0HYSTENR {
    bits: bool,
}
impl ACMP0HYSTENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ACMP1HYSTENR {
    bits: bool,
}
impl ACMP1HYSTENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `WARMUPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WARMUPMODER {
    #[doc = "The analog comparators and VDAC are shut down when LESENSE is idle"]
    NORMAL,
    #[doc = "The analog comparators are kept powered up when LESENSE is idle"]
    KEEPACMPWARM,
    #[doc = "The VDAC is kept powered up when LESENSE is idle"]
    KEEPDACWARM,
    #[doc = "The analog comparators and VDAC are kept powered up when LESENSE is idle"]
    KEEPACMPDACWARM,
}
impl WARMUPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WARMUPMODER::NORMAL => 0,
            WARMUPMODER::KEEPACMPWARM => 1,
            WARMUPMODER::KEEPDACWARM => 2,
            WARMUPMODER::KEEPACMPDACWARM => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WARMUPMODER {
        match value {
            0 => WARMUPMODER::NORMAL,
            1 => WARMUPMODER::KEEPACMPWARM,
            2 => WARMUPMODER::KEEPDACWARM,
            3 => WARMUPMODER::KEEPACMPDACWARM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `KEEPACMPWARM`"]
    #[inline]
    pub fn is_keepacmpwarm(&self) -> bool {
        *self == WARMUPMODER::KEEPACMPWARM
    }
    #[doc = "Checks if the value of the field is `KEEPDACWARM`"]
    #[inline]
    pub fn is_keepdacwarm(&self) -> bool {
        *self == WARMUPMODER::KEEPDACWARM
    }
    #[doc = "Checks if the value of the field is `KEEPACMPDACWARM`"]
    #[inline]
    pub fn is_keepacmpdacwarm(&self) -> bool {
        *self == WARMUPMODER::KEEPACMPDACWARM
    }
}
#[doc = r" Proxy"]
pub struct _DACCH0ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCH0ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DACCH1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCH1ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DACCH0DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCH0DATAW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DACCH1DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCH1DATAW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DACSTARTUPW<'a> {
    w: &'a mut W,
}
impl<'a> _DACSTARTUPW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DACCONVTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _DACCONVTRIGW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACMP0MODE`"]
pub enum ACMP0MODEW {
    #[doc = "LESENSE does not control ACMP0"]
    DISABLE,
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0"]
    MUX,
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    MUXTHRES,
}
impl ACMP0MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACMP0MODEW::DISABLE => 0,
            ACMP0MODEW::MUX => 1,
            ACMP0MODEW::MUXTHRES => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP0MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP0MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP0MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LESENSE does not control ACMP0"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP0MODEW::DISABLE)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0"]
    #[inline]
    pub fn mux(self) -> &'a mut W {
        self.variant(ACMP0MODEW::MUX)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    #[inline]
    pub fn muxthres(self) -> &'a mut W {
        self.variant(ACMP0MODEW::MUXTHRES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACMP1MODE`"]
pub enum ACMP1MODEW {
    #[doc = "LESENSE does not control ACMP1"]
    DISABLE,
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1"]
    MUX,
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    MUXTHRES,
}
impl ACMP1MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACMP1MODEW::DISABLE => 0,
            ACMP1MODEW::MUX => 1,
            ACMP1MODEW::MUXTHRES => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMP1MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP1MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMP1MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LESENSE does not control ACMP1"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP1MODEW::DISABLE)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1"]
    #[inline]
    pub fn mux(self) -> &'a mut W {
        self.variant(ACMP1MODEW::MUX)
    }
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    #[inline]
    pub fn muxthres(self) -> &'a mut W {
        self.variant(ACMP1MODEW::MUXTHRES)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACMP0INVW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP0INVW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACMP1INVW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP1INVW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACMP0HYSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP0HYSTENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACMP1HYSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMP1HYSTENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WARMUPMODE`"]
pub enum WARMUPMODEW {
    #[doc = "The analog comparators and VDAC are shut down when LESENSE is idle"]
    NORMAL,
    #[doc = "The analog comparators are kept powered up when LESENSE is idle"]
    KEEPACMPWARM,
    #[doc = "The VDAC is kept powered up when LESENSE is idle"]
    KEEPDACWARM,
    #[doc = "The analog comparators and VDAC are kept powered up when LESENSE is idle"]
    KEEPACMPDACWARM,
}
impl WARMUPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WARMUPMODEW::NORMAL => 0,
            WARMUPMODEW::KEEPACMPWARM => 1,
            WARMUPMODEW::KEEPDACWARM => 2,
            WARMUPMODEW::KEEPACMPDACWARM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WARMUPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WARMUPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WARMUPMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The analog comparators and VDAC are shut down when LESENSE is idle"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(WARMUPMODEW::NORMAL)
    }
    #[doc = "The analog comparators are kept powered up when LESENSE is idle"]
    #[inline]
    pub fn keepacmpwarm(self) -> &'a mut W {
        self.variant(WARMUPMODEW::KEEPACMPWARM)
    }
    #[doc = "The VDAC is kept powered up when LESENSE is idle"]
    #[inline]
    pub fn keepdacwarm(self) -> &'a mut W {
        self.variant(WARMUPMODEW::KEEPDACWARM)
    }
    #[doc = "The analog comparators and VDAC are kept powered up when LESENSE is idle"]
    #[inline]
    pub fn keepacmpdacwarm(self) -> &'a mut W {
        self.variant(WARMUPMODEW::KEEPACMPDACWARM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - VDAC CH0 Enable"]
    #[inline]
    pub fn dacch0en(&self) -> DACCH0ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DACCH0ENR { bits }
    }
    #[doc = "Bit 1 - VDAC CH1 Enable"]
    #[inline]
    pub fn dacch1en(&self) -> DACCH1ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DACCH1ENR { bits }
    }
    #[doc = "Bit 2 - VDAC CH0 Data Selection"]
    #[inline]
    pub fn dacch0data(&self) -> DACCH0DATAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DACCH0DATAR { bits }
    }
    #[doc = "Bit 3 - VDAC CH1 Data Selection"]
    #[inline]
    pub fn dacch1data(&self) -> DACCH1DATAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DACCH1DATAR { bits }
    }
    #[doc = "Bit 6 - VDAC Startup Configuration"]
    #[inline]
    pub fn dacstartup(&self) -> DACSTARTUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DACSTARTUPR { bits }
    }
    #[doc = "Bit 8 - VDAC Conversion Trigger Configuration"]
    #[inline]
    pub fn dacconvtrig(&self) -> DACCONVTRIGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DACCONVTRIGR { bits }
    }
    #[doc = "Bits 20:21 - ACMP0 Mode"]
    #[inline]
    pub fn acmp0mode(&self) -> ACMP0MODER {
        ACMP0MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - ACMP1 Mode"]
    #[inline]
    pub fn acmp1mode(&self) -> ACMP1MODER {
        ACMP1MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Invert Analog Comparator 0 Output"]
    #[inline]
    pub fn acmp0inv(&self) -> ACMP0INVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACMP0INVR { bits }
    }
    #[doc = "Bit 25 - Invert Analog Comparator 1 Output"]
    #[inline]
    pub fn acmp1inv(&self) -> ACMP1INVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACMP1INVR { bits }
    }
    #[doc = "Bit 26 - ACMP0 Hysteresis Enable"]
    #[inline]
    pub fn acmp0hysten(&self) -> ACMP0HYSTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACMP0HYSTENR { bits }
    }
    #[doc = "Bit 27 - ACMP1 Hysteresis Enable"]
    #[inline]
    pub fn acmp1hysten(&self) -> ACMP1HYSTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACMP1HYSTENR { bits }
    }
    #[doc = "Bits 28:29 - ACMP and VDAC Duty Cycle Mode"]
    #[inline]
    pub fn warmupmode(&self) -> WARMUPMODER {
        WARMUPMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - VDAC CH0 Enable"]
    #[inline]
    pub fn dacch0en(&mut self) -> _DACCH0ENW {
        _DACCH0ENW { w: self }
    }
    #[doc = "Bit 1 - VDAC CH1 Enable"]
    #[inline]
    pub fn dacch1en(&mut self) -> _DACCH1ENW {
        _DACCH1ENW { w: self }
    }
    #[doc = "Bit 2 - VDAC CH0 Data Selection"]
    #[inline]
    pub fn dacch0data(&mut self) -> _DACCH0DATAW {
        _DACCH0DATAW { w: self }
    }
    #[doc = "Bit 3 - VDAC CH1 Data Selection"]
    #[inline]
    pub fn dacch1data(&mut self) -> _DACCH1DATAW {
        _DACCH1DATAW { w: self }
    }
    #[doc = "Bit 6 - VDAC Startup Configuration"]
    #[inline]
    pub fn dacstartup(&mut self) -> _DACSTARTUPW {
        _DACSTARTUPW { w: self }
    }
    #[doc = "Bit 8 - VDAC Conversion Trigger Configuration"]
    #[inline]
    pub fn dacconvtrig(&mut self) -> _DACCONVTRIGW {
        _DACCONVTRIGW { w: self }
    }
    #[doc = "Bits 20:21 - ACMP0 Mode"]
    #[inline]
    pub fn acmp0mode(&mut self) -> _ACMP0MODEW {
        _ACMP0MODEW { w: self }
    }
    #[doc = "Bits 22:23 - ACMP1 Mode"]
    #[inline]
    pub fn acmp1mode(&mut self) -> _ACMP1MODEW {
        _ACMP1MODEW { w: self }
    }
    #[doc = "Bit 24 - Invert Analog Comparator 0 Output"]
    #[inline]
    pub fn acmp0inv(&mut self) -> _ACMP0INVW {
        _ACMP0INVW { w: self }
    }
    #[doc = "Bit 25 - Invert Analog Comparator 1 Output"]
    #[inline]
    pub fn acmp1inv(&mut self) -> _ACMP1INVW {
        _ACMP1INVW { w: self }
    }
    #[doc = "Bit 26 - ACMP0 Hysteresis Enable"]
    #[inline]
    pub fn acmp0hysten(&mut self) -> _ACMP0HYSTENW {
        _ACMP0HYSTENW { w: self }
    }
    #[doc = "Bit 27 - ACMP1 Hysteresis Enable"]
    #[inline]
    pub fn acmp1hysten(&mut self) -> _ACMP1HYSTENW {
        _ACMP1HYSTENW { w: self }
    }
    #[doc = "Bits 28:29 - ACMP and VDAC Duty Cycle Mode"]
    #[inline]
    pub fn warmupmode(&mut self) -> _WARMUPMODEW {
        _WARMUPMODEW { w: self }
    }
}
