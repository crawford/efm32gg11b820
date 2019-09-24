#[doc = "Reader of register PF_MODEL"]
pub type R = crate::R<u32, super::PF_MODEL>;
#[doc = "Writer for register PF_MODEL"]
pub type W = crate::W<u32, super::PF_MODEL>;
#[doc = "Register PF_MODEL `reset()`'s with value 0"]
impl crate::ResetValue for super::PF_MODEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pin 0 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE0_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "4: Push-pull output"]
    PUSHPULL,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "6: Wired-or output"]
    WIREDOR,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "8: Open-drain output"]
    WIREDAND,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl From<MODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE0_A) -> Self {
        match variant {
            MODE0_A::DISABLED => 0,
            MODE0_A::INPUT => 1,
            MODE0_A::INPUTPULL => 2,
            MODE0_A::INPUTPULLFILTER => 3,
            MODE0_A::PUSHPULL => 4,
            MODE0_A::PUSHPULLALT => 5,
            MODE0_A::WIREDOR => 6,
            MODE0_A::WIREDORPULLDOWN => 7,
            MODE0_A::WIREDAND => 8,
            MODE0_A::WIREDANDFILTER => 9,
            MODE0_A::WIREDANDPULLUP => 10,
            MODE0_A::WIREDANDPULLUPFILTER => 11,
            MODE0_A::WIREDANDALT => 12,
            MODE0_A::WIREDANDALTFILTER => 13,
            MODE0_A::WIREDANDALTPULLUP => 14,
            MODE0_A::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = "Reader of field `MODE0`"]
pub type MODE0_R = crate::R<u8, MODE0_A>;
impl MODE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE0_A {
        match self.bits {
            0 => MODE0_A::DISABLED,
            1 => MODE0_A::INPUT,
            2 => MODE0_A::INPUTPULL,
            3 => MODE0_A::INPUTPULLFILTER,
            4 => MODE0_A::PUSHPULL,
            5 => MODE0_A::PUSHPULLALT,
            6 => MODE0_A::WIREDOR,
            7 => MODE0_A::WIREDORPULLDOWN,
            8 => MODE0_A::WIREDAND,
            9 => MODE0_A::WIREDANDFILTER,
            10 => MODE0_A::WIREDANDPULLUP,
            11 => MODE0_A::WIREDANDPULLUPFILTER,
            12 => MODE0_A::WIREDANDALT,
            13 => MODE0_A::WIREDANDALTFILTER,
            14 => MODE0_A::WIREDANDALTPULLUP,
            15 => MODE0_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE0_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE0_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE0_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE0_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE0_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE0_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE0_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE0_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE0_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE0_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE0_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE0_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE0_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE0_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE0`"]
pub struct MODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE0_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE0_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE0_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE0_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE0_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE0_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE0_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Pin 1 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE1_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "4: Push-pull output"]
    PUSHPULL,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "6: Wired-or output"]
    WIREDOR,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "8: Open-drain output"]
    WIREDAND,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        match variant {
            MODE1_A::DISABLED => 0,
            MODE1_A::INPUT => 1,
            MODE1_A::INPUTPULL => 2,
            MODE1_A::INPUTPULLFILTER => 3,
            MODE1_A::PUSHPULL => 4,
            MODE1_A::PUSHPULLALT => 5,
            MODE1_A::WIREDOR => 6,
            MODE1_A::WIREDORPULLDOWN => 7,
            MODE1_A::WIREDAND => 8,
            MODE1_A::WIREDANDFILTER => 9,
            MODE1_A::WIREDANDPULLUP => 10,
            MODE1_A::WIREDANDPULLUPFILTER => 11,
            MODE1_A::WIREDANDALT => 12,
            MODE1_A::WIREDANDALTFILTER => 13,
            MODE1_A::WIREDANDALTPULLUP => 14,
            MODE1_A::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = "Reader of field `MODE1`"]
pub type MODE1_R = crate::R<u8, MODE1_A>;
impl MODE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::DISABLED,
            1 => MODE1_A::INPUT,
            2 => MODE1_A::INPUTPULL,
            3 => MODE1_A::INPUTPULLFILTER,
            4 => MODE1_A::PUSHPULL,
            5 => MODE1_A::PUSHPULLALT,
            6 => MODE1_A::WIREDOR,
            7 => MODE1_A::WIREDORPULLDOWN,
            8 => MODE1_A::WIREDAND,
            9 => MODE1_A::WIREDANDFILTER,
            10 => MODE1_A::WIREDANDPULLUP,
            11 => MODE1_A::WIREDANDPULLUPFILTER,
            12 => MODE1_A::WIREDANDALT,
            13 => MODE1_A::WIREDANDALTFILTER,
            14 => MODE1_A::WIREDANDALTPULLUP,
            15 => MODE1_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE1_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE1_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE1_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE1_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE1_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE1_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE1_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE1_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE1_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE1_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE1_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE1_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE1_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE1_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE1_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE1`"]
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE1_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE1_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE1_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE1_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE1_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE1_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE1_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Pin 2 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE2_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "4: Push-pull output"]
    PUSHPULL,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "6: Wired-or output"]
    WIREDOR,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "8: Open-drain output"]
    WIREDAND,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl From<MODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE2_A) -> Self {
        match variant {
            MODE2_A::DISABLED => 0,
            MODE2_A::INPUT => 1,
            MODE2_A::INPUTPULL => 2,
            MODE2_A::INPUTPULLFILTER => 3,
            MODE2_A::PUSHPULL => 4,
            MODE2_A::PUSHPULLALT => 5,
            MODE2_A::WIREDOR => 6,
            MODE2_A::WIREDORPULLDOWN => 7,
            MODE2_A::WIREDAND => 8,
            MODE2_A::WIREDANDFILTER => 9,
            MODE2_A::WIREDANDPULLUP => 10,
            MODE2_A::WIREDANDPULLUPFILTER => 11,
            MODE2_A::WIREDANDALT => 12,
            MODE2_A::WIREDANDALTFILTER => 13,
            MODE2_A::WIREDANDALTPULLUP => 14,
            MODE2_A::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = "Reader of field `MODE2`"]
pub type MODE2_R = crate::R<u8, MODE2_A>;
impl MODE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE2_A {
        match self.bits {
            0 => MODE2_A::DISABLED,
            1 => MODE2_A::INPUT,
            2 => MODE2_A::INPUTPULL,
            3 => MODE2_A::INPUTPULLFILTER,
            4 => MODE2_A::PUSHPULL,
            5 => MODE2_A::PUSHPULLALT,
            6 => MODE2_A::WIREDOR,
            7 => MODE2_A::WIREDORPULLDOWN,
            8 => MODE2_A::WIREDAND,
            9 => MODE2_A::WIREDANDFILTER,
            10 => MODE2_A::WIREDANDPULLUP,
            11 => MODE2_A::WIREDANDPULLUPFILTER,
            12 => MODE2_A::WIREDANDALT,
            13 => MODE2_A::WIREDANDALTFILTER,
            14 => MODE2_A::WIREDANDALTPULLUP,
            15 => MODE2_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE2_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE2_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE2_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE2_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE2_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE2_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE2_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE2_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE2_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE2_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE2_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE2_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE2_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE2_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE2_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE2`"]
pub struct MODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE2_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE2_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE2_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE2_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE2_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE2_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE2_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE2_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE2_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE2_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE2_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE2_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE2_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE2_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE2_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE2_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Pin 3 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE3_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "4: Push-pull output"]
    PUSHPULL,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "6: Wired-or output"]
    WIREDOR,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "8: Open-drain output"]
    WIREDAND,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl From<MODE3_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE3_A) -> Self {
        match variant {
            MODE3_A::DISABLED => 0,
            MODE3_A::INPUT => 1,
            MODE3_A::INPUTPULL => 2,
            MODE3_A::INPUTPULLFILTER => 3,
            MODE3_A::PUSHPULL => 4,
            MODE3_A::PUSHPULLALT => 5,
            MODE3_A::WIREDOR => 6,
            MODE3_A::WIREDORPULLDOWN => 7,
            MODE3_A::WIREDAND => 8,
            MODE3_A::WIREDANDFILTER => 9,
            MODE3_A::WIREDANDPULLUP => 10,
            MODE3_A::WIREDANDPULLUPFILTER => 11,
            MODE3_A::WIREDANDALT => 12,
            MODE3_A::WIREDANDALTFILTER => 13,
            MODE3_A::WIREDANDALTPULLUP => 14,
            MODE3_A::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = "Reader of field `MODE3`"]
pub type MODE3_R = crate::R<u8, MODE3_A>;
impl MODE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE3_A {
        match self.bits {
            0 => MODE3_A::DISABLED,
            1 => MODE3_A::INPUT,
            2 => MODE3_A::INPUTPULL,
            3 => MODE3_A::INPUTPULLFILTER,
            4 => MODE3_A::PUSHPULL,
            5 => MODE3_A::PUSHPULLALT,
            6 => MODE3_A::WIREDOR,
            7 => MODE3_A::WIREDORPULLDOWN,
            8 => MODE3_A::WIREDAND,
            9 => MODE3_A::WIREDANDFILTER,
            10 => MODE3_A::WIREDANDPULLUP,
            11 => MODE3_A::WIREDANDPULLUPFILTER,
            12 => MODE3_A::WIREDANDALT,
            13 => MODE3_A::WIREDANDALTFILTER,
            14 => MODE3_A::WIREDANDALTPULLUP,
            15 => MODE3_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE3_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE3_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE3_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE3_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE3_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE3_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE3_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE3_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE3_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE3_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE3_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE3_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE3_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE3_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE3_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE3`"]
pub struct MODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE3_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE3_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE3_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE3_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE3_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE3_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE3_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE3_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE3_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE3_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE3_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE3_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE3_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE3_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE3_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE3_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Pin 4 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE4_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "4: Push-pull output"]
    PUSHPULL,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "6: Wired-or output"]
    WIREDOR,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "8: Open-drain output"]
    WIREDAND,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl From<MODE4_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE4_A) -> Self {
        match variant {
            MODE4_A::DISABLED => 0,
            MODE4_A::INPUT => 1,
            MODE4_A::INPUTPULL => 2,
            MODE4_A::INPUTPULLFILTER => 3,
            MODE4_A::PUSHPULL => 4,
            MODE4_A::PUSHPULLALT => 5,
            MODE4_A::WIREDOR => 6,
            MODE4_A::WIREDORPULLDOWN => 7,
            MODE4_A::WIREDAND => 8,
            MODE4_A::WIREDANDFILTER => 9,
            MODE4_A::WIREDANDPULLUP => 10,
            MODE4_A::WIREDANDPULLUPFILTER => 11,
            MODE4_A::WIREDANDALT => 12,
            MODE4_A::WIREDANDALTFILTER => 13,
            MODE4_A::WIREDANDALTPULLUP => 14,
            MODE4_A::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = "Reader of field `MODE4`"]
pub type MODE4_R = crate::R<u8, MODE4_A>;
impl MODE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE4_A {
        match self.bits {
            0 => MODE4_A::DISABLED,
            1 => MODE4_A::INPUT,
            2 => MODE4_A::INPUTPULL,
            3 => MODE4_A::INPUTPULLFILTER,
            4 => MODE4_A::PUSHPULL,
            5 => MODE4_A::PUSHPULLALT,
            6 => MODE4_A::WIREDOR,
            7 => MODE4_A::WIREDORPULLDOWN,
            8 => MODE4_A::WIREDAND,
            9 => MODE4_A::WIREDANDFILTER,
            10 => MODE4_A::WIREDANDPULLUP,
            11 => MODE4_A::WIREDANDPULLUPFILTER,
            12 => MODE4_A::WIREDANDALT,
            13 => MODE4_A::WIREDANDALTFILTER,
            14 => MODE4_A::WIREDANDALTPULLUP,
            15 => MODE4_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE4_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE4_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE4_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE4_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE4_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE4_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE4_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE4_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE4_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE4_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE4_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE4_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE4_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE4_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE4_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE4`"]
pub struct MODE4_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE4_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE4_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE4_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE4_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE4_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE4_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE4_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE4_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE4_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE4_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE4_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE4_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE4_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE4_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE4_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE4_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Pin 5 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE5_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "4: Push-pull output"]
    PUSHPULL,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "6: Wired-or output"]
    WIREDOR,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "8: Open-drain output"]
    WIREDAND,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl From<MODE5_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE5_A) -> Self {
        match variant {
            MODE5_A::DISABLED => 0,
            MODE5_A::INPUT => 1,
            MODE5_A::INPUTPULL => 2,
            MODE5_A::INPUTPULLFILTER => 3,
            MODE5_A::PUSHPULL => 4,
            MODE5_A::PUSHPULLALT => 5,
            MODE5_A::WIREDOR => 6,
            MODE5_A::WIREDORPULLDOWN => 7,
            MODE5_A::WIREDAND => 8,
            MODE5_A::WIREDANDFILTER => 9,
            MODE5_A::WIREDANDPULLUP => 10,
            MODE5_A::WIREDANDPULLUPFILTER => 11,
            MODE5_A::WIREDANDALT => 12,
            MODE5_A::WIREDANDALTFILTER => 13,
            MODE5_A::WIREDANDALTPULLUP => 14,
            MODE5_A::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = "Reader of field `MODE5`"]
pub type MODE5_R = crate::R<u8, MODE5_A>;
impl MODE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE5_A {
        match self.bits {
            0 => MODE5_A::DISABLED,
            1 => MODE5_A::INPUT,
            2 => MODE5_A::INPUTPULL,
            3 => MODE5_A::INPUTPULLFILTER,
            4 => MODE5_A::PUSHPULL,
            5 => MODE5_A::PUSHPULLALT,
            6 => MODE5_A::WIREDOR,
            7 => MODE5_A::WIREDORPULLDOWN,
            8 => MODE5_A::WIREDAND,
            9 => MODE5_A::WIREDANDFILTER,
            10 => MODE5_A::WIREDANDPULLUP,
            11 => MODE5_A::WIREDANDPULLUPFILTER,
            12 => MODE5_A::WIREDANDALT,
            13 => MODE5_A::WIREDANDALTFILTER,
            14 => MODE5_A::WIREDANDALTPULLUP,
            15 => MODE5_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE5_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE5_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE5_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE5_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE5_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE5_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE5_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE5_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE5_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE5_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE5_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE5_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE5_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE5_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE5_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE5`"]
pub struct MODE5_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE5_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE5_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE5_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE5_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE5_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE5_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE5_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE5_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE5_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE5_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE5_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE5_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE5_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE5_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE5_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE5_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Pin 6 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE6_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "4: Push-pull output"]
    PUSHPULL,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "6: Wired-or output"]
    WIREDOR,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "8: Open-drain output"]
    WIREDAND,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl From<MODE6_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE6_A) -> Self {
        match variant {
            MODE6_A::DISABLED => 0,
            MODE6_A::INPUT => 1,
            MODE6_A::INPUTPULL => 2,
            MODE6_A::INPUTPULLFILTER => 3,
            MODE6_A::PUSHPULL => 4,
            MODE6_A::PUSHPULLALT => 5,
            MODE6_A::WIREDOR => 6,
            MODE6_A::WIREDORPULLDOWN => 7,
            MODE6_A::WIREDAND => 8,
            MODE6_A::WIREDANDFILTER => 9,
            MODE6_A::WIREDANDPULLUP => 10,
            MODE6_A::WIREDANDPULLUPFILTER => 11,
            MODE6_A::WIREDANDALT => 12,
            MODE6_A::WIREDANDALTFILTER => 13,
            MODE6_A::WIREDANDALTPULLUP => 14,
            MODE6_A::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = "Reader of field `MODE6`"]
pub type MODE6_R = crate::R<u8, MODE6_A>;
impl MODE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE6_A {
        match self.bits {
            0 => MODE6_A::DISABLED,
            1 => MODE6_A::INPUT,
            2 => MODE6_A::INPUTPULL,
            3 => MODE6_A::INPUTPULLFILTER,
            4 => MODE6_A::PUSHPULL,
            5 => MODE6_A::PUSHPULLALT,
            6 => MODE6_A::WIREDOR,
            7 => MODE6_A::WIREDORPULLDOWN,
            8 => MODE6_A::WIREDAND,
            9 => MODE6_A::WIREDANDFILTER,
            10 => MODE6_A::WIREDANDPULLUP,
            11 => MODE6_A::WIREDANDPULLUPFILTER,
            12 => MODE6_A::WIREDANDALT,
            13 => MODE6_A::WIREDANDALTFILTER,
            14 => MODE6_A::WIREDANDALTPULLUP,
            15 => MODE6_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE6_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE6_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE6_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE6_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE6_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE6_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE6_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE6_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE6_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE6_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE6_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE6_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE6_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE6_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE6_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE6`"]
pub struct MODE6_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE6_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE6_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE6_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE6_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE6_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE6_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE6_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE6_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE6_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE6_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE6_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE6_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE6_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE6_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE6_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE6_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Pin 7 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE7_A {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    DISABLED,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    INPUT,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    INPUTPULL,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    INPUTPULLFILTER,
    #[doc = "4: Push-pull output"]
    PUSHPULL,
    #[doc = "5: Push-pull using alternate control"]
    PUSHPULLALT,
    #[doc = "6: Wired-or output"]
    WIREDOR,
    #[doc = "7: Wired-or output with pull-down"]
    WIREDORPULLDOWN,
    #[doc = "8: Open-drain output"]
    WIREDAND,
    #[doc = "9: Open-drain output with filter"]
    WIREDANDFILTER,
    #[doc = "10: Open-drain output with pullup"]
    WIREDANDPULLUP,
    #[doc = "11: Open-drain output with filter and pullup"]
    WIREDANDPULLUPFILTER,
    #[doc = "12: Open-drain output using alternate control"]
    WIREDANDALT,
    #[doc = "13: Open-drain output using alternate control with filter"]
    WIREDANDALTFILTER,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    WIREDANDALTPULLUP,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    WIREDANDALTPULLUPFILTER,
}
impl From<MODE7_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE7_A) -> Self {
        match variant {
            MODE7_A::DISABLED => 0,
            MODE7_A::INPUT => 1,
            MODE7_A::INPUTPULL => 2,
            MODE7_A::INPUTPULLFILTER => 3,
            MODE7_A::PUSHPULL => 4,
            MODE7_A::PUSHPULLALT => 5,
            MODE7_A::WIREDOR => 6,
            MODE7_A::WIREDORPULLDOWN => 7,
            MODE7_A::WIREDAND => 8,
            MODE7_A::WIREDANDFILTER => 9,
            MODE7_A::WIREDANDPULLUP => 10,
            MODE7_A::WIREDANDPULLUPFILTER => 11,
            MODE7_A::WIREDANDALT => 12,
            MODE7_A::WIREDANDALTFILTER => 13,
            MODE7_A::WIREDANDALTPULLUP => 14,
            MODE7_A::WIREDANDALTPULLUPFILTER => 15,
        }
    }
}
#[doc = "Reader of field `MODE7`"]
pub type MODE7_R = crate::R<u8, MODE7_A>;
impl MODE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE7_A {
        match self.bits {
            0 => MODE7_A::DISABLED,
            1 => MODE7_A::INPUT,
            2 => MODE7_A::INPUTPULL,
            3 => MODE7_A::INPUTPULLFILTER,
            4 => MODE7_A::PUSHPULL,
            5 => MODE7_A::PUSHPULLALT,
            6 => MODE7_A::WIREDOR,
            7 => MODE7_A::WIREDORPULLDOWN,
            8 => MODE7_A::WIREDAND,
            9 => MODE7_A::WIREDANDFILTER,
            10 => MODE7_A::WIREDANDPULLUP,
            11 => MODE7_A::WIREDANDPULLUPFILTER,
            12 => MODE7_A::WIREDANDALT,
            13 => MODE7_A::WIREDANDALTFILTER,
            14 => MODE7_A::WIREDANDALTPULLUP,
            15 => MODE7_A::WIREDANDALTPULLUPFILTER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE7_A::INPUT
    }
    #[doc = "Checks if the value of the field is `INPUTPULL`"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE7_A::INPUTPULL
    }
    #[doc = "Checks if the value of the field is `INPUTPULLFILTER`"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE7_A::INPUTPULLFILTER
    }
    #[doc = "Checks if the value of the field is `PUSHPULL`"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE7_A::PUSHPULL
    }
    #[doc = "Checks if the value of the field is `PUSHPULLALT`"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE7_A::PUSHPULLALT
    }
    #[doc = "Checks if the value of the field is `WIREDOR`"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE7_A::WIREDOR
    }
    #[doc = "Checks if the value of the field is `WIREDORPULLDOWN`"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE7_A::WIREDORPULLDOWN
    }
    #[doc = "Checks if the value of the field is `WIREDAND`"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE7_A::WIREDAND
    }
    #[doc = "Checks if the value of the field is `WIREDANDFILTER`"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE7_A::WIREDANDFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE7_A::WIREDANDPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE7_A::WIREDANDPULLUPFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALT`"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE7_A::WIREDANDALT
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE7_A::WIREDANDALTFILTER
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUP`"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE7_A::WIREDANDALTPULLUP
    }
    #[doc = "Checks if the value of the field is `WIREDANDALTPULLUPFILTER`"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE7_A::WIREDANDALTPULLUPFILTER
    }
}
#[doc = "Write proxy for field `MODE7`"]
pub struct MODE7_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE7_A::DISABLED)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(MODE7_A::INPUT)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut W {
        self.variant(MODE7_A::INPUTPULL)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut W {
        self.variant(MODE7_A::INPUTPULLFILTER)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(MODE7_A::PUSHPULL)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut W {
        self.variant(MODE7_A::PUSHPULLALT)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut W {
        self.variant(MODE7_A::WIREDOR)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut W {
        self.variant(MODE7_A::WIREDORPULLDOWN)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut W {
        self.variant(MODE7_A::WIREDAND)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut W {
        self.variant(MODE7_A::WIREDANDFILTER)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut W {
        self.variant(MODE7_A::WIREDANDPULLUP)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut W {
        self.variant(MODE7_A::WIREDANDPULLUPFILTER)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut W {
        self.variant(MODE7_A::WIREDANDALT)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut W {
        self.variant(MODE7_A::WIREDANDALTFILTER)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut W {
        self.variant(MODE7_A::WIREDANDALTPULLUP)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut W {
        self.variant(MODE7_A::WIREDANDALTPULLUPFILTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Pin 0 Mode"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pin 1 Mode"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pin 2 Mode"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pin 3 Mode"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Pin 4 Mode"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Pin 5 Mode"]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Pin 6 Mode"]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Pin 7 Mode"]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin 0 Mode"]
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE0_W {
        MODE0_W { w: self }
    }
    #[doc = "Bits 4:7 - Pin 1 Mode"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    #[doc = "Bits 8:11 - Pin 2 Mode"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W { w: self }
    }
    #[doc = "Bits 12:15 - Pin 3 Mode"]
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W {
        MODE3_W { w: self }
    }
    #[doc = "Bits 16:19 - Pin 4 Mode"]
    #[inline(always)]
    pub fn mode4(&mut self) -> MODE4_W {
        MODE4_W { w: self }
    }
    #[doc = "Bits 20:23 - Pin 5 Mode"]
    #[inline(always)]
    pub fn mode5(&mut self) -> MODE5_W {
        MODE5_W { w: self }
    }
    #[doc = "Bits 24:27 - Pin 6 Mode"]
    #[inline(always)]
    pub fn mode6(&mut self) -> MODE6_W {
        MODE6_W { w: self }
    }
    #[doc = "Bits 28:31 - Pin 7 Mode"]
    #[inline(always)]
    pub fn mode7(&mut self) -> MODE7_W {
        MODE7_W { w: self }
    }
}
