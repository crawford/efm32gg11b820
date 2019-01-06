#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IDLECONF {
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
#[doc = "Possible values of the field `CH0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0R {
    #[doc = "CH0 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH0 output is high in idle phase"]
    HIGH,
    #[doc = "CH0 output is low in idle phase"]
    LOW,
    #[doc = "CH0 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH0R::DISABLE => 0,
            CH0R::HIGH => 1,
            CH0R::LOW => 2,
            CH0R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH0R {
        match value {
            0 => CH0R::DISABLE,
            1 => CH0R::HIGH,
            2 => CH0R::LOW,
            3 => CH0R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH0R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH0R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH0R::DAC
    }
}
#[doc = "Possible values of the field `CH1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1R {
    #[doc = "CH1 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH1 output is high in idle phase"]
    HIGH,
    #[doc = "CH1 output is low in idle phase"]
    LOW,
    #[doc = "CH1 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH1R::DISABLE => 0,
            CH1R::HIGH => 1,
            CH1R::LOW => 2,
            CH1R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH1R {
        match value {
            0 => CH1R::DISABLE,
            1 => CH1R::HIGH,
            2 => CH1R::LOW,
            3 => CH1R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH1R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH1R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH1R::DAC
    }
}
#[doc = "Possible values of the field `CH2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2R {
    #[doc = "CH2 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH2 output is high in idle phase"]
    HIGH,
    #[doc = "CH2 output is low in idle phase"]
    LOW,
    #[doc = "CH2 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH2R::DISABLE => 0,
            CH2R::HIGH => 1,
            CH2R::LOW => 2,
            CH2R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH2R {
        match value {
            0 => CH2R::DISABLE,
            1 => CH2R::HIGH,
            2 => CH2R::LOW,
            3 => CH2R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH2R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH2R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH2R::DAC
    }
}
#[doc = "Possible values of the field `CH3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3R {
    #[doc = "CH3 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH3 output is high in idle phase"]
    HIGH,
    #[doc = "CH3 output is low in idle phase"]
    LOW,
    #[doc = "CH3 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH3R::DISABLE => 0,
            CH3R::HIGH => 1,
            CH3R::LOW => 2,
            CH3R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH3R {
        match value {
            0 => CH3R::DISABLE,
            1 => CH3R::HIGH,
            2 => CH3R::LOW,
            3 => CH3R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH3R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH3R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH3R::DAC
    }
}
#[doc = "Possible values of the field `CH4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4R {
    #[doc = "CH4 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH4 output is high in idle phase"]
    HIGH,
    #[doc = "CH4 output is low in idle phase"]
    LOW,
    #[doc = "CH4 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH4R::DISABLE => 0,
            CH4R::HIGH => 1,
            CH4R::LOW => 2,
            CH4R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH4R {
        match value {
            0 => CH4R::DISABLE,
            1 => CH4R::HIGH,
            2 => CH4R::LOW,
            3 => CH4R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH4R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH4R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH4R::DAC
    }
}
#[doc = "Possible values of the field `CH5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5R {
    #[doc = "CH5 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH5 output is high in idle phase"]
    HIGH,
    #[doc = "CH5 output is low in idle phase"]
    LOW,
    #[doc = "CH5 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH5R::DISABLE => 0,
            CH5R::HIGH => 1,
            CH5R::LOW => 2,
            CH5R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH5R {
        match value {
            0 => CH5R::DISABLE,
            1 => CH5R::HIGH,
            2 => CH5R::LOW,
            3 => CH5R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH5R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH5R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH5R::DAC
    }
}
#[doc = "Possible values of the field `CH6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6R {
    #[doc = "CH6 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH6 output is high in idle phase"]
    HIGH,
    #[doc = "CH6 output is low in idle phase"]
    LOW,
    #[doc = "CH6 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH6R::DISABLE => 0,
            CH6R::HIGH => 1,
            CH6R::LOW => 2,
            CH6R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH6R {
        match value {
            0 => CH6R::DISABLE,
            1 => CH6R::HIGH,
            2 => CH6R::LOW,
            3 => CH6R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH6R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH6R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH6R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH6R::DAC
    }
}
#[doc = "Possible values of the field `CH7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7R {
    #[doc = "CH7 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH7 output is high in idle phase"]
    HIGH,
    #[doc = "CH7 output is low in idle phase"]
    LOW,
    #[doc = "CH7 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH7R::DISABLE => 0,
            CH7R::HIGH => 1,
            CH7R::LOW => 2,
            CH7R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH7R {
        match value {
            0 => CH7R::DISABLE,
            1 => CH7R::HIGH,
            2 => CH7R::LOW,
            3 => CH7R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH7R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH7R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH7R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH7R::DAC
    }
}
#[doc = "Possible values of the field `CH8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8R {
    #[doc = "CH8 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH8 output is high in idle phase"]
    HIGH,
    #[doc = "CH8 output is low in idle phase"]
    LOW,
    #[doc = "CH8 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH8R::DISABLE => 0,
            CH8R::HIGH => 1,
            CH8R::LOW => 2,
            CH8R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH8R {
        match value {
            0 => CH8R::DISABLE,
            1 => CH8R::HIGH,
            2 => CH8R::LOW,
            3 => CH8R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH8R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH8R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH8R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH8R::DAC
    }
}
#[doc = "Possible values of the field `CH9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9R {
    #[doc = "CH9 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH9 output is high in idle phase"]
    HIGH,
    #[doc = "CH9 output is low in idle phase"]
    LOW,
    #[doc = "CH9 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH9R::DISABLE => 0,
            CH9R::HIGH => 1,
            CH9R::LOW => 2,
            CH9R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH9R {
        match value {
            0 => CH9R::DISABLE,
            1 => CH9R::HIGH,
            2 => CH9R::LOW,
            3 => CH9R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH9R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH9R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH9R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH9R::DAC
    }
}
#[doc = "Possible values of the field `CH10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10R {
    #[doc = "CH10 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH10 output is high in idle phase"]
    HIGH,
    #[doc = "CH10 output is low in idle phase"]
    LOW,
    #[doc = "CH10 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH10R::DISABLE => 0,
            CH10R::HIGH => 1,
            CH10R::LOW => 2,
            CH10R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH10R {
        match value {
            0 => CH10R::DISABLE,
            1 => CH10R::HIGH,
            2 => CH10R::LOW,
            3 => CH10R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH10R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH10R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH10R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH10R::DAC
    }
}
#[doc = "Possible values of the field `CH11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11R {
    #[doc = "CH11 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH11 output is high in idle phase"]
    HIGH,
    #[doc = "CH11 output is low in idle phase"]
    LOW,
    #[doc = "CH11 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH11R::DISABLE => 0,
            CH11R::HIGH => 1,
            CH11R::LOW => 2,
            CH11R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH11R {
        match value {
            0 => CH11R::DISABLE,
            1 => CH11R::HIGH,
            2 => CH11R::LOW,
            3 => CH11R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH11R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH11R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH11R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH11R::DAC
    }
}
#[doc = "Possible values of the field `CH12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12R {
    #[doc = "CH12 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH12 output is high in idle phase"]
    HIGH,
    #[doc = "CH12 output is low in idle phase"]
    LOW,
    #[doc = "CH12 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH12R::DISABLE => 0,
            CH12R::HIGH => 1,
            CH12R::LOW => 2,
            CH12R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH12R {
        match value {
            0 => CH12R::DISABLE,
            1 => CH12R::HIGH,
            2 => CH12R::LOW,
            3 => CH12R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH12R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH12R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH12R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH12R::DAC
    }
}
#[doc = "Possible values of the field `CH13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13R {
    #[doc = "CH13 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH13 output is high in idle phase"]
    HIGH,
    #[doc = "CH13 output is low in idle phase"]
    LOW,
    #[doc = "CH13 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH13R::DISABLE => 0,
            CH13R::HIGH => 1,
            CH13R::LOW => 2,
            CH13R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH13R {
        match value {
            0 => CH13R::DISABLE,
            1 => CH13R::HIGH,
            2 => CH13R::LOW,
            3 => CH13R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH13R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH13R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH13R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH13R::DAC
    }
}
#[doc = "Possible values of the field `CH14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14R {
    #[doc = "CH14 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH14 output is high in idle phase"]
    HIGH,
    #[doc = "CH14 output is low in idle phase"]
    LOW,
    #[doc = "CH14 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH14R::DISABLE => 0,
            CH14R::HIGH => 1,
            CH14R::LOW => 2,
            CH14R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH14R {
        match value {
            0 => CH14R::DISABLE,
            1 => CH14R::HIGH,
            2 => CH14R::LOW,
            3 => CH14R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH14R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH14R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH14R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH14R::DAC
    }
}
#[doc = "Possible values of the field `CH15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15R {
    #[doc = "CH15 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH15 output is high in idle phase"]
    HIGH,
    #[doc = "CH15 output is low in idle phase"]
    LOW,
    #[doc = "CH15 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH15R::DISABLE => 0,
            CH15R::HIGH => 1,
            CH15R::LOW => 2,
            CH15R::DAC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH15R {
        match value {
            0 => CH15R::DISABLE,
            1 => CH15R::HIGH,
            2 => CH15R::LOW,
            3 => CH15R::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CH15R::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CH15R::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CH15R::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline]
    pub fn is_dac(&self) -> bool {
        *self == CH15R::DAC
    }
}
#[doc = "Values that can be written to the field `CH0`"]
pub enum CH0W {
    #[doc = "CH0 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH0 output is high in idle phase"]
    HIGH,
    #[doc = "CH0 output is low in idle phase"]
    LOW,
    #[doc = "CH0 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH0W::DISABLE => 0,
            CH0W::HIGH => 1,
            CH0W::LOW => 2,
            CH0W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH0W<'a> {
    w: &'a mut W,
}
impl<'a> _CH0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH0 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH0W::DISABLE)
    }
    #[doc = "CH0 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH0W::HIGH)
    }
    #[doc = "CH0 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH0W::LOW)
    }
    #[doc = "CH0 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH0W::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH1`"]
pub enum CH1W {
    #[doc = "CH1 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH1 output is high in idle phase"]
    HIGH,
    #[doc = "CH1 output is low in idle phase"]
    LOW,
    #[doc = "CH1 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH1W::DISABLE => 0,
            CH1W::HIGH => 1,
            CH1W::LOW => 2,
            CH1W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH1W<'a> {
    w: &'a mut W,
}
impl<'a> _CH1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH1 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH1W::DISABLE)
    }
    #[doc = "CH1 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH1W::HIGH)
    }
    #[doc = "CH1 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH1W::LOW)
    }
    #[doc = "CH1 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH1W::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH2`"]
pub enum CH2W {
    #[doc = "CH2 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH2 output is high in idle phase"]
    HIGH,
    #[doc = "CH2 output is low in idle phase"]
    LOW,
    #[doc = "CH2 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH2W::DISABLE => 0,
            CH2W::HIGH => 1,
            CH2W::LOW => 2,
            CH2W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH2W<'a> {
    w: &'a mut W,
}
impl<'a> _CH2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH2 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH2W::DISABLE)
    }
    #[doc = "CH2 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH2W::HIGH)
    }
    #[doc = "CH2 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH2W::LOW)
    }
    #[doc = "CH2 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH2W::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH3`"]
pub enum CH3W {
    #[doc = "CH3 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH3 output is high in idle phase"]
    HIGH,
    #[doc = "CH3 output is low in idle phase"]
    LOW,
    #[doc = "CH3 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH3W::DISABLE => 0,
            CH3W::HIGH => 1,
            CH3W::LOW => 2,
            CH3W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH3W<'a> {
    w: &'a mut W,
}
impl<'a> _CH3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH3 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH3W::DISABLE)
    }
    #[doc = "CH3 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH3W::HIGH)
    }
    #[doc = "CH3 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH3W::LOW)
    }
    #[doc = "CH3 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH3W::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH4`"]
pub enum CH4W {
    #[doc = "CH4 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH4 output is high in idle phase"]
    HIGH,
    #[doc = "CH4 output is low in idle phase"]
    LOW,
    #[doc = "CH4 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH4W::DISABLE => 0,
            CH4W::HIGH => 1,
            CH4W::LOW => 2,
            CH4W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH4W<'a> {
    w: &'a mut W,
}
impl<'a> _CH4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH4 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH4W::DISABLE)
    }
    #[doc = "CH4 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH4W::HIGH)
    }
    #[doc = "CH4 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH4W::LOW)
    }
    #[doc = "CH4 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH4W::DAC)
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
#[doc = "Values that can be written to the field `CH5`"]
pub enum CH5W {
    #[doc = "CH5 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH5 output is high in idle phase"]
    HIGH,
    #[doc = "CH5 output is low in idle phase"]
    LOW,
    #[doc = "CH5 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH5W::DISABLE => 0,
            CH5W::HIGH => 1,
            CH5W::LOW => 2,
            CH5W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH5W<'a> {
    w: &'a mut W,
}
impl<'a> _CH5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH5 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH5W::DISABLE)
    }
    #[doc = "CH5 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH5W::HIGH)
    }
    #[doc = "CH5 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH5W::LOW)
    }
    #[doc = "CH5 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH5W::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH6`"]
pub enum CH6W {
    #[doc = "CH6 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH6 output is high in idle phase"]
    HIGH,
    #[doc = "CH6 output is low in idle phase"]
    LOW,
    #[doc = "CH6 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH6W::DISABLE => 0,
            CH6W::HIGH => 1,
            CH6W::LOW => 2,
            CH6W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH6W<'a> {
    w: &'a mut W,
}
impl<'a> _CH6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH6 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH6W::DISABLE)
    }
    #[doc = "CH6 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH6W::HIGH)
    }
    #[doc = "CH6 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH6W::LOW)
    }
    #[doc = "CH6 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH6W::DAC)
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
#[doc = "Values that can be written to the field `CH7`"]
pub enum CH7W {
    #[doc = "CH7 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH7 output is high in idle phase"]
    HIGH,
    #[doc = "CH7 output is low in idle phase"]
    LOW,
    #[doc = "CH7 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH7W::DISABLE => 0,
            CH7W::HIGH => 1,
            CH7W::LOW => 2,
            CH7W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH7W<'a> {
    w: &'a mut W,
}
impl<'a> _CH7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH7 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH7W::DISABLE)
    }
    #[doc = "CH7 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH7W::HIGH)
    }
    #[doc = "CH7 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH7W::LOW)
    }
    #[doc = "CH7 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH7W::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH8`"]
pub enum CH8W {
    #[doc = "CH8 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH8 output is high in idle phase"]
    HIGH,
    #[doc = "CH8 output is low in idle phase"]
    LOW,
    #[doc = "CH8 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH8W::DISABLE => 0,
            CH8W::HIGH => 1,
            CH8W::LOW => 2,
            CH8W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH8W<'a> {
    w: &'a mut W,
}
impl<'a> _CH8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH8 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH8W::DISABLE)
    }
    #[doc = "CH8 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH8W::HIGH)
    }
    #[doc = "CH8 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH8W::LOW)
    }
    #[doc = "CH8 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH8W::DAC)
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
#[doc = "Values that can be written to the field `CH9`"]
pub enum CH9W {
    #[doc = "CH9 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH9 output is high in idle phase"]
    HIGH,
    #[doc = "CH9 output is low in idle phase"]
    LOW,
    #[doc = "CH9 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH9W::DISABLE => 0,
            CH9W::HIGH => 1,
            CH9W::LOW => 2,
            CH9W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH9W<'a> {
    w: &'a mut W,
}
impl<'a> _CH9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH9 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH9W::DISABLE)
    }
    #[doc = "CH9 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH9W::HIGH)
    }
    #[doc = "CH9 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH9W::LOW)
    }
    #[doc = "CH9 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH9W::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH10`"]
pub enum CH10W {
    #[doc = "CH10 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH10 output is high in idle phase"]
    HIGH,
    #[doc = "CH10 output is low in idle phase"]
    LOW,
    #[doc = "CH10 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH10W::DISABLE => 0,
            CH10W::HIGH => 1,
            CH10W::LOW => 2,
            CH10W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH10W<'a> {
    w: &'a mut W,
}
impl<'a> _CH10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH10 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH10W::DISABLE)
    }
    #[doc = "CH10 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH10W::HIGH)
    }
    #[doc = "CH10 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH10W::LOW)
    }
    #[doc = "CH10 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH10W::DAC)
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
#[doc = "Values that can be written to the field `CH11`"]
pub enum CH11W {
    #[doc = "CH11 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH11 output is high in idle phase"]
    HIGH,
    #[doc = "CH11 output is low in idle phase"]
    LOW,
    #[doc = "CH11 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH11W::DISABLE => 0,
            CH11W::HIGH => 1,
            CH11W::LOW => 2,
            CH11W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH11W<'a> {
    w: &'a mut W,
}
impl<'a> _CH11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH11 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH11W::DISABLE)
    }
    #[doc = "CH11 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH11W::HIGH)
    }
    #[doc = "CH11 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH11W::LOW)
    }
    #[doc = "CH11 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH11W::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH12`"]
pub enum CH12W {
    #[doc = "CH12 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH12 output is high in idle phase"]
    HIGH,
    #[doc = "CH12 output is low in idle phase"]
    LOW,
    #[doc = "CH12 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH12W::DISABLE => 0,
            CH12W::HIGH => 1,
            CH12W::LOW => 2,
            CH12W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH12W<'a> {
    w: &'a mut W,
}
impl<'a> _CH12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH12W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH12 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH12W::DISABLE)
    }
    #[doc = "CH12 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH12W::HIGH)
    }
    #[doc = "CH12 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH12W::LOW)
    }
    #[doc = "CH12 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH12W::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH13`"]
pub enum CH13W {
    #[doc = "CH13 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH13 output is high in idle phase"]
    HIGH,
    #[doc = "CH13 output is low in idle phase"]
    LOW,
    #[doc = "CH13 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH13W::DISABLE => 0,
            CH13W::HIGH => 1,
            CH13W::LOW => 2,
            CH13W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH13W<'a> {
    w: &'a mut W,
}
impl<'a> _CH13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH13W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH13 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH13W::DISABLE)
    }
    #[doc = "CH13 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH13W::HIGH)
    }
    #[doc = "CH13 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH13W::LOW)
    }
    #[doc = "CH13 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH13W::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CH14`"]
pub enum CH14W {
    #[doc = "CH14 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH14 output is high in idle phase"]
    HIGH,
    #[doc = "CH14 output is low in idle phase"]
    LOW,
    #[doc = "CH14 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH14W::DISABLE => 0,
            CH14W::HIGH => 1,
            CH14W::LOW => 2,
            CH14W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH14W<'a> {
    w: &'a mut W,
}
impl<'a> _CH14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH14W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH14 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH14W::DISABLE)
    }
    #[doc = "CH14 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH14W::HIGH)
    }
    #[doc = "CH14 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH14W::LOW)
    }
    #[doc = "CH14 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH14W::DAC)
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
#[doc = "Values that can be written to the field `CH15`"]
pub enum CH15W {
    #[doc = "CH15 output is disabled in idle phase"]
    DISABLE,
    #[doc = "CH15 output is high in idle phase"]
    HIGH,
    #[doc = "CH15 output is low in idle phase"]
    LOW,
    #[doc = "CH15 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl CH15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CH15W::DISABLE => 0,
            CH15W::HIGH => 1,
            CH15W::LOW => 2,
            CH15W::DAC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CH15W<'a> {
    w: &'a mut W,
}
impl<'a> _CH15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CH15 output is disabled in idle phase"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH15W::DISABLE)
    }
    #[doc = "CH15 output is high in idle phase"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CH15W::HIGH)
    }
    #[doc = "CH15 output is low in idle phase"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CH15W::LOW)
    }
    #[doc = "CH15 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH15W::DAC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Channel 0 Idle Phase Configuration"]
    #[inline]
    pub fn ch0(&self) -> CH0R {
        CH0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Channel 1 Idle Phase Configuration"]
    #[inline]
    pub fn ch1(&self) -> CH1R {
        CH1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Channel 2 Idle Phase Configuration"]
    #[inline]
    pub fn ch2(&self) -> CH2R {
        CH2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Channel 3 Idle Phase Configuration"]
    #[inline]
    pub fn ch3(&self) -> CH3R {
        CH3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Channel 4 Idle Phase Configuration"]
    #[inline]
    pub fn ch4(&self) -> CH4R {
        CH4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Channel 5 Idle Phase Configuration"]
    #[inline]
    pub fn ch5(&self) -> CH5R {
        CH5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Channel 6 Idle Phase Configuration"]
    #[inline]
    pub fn ch6(&self) -> CH6R {
        CH6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Channel 7 Idle Phase Configuration"]
    #[inline]
    pub fn ch7(&self) -> CH7R {
        CH7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Channel 8 Idle Phase Configuration"]
    #[inline]
    pub fn ch8(&self) -> CH8R {
        CH8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Channel 9 Idle Phase Configuration"]
    #[inline]
    pub fn ch9(&self) -> CH9R {
        CH9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Channel 10 Idle Phase Configuration"]
    #[inline]
    pub fn ch10(&self) -> CH10R {
        CH10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Channel 11 Idle Phase Configuration"]
    #[inline]
    pub fn ch11(&self) -> CH11R {
        CH11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Channel 12 Idle Phase Configuration"]
    #[inline]
    pub fn ch12(&self) -> CH12R {
        CH12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Channel 13 Idle Phase Configuration"]
    #[inline]
    pub fn ch13(&self) -> CH13R {
        CH13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Channel 14 Idle Phase Configuration"]
    #[inline]
    pub fn ch14(&self) -> CH14R {
        CH14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Channel 15 Idle Phase Configuration"]
    #[inline]
    pub fn ch15(&self) -> CH15R {
        CH15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Channel 0 Idle Phase Configuration"]
    #[inline]
    pub fn ch0(&mut self) -> _CH0W {
        _CH0W { w: self }
    }
    #[doc = "Bits 2:3 - Channel 1 Idle Phase Configuration"]
    #[inline]
    pub fn ch1(&mut self) -> _CH1W {
        _CH1W { w: self }
    }
    #[doc = "Bits 4:5 - Channel 2 Idle Phase Configuration"]
    #[inline]
    pub fn ch2(&mut self) -> _CH2W {
        _CH2W { w: self }
    }
    #[doc = "Bits 6:7 - Channel 3 Idle Phase Configuration"]
    #[inline]
    pub fn ch3(&mut self) -> _CH3W {
        _CH3W { w: self }
    }
    #[doc = "Bits 8:9 - Channel 4 Idle Phase Configuration"]
    #[inline]
    pub fn ch4(&mut self) -> _CH4W {
        _CH4W { w: self }
    }
    #[doc = "Bits 10:11 - Channel 5 Idle Phase Configuration"]
    #[inline]
    pub fn ch5(&mut self) -> _CH5W {
        _CH5W { w: self }
    }
    #[doc = "Bits 12:13 - Channel 6 Idle Phase Configuration"]
    #[inline]
    pub fn ch6(&mut self) -> _CH6W {
        _CH6W { w: self }
    }
    #[doc = "Bits 14:15 - Channel 7 Idle Phase Configuration"]
    #[inline]
    pub fn ch7(&mut self) -> _CH7W {
        _CH7W { w: self }
    }
    #[doc = "Bits 16:17 - Channel 8 Idle Phase Configuration"]
    #[inline]
    pub fn ch8(&mut self) -> _CH8W {
        _CH8W { w: self }
    }
    #[doc = "Bits 18:19 - Channel 9 Idle Phase Configuration"]
    #[inline]
    pub fn ch9(&mut self) -> _CH9W {
        _CH9W { w: self }
    }
    #[doc = "Bits 20:21 - Channel 10 Idle Phase Configuration"]
    #[inline]
    pub fn ch10(&mut self) -> _CH10W {
        _CH10W { w: self }
    }
    #[doc = "Bits 22:23 - Channel 11 Idle Phase Configuration"]
    #[inline]
    pub fn ch11(&mut self) -> _CH11W {
        _CH11W { w: self }
    }
    #[doc = "Bits 24:25 - Channel 12 Idle Phase Configuration"]
    #[inline]
    pub fn ch12(&mut self) -> _CH12W {
        _CH12W { w: self }
    }
    #[doc = "Bits 26:27 - Channel 13 Idle Phase Configuration"]
    #[inline]
    pub fn ch13(&mut self) -> _CH13W {
        _CH13W { w: self }
    }
    #[doc = "Bits 28:29 - Channel 14 Idle Phase Configuration"]
    #[inline]
    pub fn ch14(&mut self) -> _CH14W {
        _CH14W { w: self }
    }
    #[doc = "Bits 30:31 - Channel 15 Idle Phase Configuration"]
    #[inline]
    pub fn ch15(&mut self) -> _CH15W {
        _CH15W { w: self }
    }
}
