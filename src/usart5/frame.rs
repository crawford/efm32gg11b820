#[doc = "Reader of register FRAME"]
pub type R = crate::R<u32, super::FRAME>;
#[doc = "Writer for register FRAME"]
pub type W = crate::W<u32, super::FRAME>;
#[doc = "Register FRAME `reset()`'s with value 0x1005"]
impl crate::ResetValue for super::FRAME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1005
    }
}
#[doc = "Possible values of the field `DATABITS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATABITS_A {
    #[doc = "Each frame contains 4 data bits"]
    FOUR,
    #[doc = "Each frame contains 5 data bits"]
    FIVE,
    #[doc = "Each frame contains 6 data bits"]
    SIX,
    #[doc = "Each frame contains 7 data bits"]
    SEVEN,
    #[doc = "Each frame contains 8 data bits"]
    EIGHT,
    #[doc = "Each frame contains 9 data bits"]
    NINE,
    #[doc = "Each frame contains 10 data bits"]
    TEN,
    #[doc = "Each frame contains 11 data bits"]
    ELEVEN,
    #[doc = "Each frame contains 12 data bits"]
    TWELVE,
    #[doc = "Each frame contains 13 data bits"]
    THIRTEEN,
    #[doc = "Each frame contains 14 data bits"]
    FOURTEEN,
    #[doc = "Each frame contains 15 data bits"]
    FIFTEEN,
    #[doc = "Each frame contains 16 data bits"]
    SIXTEEN,
}
impl crate::ToBits<u8> for DATABITS_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DATABITS_A::FOUR => 1,
            DATABITS_A::FIVE => 2,
            DATABITS_A::SIX => 3,
            DATABITS_A::SEVEN => 4,
            DATABITS_A::EIGHT => 5,
            DATABITS_A::NINE => 6,
            DATABITS_A::TEN => 7,
            DATABITS_A::ELEVEN => 8,
            DATABITS_A::TWELVE => 9,
            DATABITS_A::THIRTEEN => 10,
            DATABITS_A::FOURTEEN => 11,
            DATABITS_A::FIFTEEN => 12,
            DATABITS_A::SIXTEEN => 13,
        }
    }
}
#[doc = "Reader of field `DATABITS`"]
pub type DATABITS_R = crate::R<u8, DATABITS_A>;
impl DATABITS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATABITS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(DATABITS_A::FOUR),
            2 => Val(DATABITS_A::FIVE),
            3 => Val(DATABITS_A::SIX),
            4 => Val(DATABITS_A::SEVEN),
            5 => Val(DATABITS_A::EIGHT),
            6 => Val(DATABITS_A::NINE),
            7 => Val(DATABITS_A::TEN),
            8 => Val(DATABITS_A::ELEVEN),
            9 => Val(DATABITS_A::TWELVE),
            10 => Val(DATABITS_A::THIRTEEN),
            11 => Val(DATABITS_A::FOURTEEN),
            12 => Val(DATABITS_A::FIFTEEN),
            13 => Val(DATABITS_A::SIXTEEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == DATABITS_A::FOUR
    }
    #[doc = "Checks if the value of the field is `FIVE`"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == DATABITS_A::FIVE
    }
    #[doc = "Checks if the value of the field is `SIX`"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == DATABITS_A::SIX
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == DATABITS_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == DATABITS_A::EIGHT
    }
    #[doc = "Checks if the value of the field is `NINE`"]
    #[inline(always)]
    pub fn is_nine(&self) -> bool {
        *self == DATABITS_A::NINE
    }
    #[doc = "Checks if the value of the field is `TEN`"]
    #[inline(always)]
    pub fn is_ten(&self) -> bool {
        *self == DATABITS_A::TEN
    }
    #[doc = "Checks if the value of the field is `ELEVEN`"]
    #[inline(always)]
    pub fn is_eleven(&self) -> bool {
        *self == DATABITS_A::ELEVEN
    }
    #[doc = "Checks if the value of the field is `TWELVE`"]
    #[inline(always)]
    pub fn is_twelve(&self) -> bool {
        *self == DATABITS_A::TWELVE
    }
    #[doc = "Checks if the value of the field is `THIRTEEN`"]
    #[inline(always)]
    pub fn is_thirteen(&self) -> bool {
        *self == DATABITS_A::THIRTEEN
    }
    #[doc = "Checks if the value of the field is `FOURTEEN`"]
    #[inline(always)]
    pub fn is_fourteen(&self) -> bool {
        *self == DATABITS_A::FOURTEEN
    }
    #[doc = "Checks if the value of the field is `FIFTEEN`"]
    #[inline(always)]
    pub fn is_fifteen(&self) -> bool {
        *self == DATABITS_A::FIFTEEN
    }
    #[doc = "Checks if the value of the field is `SIXTEEN`"]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == DATABITS_A::SIXTEEN
    }
}
#[doc = "Write proxy for field `DATABITS`"]
pub struct DATABITS_W<'a> {
    w: &'a mut W,
}
impl<'a> DATABITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATABITS_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Each frame contains 4 data bits"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(DATABITS_A::FOUR)
    }
    #[doc = "Each frame contains 5 data bits"]
    #[inline(always)]
    pub fn five(self) -> &'a mut W {
        self.variant(DATABITS_A::FIVE)
    }
    #[doc = "Each frame contains 6 data bits"]
    #[inline(always)]
    pub fn six(self) -> &'a mut W {
        self.variant(DATABITS_A::SIX)
    }
    #[doc = "Each frame contains 7 data bits"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(DATABITS_A::SEVEN)
    }
    #[doc = "Each frame contains 8 data bits"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(DATABITS_A::EIGHT)
    }
    #[doc = "Each frame contains 9 data bits"]
    #[inline(always)]
    pub fn nine(self) -> &'a mut W {
        self.variant(DATABITS_A::NINE)
    }
    #[doc = "Each frame contains 10 data bits"]
    #[inline(always)]
    pub fn ten(self) -> &'a mut W {
        self.variant(DATABITS_A::TEN)
    }
    #[doc = "Each frame contains 11 data bits"]
    #[inline(always)]
    pub fn eleven(self) -> &'a mut W {
        self.variant(DATABITS_A::ELEVEN)
    }
    #[doc = "Each frame contains 12 data bits"]
    #[inline(always)]
    pub fn twelve(self) -> &'a mut W {
        self.variant(DATABITS_A::TWELVE)
    }
    #[doc = "Each frame contains 13 data bits"]
    #[inline(always)]
    pub fn thirteen(self) -> &'a mut W {
        self.variant(DATABITS_A::THIRTEEN)
    }
    #[doc = "Each frame contains 14 data bits"]
    #[inline(always)]
    pub fn fourteen(self) -> &'a mut W {
        self.variant(DATABITS_A::FOURTEEN)
    }
    #[doc = "Each frame contains 15 data bits"]
    #[inline(always)]
    pub fn fifteen(self) -> &'a mut W {
        self.variant(DATABITS_A::FIFTEEN)
    }
    #[doc = "Each frame contains 16 data bits"]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut W {
        self.variant(DATABITS_A::SIXTEEN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Possible values of the field `PARITY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITY_A {
    #[doc = "Parity bits are not used"]
    NONE,
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    EVEN,
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    ODD,
}
impl crate::ToBits<u8> for PARITY_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PARITY_A::NONE => 0,
            PARITY_A::EVEN => 2,
            PARITY_A::ODD => 3,
        }
    }
}
#[doc = "Reader of field `PARITY`"]
pub type PARITY_R = crate::R<u8, PARITY_A>;
impl PARITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PARITY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PARITY_A::NONE),
            2 => Val(PARITY_A::EVEN),
            3 => Val(PARITY_A::ODD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PARITY_A::NONE
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARITY_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARITY_A::ODD
    }
}
#[doc = "Write proxy for field `PARITY`"]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITY_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PARITY_A::NONE)
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PARITY_A::EVEN)
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARITY_A::ODD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `STOPBITS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPBITS_A {
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    HALF,
    #[doc = "One stop bit is generated and verified"]
    ONE,
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    ONEANDAHALF,
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    TWO,
}
impl crate::ToBits<u8> for STOPBITS_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            STOPBITS_A::HALF => 0,
            STOPBITS_A::ONE => 1,
            STOPBITS_A::ONEANDAHALF => 2,
            STOPBITS_A::TWO => 3,
        }
    }
}
#[doc = "Reader of field `STOPBITS`"]
pub type STOPBITS_R = crate::R<u8, STOPBITS_A>;
impl STOPBITS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPBITS_A {
        match self.bits {
            0 => STOPBITS_A::HALF,
            1 => STOPBITS_A::ONE,
            2 => STOPBITS_A::ONEANDAHALF,
            3 => STOPBITS_A::TWO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == STOPBITS_A::HALF
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == STOPBITS_A::ONE
    }
    #[doc = "Checks if the value of the field is `ONEANDAHALF`"]
    #[inline(always)]
    pub fn is_oneandahalf(&self) -> bool {
        *self == STOPBITS_A::ONEANDAHALF
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == STOPBITS_A::TWO
    }
}
#[doc = "Write proxy for field `STOPBITS`"]
pub struct STOPBITS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPBITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPBITS_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(STOPBITS_A::HALF)
    }
    #[doc = "One stop bit is generated and verified"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(STOPBITS_A::ONE)
    }
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    #[inline(always)]
    pub fn oneandahalf(self) -> &'a mut W {
        self.variant(STOPBITS_A::ONEANDAHALF)
    }
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(STOPBITS_A::TWO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&self) -> DATABITS_R {
        DATABITS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&self) -> STOPBITS_R {
        STOPBITS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&mut self) -> DATABITS_W {
        DATABITS_W { w: self }
    }
    #[doc = "Bits 8:9 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
    #[doc = "Bits 12:13 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&mut self) -> STOPBITS_W {
        STOPBITS_W { w: self }
    }
}
