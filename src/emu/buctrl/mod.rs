#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BUCTRL {
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
#[doc = r" Value of the field"]
pub struct ENR {
    bits: bool,
}
impl ENR {
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
pub struct STATENR {
    bits: bool,
}
impl STATENR {
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
pub struct BUVINPROBEENR {
    bits: bool,
}
impl BUVINPROBEENR {
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
#[doc = "Possible values of the field `VOUTRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOUTRESR {
    #[doc = "BU_VOUT is not connected"]
    DIS,
    #[doc = "Enable weak switch between BU_VOUT and backup domain power supply."]
    WEAK,
    #[doc = "Enable medium switch between BU_VOUT and backup domain power supply."]
    MED,
    #[doc = "Enable strong switch between BU_VOUT and backup domain power supply."]
    STRONG,
}
impl VOUTRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VOUTRESR::DIS => 0,
            VOUTRESR::WEAK => 1,
            VOUTRESR::MED => 2,
            VOUTRESR::STRONG => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VOUTRESR {
        match value {
            0 => VOUTRESR::DIS,
            1 => VOUTRESR::WEAK,
            2 => VOUTRESR::MED,
            3 => VOUTRESR::STRONG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == VOUTRESR::DIS
    }
    #[doc = "Checks if the value of the field is `WEAK`"]
    #[inline]
    pub fn is_weak(&self) -> bool {
        *self == VOUTRESR::WEAK
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline]
    pub fn is_med(&self) -> bool {
        *self == VOUTRESR::MED
    }
    #[doc = "Checks if the value of the field is `STRONG`"]
    #[inline]
    pub fn is_strong(&self) -> bool {
        *self == VOUTRESR::STRONG
    }
}
#[doc = "Possible values of the field `PWRRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRRESR {
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    RES0,
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    RES1,
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    RES2,
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    RES3,
}
impl PWRRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWRRESR::RES0 => 0,
            PWRRESR::RES1 => 1,
            PWRRESR::RES2 => 2,
            PWRRESR::RES3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWRRESR {
        match value {
            0 => PWRRESR::RES0,
            1 => PWRRESR::RES1,
            2 => PWRRESR::RES2,
            3 => PWRRESR::RES3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline]
    pub fn is_res0(&self) -> bool {
        *self == PWRRESR::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline]
    pub fn is_res1(&self) -> bool {
        *self == PWRRESR::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline]
    pub fn is_res2(&self) -> bool {
        *self == PWRRESR::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline]
    pub fn is_res3(&self) -> bool {
        *self == PWRRESR::RES3
    }
}
#[doc = "Possible values of the field `BUACTPWRCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUACTPWRCONR {
    #[doc = "No connection."]
    NONE,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    MAINBU,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    BUMAIN,
    #[doc = "Main power and backup power are connected without diode."]
    NODIODE,
}
impl BUACTPWRCONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BUACTPWRCONR::NONE => 0,
            BUACTPWRCONR::MAINBU => 1,
            BUACTPWRCONR::BUMAIN => 2,
            BUACTPWRCONR::NODIODE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BUACTPWRCONR {
        match value {
            0 => BUACTPWRCONR::NONE,
            1 => BUACTPWRCONR::MAINBU,
            2 => BUACTPWRCONR::BUMAIN,
            3 => BUACTPWRCONR::NODIODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BUACTPWRCONR::NONE
    }
    #[doc = "Checks if the value of the field is `MAINBU`"]
    #[inline]
    pub fn is_mainbu(&self) -> bool {
        *self == BUACTPWRCONR::MAINBU
    }
    #[doc = "Checks if the value of the field is `BUMAIN`"]
    #[inline]
    pub fn is_bumain(&self) -> bool {
        *self == BUACTPWRCONR::BUMAIN
    }
    #[doc = "Checks if the value of the field is `NODIODE`"]
    #[inline]
    pub fn is_nodiode(&self) -> bool {
        *self == BUACTPWRCONR::NODIODE
    }
}
#[doc = "Possible values of the field `BUINACTPWRCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUINACTPWRCONR {
    #[doc = "No connection."]
    NONE,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    MAINBU,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    BUMAIN,
    #[doc = "Main power and backup power are connected without diode."]
    NODIODE,
}
impl BUINACTPWRCONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BUINACTPWRCONR::NONE => 0,
            BUINACTPWRCONR::MAINBU => 1,
            BUINACTPWRCONR::BUMAIN => 2,
            BUINACTPWRCONR::NODIODE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BUINACTPWRCONR {
        match value {
            0 => BUINACTPWRCONR::NONE,
            1 => BUINACTPWRCONR::MAINBU,
            2 => BUINACTPWRCONR::BUMAIN,
            3 => BUINACTPWRCONR::NODIODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BUINACTPWRCONR::NONE
    }
    #[doc = "Checks if the value of the field is `MAINBU`"]
    #[inline]
    pub fn is_mainbu(&self) -> bool {
        *self == BUINACTPWRCONR::MAINBU
    }
    #[doc = "Checks if the value of the field is `BUMAIN`"]
    #[inline]
    pub fn is_bumain(&self) -> bool {
        *self == BUINACTPWRCONR::BUMAIN
    }
    #[doc = "Checks if the value of the field is `NODIODE`"]
    #[inline]
    pub fn is_nodiode(&self) -> bool {
        *self == BUINACTPWRCONR::NODIODE
    }
}
#[doc = r" Value of the field"]
pub struct DISMAXCOMPR {
    bits: bool,
}
impl DISMAXCOMPR {
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
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
pub struct _STATENW<'a> {
    w: &'a mut W,
}
impl<'a> _STATENW<'a> {
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
pub struct _BUVINPROBEENW<'a> {
    w: &'a mut W,
}
impl<'a> _BUVINPROBEENW<'a> {
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
#[doc = "Values that can be written to the field `VOUTRES`"]
pub enum VOUTRESW {
    #[doc = "BU_VOUT is not connected"]
    DIS,
    #[doc = "Enable weak switch between BU_VOUT and backup domain power supply."]
    WEAK,
    #[doc = "Enable medium switch between BU_VOUT and backup domain power supply."]
    MED,
    #[doc = "Enable strong switch between BU_VOUT and backup domain power supply."]
    STRONG,
}
impl VOUTRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VOUTRESW::DIS => 0,
            VOUTRESW::WEAK => 1,
            VOUTRESW::MED => 2,
            VOUTRESW::STRONG => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VOUTRESW<'a> {
    w: &'a mut W,
}
impl<'a> _VOUTRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VOUTRESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "BU_VOUT is not connected"]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(VOUTRESW::DIS)
    }
    #[doc = "Enable weak switch between BU_VOUT and backup domain power supply."]
    #[inline]
    pub fn weak(self) -> &'a mut W {
        self.variant(VOUTRESW::WEAK)
    }
    #[doc = "Enable medium switch between BU_VOUT and backup domain power supply."]
    #[inline]
    pub fn med(self) -> &'a mut W {
        self.variant(VOUTRESW::MED)
    }
    #[doc = "Enable strong switch between BU_VOUT and backup domain power supply."]
    #[inline]
    pub fn strong(self) -> &'a mut W {
        self.variant(VOUTRESW::STRONG)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWRRES`"]
pub enum PWRRESW {
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    RES0,
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    RES1,
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    RES2,
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    RES3,
}
impl PWRRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWRRESW::RES0 => 0,
            PWRRESW::RES1 => 1,
            PWRRESW::RES2 => 2,
            PWRRESW::RES3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRRESW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRRESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    #[inline]
    pub fn res0(self) -> &'a mut W {
        self.variant(PWRRESW::RES0)
    }
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    #[inline]
    pub fn res1(self) -> &'a mut W {
        self.variant(PWRRESW::RES1)
    }
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    #[inline]
    pub fn res2(self) -> &'a mut W {
        self.variant(PWRRESW::RES2)
    }
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    #[inline]
    pub fn res3(self) -> &'a mut W {
        self.variant(PWRRESW::RES3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUACTPWRCON`"]
pub enum BUACTPWRCONW {
    #[doc = "No connection."]
    NONE,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    MAINBU,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    BUMAIN,
    #[doc = "Main power and backup power are connected without diode."]
    NODIODE,
}
impl BUACTPWRCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BUACTPWRCONW::NONE => 0,
            BUACTPWRCONW::MAINBU => 1,
            BUACTPWRCONW::BUMAIN => 2,
            BUACTPWRCONW::NODIODE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUACTPWRCONW<'a> {
    w: &'a mut W,
}
impl<'a> _BUACTPWRCONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUACTPWRCONW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No connection."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BUACTPWRCONW::NONE)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline]
    pub fn mainbu(self) -> &'a mut W {
        self.variant(BUACTPWRCONW::MAINBU)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline]
    pub fn bumain(self) -> &'a mut W {
        self.variant(BUACTPWRCONW::BUMAIN)
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline]
    pub fn nodiode(self) -> &'a mut W {
        self.variant(BUACTPWRCONW::NODIODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BUINACTPWRCON`"]
pub enum BUINACTPWRCONW {
    #[doc = "No connection."]
    NONE,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    MAINBU,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    BUMAIN,
    #[doc = "Main power and backup power are connected without diode."]
    NODIODE,
}
impl BUINACTPWRCONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BUINACTPWRCONW::NONE => 0,
            BUINACTPWRCONW::MAINBU => 1,
            BUINACTPWRCONW::BUMAIN => 2,
            BUINACTPWRCONW::NODIODE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BUINACTPWRCONW<'a> {
    w: &'a mut W,
}
impl<'a> _BUINACTPWRCONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUINACTPWRCONW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No connection."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BUINACTPWRCONW::NONE)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline]
    pub fn mainbu(self) -> &'a mut W {
        self.variant(BUINACTPWRCONW::MAINBU)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline]
    pub fn bumain(self) -> &'a mut W {
        self.variant(BUINACTPWRCONW::BUMAIN)
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline]
    pub fn nodiode(self) -> &'a mut W {
        self.variant(BUINACTPWRCONW::NODIODE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISMAXCOMPW<'a> {
    w: &'a mut W,
}
impl<'a> _DISMAXCOMPW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Enable Backup Mode"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 1 - Enable Backup Mode Status Export"]
    #[inline]
    pub fn staten(&self) -> STATENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STATENR { bits }
    }
    #[doc = "Bit 2 - Enable BU_VIN Probing"]
    #[inline]
    pub fn buvinprobeen(&self) -> BUVINPROBEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BUVINPROBEENR { bits }
    }
    #[doc = "Bits 8:9 - BU_VOUT Resistor Select"]
    #[inline]
    pub fn voutres(&self) -> VOUTRESR {
        VOUTRESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Power Domain Resistor Select"]
    #[inline]
    pub fn pwrres(&self) -> PWRRESR {
        PWRRESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Power Connection Configuration in Backup Mode"]
    #[inline]
    pub fn buactpwrcon(&self) -> BUACTPWRCONR {
        BUACTPWRCONR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Power Connection Configuration When Not in Backup Mode"]
    #[inline]
    pub fn buinactpwrcon(&self) -> BUINACTPWRCONR {
        BUINACTPWRCONR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Disable MAIN-BU Comparator"]
    #[inline]
    pub fn dismaxcomp(&self) -> DISMAXCOMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISMAXCOMPR { bits }
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
    #[doc = "Bit 0 - Enable Backup Mode"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 1 - Enable Backup Mode Status Export"]
    #[inline]
    pub fn staten(&mut self) -> _STATENW {
        _STATENW { w: self }
    }
    #[doc = "Bit 2 - Enable BU_VIN Probing"]
    #[inline]
    pub fn buvinprobeen(&mut self) -> _BUVINPROBEENW {
        _BUVINPROBEENW { w: self }
    }
    #[doc = "Bits 8:9 - BU_VOUT Resistor Select"]
    #[inline]
    pub fn voutres(&mut self) -> _VOUTRESW {
        _VOUTRESW { w: self }
    }
    #[doc = "Bits 12:13 - Power Domain Resistor Select"]
    #[inline]
    pub fn pwrres(&mut self) -> _PWRRESW {
        _PWRRESW { w: self }
    }
    #[doc = "Bits 16:17 - Power Connection Configuration in Backup Mode"]
    #[inline]
    pub fn buactpwrcon(&mut self) -> _BUACTPWRCONW {
        _BUACTPWRCONW { w: self }
    }
    #[doc = "Bits 20:21 - Power Connection Configuration When Not in Backup Mode"]
    #[inline]
    pub fn buinactpwrcon(&mut self) -> _BUINACTPWRCONW {
        _BUINACTPWRCONW { w: self }
    }
    #[doc = "Bit 31 - Disable MAIN-BU Comparator"]
    #[inline]
    pub fn dismaxcomp(&mut self) -> _DISMAXCOMPW {
        _DISMAXCOMPW { w: self }
    }
}
