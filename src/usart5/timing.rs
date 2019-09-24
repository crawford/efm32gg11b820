#[doc = "Reader of register TIMING"]
pub type R = crate::R<u32, super::TIMING>;
#[doc = "Writer for register TIMING"]
pub type W = crate::W<u32, super::TIMING>;
#[doc = "Register TIMING `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TX Frame Start Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDELAY_A {
    #[doc = "0: Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    DISABLE,
    #[doc = "1: Start of transmission is delayed for 1 baud-times"]
    ONE,
    #[doc = "2: Start of transmission is delayed for 2 baud-times"]
    TWO,
    #[doc = "3: Start of transmission is delayed for 3 baud-times"]
    THREE,
    #[doc = "4: Start of transmission is delayed for 7 baud-times"]
    SEVEN,
    #[doc = "5: Start of transmission is delayed for TCMPVAL0 baud-times"]
    TCMP0,
    #[doc = "6: Start of transmission is delayed for TCMPVAL1 baud-times"]
    TCMP1,
    #[doc = "7: Start of transmission is delayed for TCMPVAL2 baud-times"]
    TCMP2,
}
impl From<TXDELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: TXDELAY_A) -> Self {
        match variant {
            TXDELAY_A::DISABLE => 0,
            TXDELAY_A::ONE => 1,
            TXDELAY_A::TWO => 2,
            TXDELAY_A::THREE => 3,
            TXDELAY_A::SEVEN => 4,
            TXDELAY_A::TCMP0 => 5,
            TXDELAY_A::TCMP1 => 6,
            TXDELAY_A::TCMP2 => 7,
        }
    }
}
#[doc = "Reader of field `TXDELAY`"]
pub type TXDELAY_R = crate::R<u8, TXDELAY_A>;
impl TXDELAY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDELAY_A {
        match self.bits {
            0 => TXDELAY_A::DISABLE,
            1 => TXDELAY_A::ONE,
            2 => TXDELAY_A::TWO,
            3 => TXDELAY_A::THREE,
            4 => TXDELAY_A::SEVEN,
            5 => TXDELAY_A::TCMP0,
            6 => TXDELAY_A::TCMP1,
            7 => TXDELAY_A::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXDELAY_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == TXDELAY_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == TXDELAY_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == TXDELAY_A::THREE
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == TXDELAY_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == TXDELAY_A::TCMP0
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == TXDELAY_A::TCMP1
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == TXDELAY_A::TCMP2
    }
}
#[doc = "Write proxy for field `TXDELAY`"]
pub struct TXDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDELAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDELAY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXDELAY_A::DISABLE)
    }
    #[doc = "Start of transmission is delayed for 1 baud-times"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(TXDELAY_A::ONE)
    }
    #[doc = "Start of transmission is delayed for 2 baud-times"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(TXDELAY_A::TWO)
    }
    #[doc = "Start of transmission is delayed for 3 baud-times"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(TXDELAY_A::THREE)
    }
    #[doc = "Start of transmission is delayed for 7 baud-times"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(TXDELAY_A::SEVEN)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(TXDELAY_A::TCMP0)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(TXDELAY_A::TCMP1)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(TXDELAY_A::TCMP2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Chip Select Setup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSETUP_A {
    #[doc = "0: CS is not asserted before start of transmission"]
    ZERO,
    #[doc = "1: CS is asserted for 1 baud-times before start of transmission"]
    ONE,
    #[doc = "2: CS is asserted for 2 baud-times before start of transmission"]
    TWO,
    #[doc = "3: CS is asserted for 3 baud-times before start of transmission"]
    THREE,
    #[doc = "4: CS is asserted for 7 baud-times before start of transmission"]
    SEVEN,
    #[doc = "5: CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    TCMP0,
    #[doc = "6: CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    TCMP1,
    #[doc = "7: CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    TCMP2,
}
impl From<CSSETUP_A> for u8 {
    #[inline(always)]
    fn from(variant: CSSETUP_A) -> Self {
        match variant {
            CSSETUP_A::ZERO => 0,
            CSSETUP_A::ONE => 1,
            CSSETUP_A::TWO => 2,
            CSSETUP_A::THREE => 3,
            CSSETUP_A::SEVEN => 4,
            CSSETUP_A::TCMP0 => 5,
            CSSETUP_A::TCMP1 => 6,
            CSSETUP_A::TCMP2 => 7,
        }
    }
}
#[doc = "Reader of field `CSSETUP`"]
pub type CSSETUP_R = crate::R<u8, CSSETUP_A>;
impl CSSETUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSSETUP_A {
        match self.bits {
            0 => CSSETUP_A::ZERO,
            1 => CSSETUP_A::ONE,
            2 => CSSETUP_A::TWO,
            3 => CSSETUP_A::THREE,
            4 => CSSETUP_A::SEVEN,
            5 => CSSETUP_A::TCMP0,
            6 => CSSETUP_A::TCMP1,
            7 => CSSETUP_A::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSSETUP_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSSETUP_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSSETUP_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CSSETUP_A::THREE
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == CSSETUP_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == CSSETUP_A::TCMP0
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == CSSETUP_A::TCMP1
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == CSSETUP_A::TCMP2
    }
}
#[doc = "Write proxy for field `CSSETUP`"]
pub struct CSSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSETUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSSETUP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CS is not asserted before start of transmission"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CSSETUP_A::ZERO)
    }
    #[doc = "CS is asserted for 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CSSETUP_A::ONE)
    }
    #[doc = "CS is asserted for 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(CSSETUP_A::TWO)
    }
    #[doc = "CS is asserted for 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(CSSETUP_A::THREE)
    }
    #[doc = "CS is asserted for 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(CSSETUP_A::SEVEN)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(CSSETUP_A::TCMP0)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(CSSETUP_A::TCMP1)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(CSSETUP_A::TCMP2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Inter-character Spacing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICS_A {
    #[doc = "0: There is no space between charcters"]
    ZERO,
    #[doc = "1: Create a space of 1 baud-times before start of transmission "]
    ONE,
    #[doc = "2: Create a space of 2 baud-times before start of transmission"]
    TWO,
    #[doc = "3: Create a space of 3 baud-times before start of transmission"]
    THREE,
    #[doc = "4: Create a space of 7 baud-times before start of transmission"]
    SEVEN,
    #[doc = "5: Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    TCMP0,
    #[doc = "6: Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    TCMP1,
    #[doc = "7: Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    TCMP2,
}
impl From<ICS_A> for u8 {
    #[inline(always)]
    fn from(variant: ICS_A) -> Self {
        match variant {
            ICS_A::ZERO => 0,
            ICS_A::ONE => 1,
            ICS_A::TWO => 2,
            ICS_A::THREE => 3,
            ICS_A::SEVEN => 4,
            ICS_A::TCMP0 => 5,
            ICS_A::TCMP1 => 6,
            ICS_A::TCMP2 => 7,
        }
    }
}
#[doc = "Reader of field `ICS`"]
pub type ICS_R = crate::R<u8, ICS_A>;
impl ICS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICS_A {
        match self.bits {
            0 => ICS_A::ZERO,
            1 => ICS_A::ONE,
            2 => ICS_A::TWO,
            3 => ICS_A::THREE,
            4 => ICS_A::SEVEN,
            5 => ICS_A::TCMP0,
            6 => ICS_A::TCMP1,
            7 => ICS_A::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ICS_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ICS_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == ICS_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == ICS_A::THREE
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == ICS_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == ICS_A::TCMP0
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == ICS_A::TCMP1
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == ICS_A::TCMP2
    }
}
#[doc = "Write proxy for field `ICS`"]
pub struct ICS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "There is no space between charcters"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(ICS_A::ZERO)
    }
    #[doc = "Create a space of 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ICS_A::ONE)
    }
    #[doc = "Create a space of 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(ICS_A::TWO)
    }
    #[doc = "Create a space of 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(ICS_A::THREE)
    }
    #[doc = "Create a space of 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(ICS_A::SEVEN)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(ICS_A::TCMP0)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(ICS_A::TCMP1)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(ICS_A::TCMP2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Chip Select Hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSHOLD_A {
    #[doc = "0: Disable CS being asserted after the end of transmission"]
    ZERO,
    #[doc = "1: CS is asserted for 1 baud-times after the end of transmission"]
    ONE,
    #[doc = "2: CS is asserted for 2 baud-times after the end of transmission"]
    TWO,
    #[doc = "3: CS is asserted for 3 baud-times after the end of transmission"]
    THREE,
    #[doc = "4: CS is asserted for 7 baud-times after the end of transmission"]
    SEVEN,
    #[doc = "5: CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    TCMP0,
    #[doc = "6: CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    TCMP1,
    #[doc = "7: CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    TCMP2,
}
impl From<CSHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: CSHOLD_A) -> Self {
        match variant {
            CSHOLD_A::ZERO => 0,
            CSHOLD_A::ONE => 1,
            CSHOLD_A::TWO => 2,
            CSHOLD_A::THREE => 3,
            CSHOLD_A::SEVEN => 4,
            CSHOLD_A::TCMP0 => 5,
            CSHOLD_A::TCMP1 => 6,
            CSHOLD_A::TCMP2 => 7,
        }
    }
}
#[doc = "Reader of field `CSHOLD`"]
pub type CSHOLD_R = crate::R<u8, CSHOLD_A>;
impl CSHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSHOLD_A {
        match self.bits {
            0 => CSHOLD_A::ZERO,
            1 => CSHOLD_A::ONE,
            2 => CSHOLD_A::TWO,
            3 => CSHOLD_A::THREE,
            4 => CSHOLD_A::SEVEN,
            5 => CSHOLD_A::TCMP0,
            6 => CSHOLD_A::TCMP1,
            7 => CSHOLD_A::TCMP2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CSHOLD_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CSHOLD_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CSHOLD_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CSHOLD_A::THREE
    }
    #[doc = "Checks if the value of the field is `SEVEN`"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == CSHOLD_A::SEVEN
    }
    #[doc = "Checks if the value of the field is `TCMP0`"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == CSHOLD_A::TCMP0
    }
    #[doc = "Checks if the value of the field is `TCMP1`"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == CSHOLD_A::TCMP1
    }
    #[doc = "Checks if the value of the field is `TCMP2`"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == CSHOLD_A::TCMP2
    }
}
#[doc = "Write proxy for field `CSHOLD`"]
pub struct CSHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CSHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSHOLD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disable CS being asserted after the end of transmission"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CSHOLD_A::ZERO)
    }
    #[doc = "CS is asserted for 1 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CSHOLD_A::ONE)
    }
    #[doc = "CS is asserted for 2 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(CSHOLD_A::TWO)
    }
    #[doc = "CS is asserted for 3 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(CSHOLD_A::THREE)
    }
    #[doc = "CS is asserted for 7 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut W {
        self.variant(CSHOLD_A::SEVEN)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut W {
        self.variant(CSHOLD_A::TCMP0)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut W {
        self.variant(CSHOLD_A::TCMP1)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut W {
        self.variant(CSHOLD_A::TCMP2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18 - TX Frame Start Delay"]
    #[inline(always)]
    pub fn txdelay(&self) -> TXDELAY_R {
        TXDELAY_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Chip Select Setup"]
    #[inline(always)]
    pub fn cssetup(&self) -> CSSETUP_R {
        CSSETUP_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Inter-character Spacing"]
    #[inline(always)]
    pub fn ics(&self) -> ICS_R {
        ICS_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Chip Select Hold"]
    #[inline(always)]
    pub fn cshold(&self) -> CSHOLD_R {
        CSHOLD_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - TX Frame Start Delay"]
    #[inline(always)]
    pub fn txdelay(&mut self) -> TXDELAY_W {
        TXDELAY_W { w: self }
    }
    #[doc = "Bits 20:22 - Chip Select Setup"]
    #[inline(always)]
    pub fn cssetup(&mut self) -> CSSETUP_W {
        CSSETUP_W { w: self }
    }
    #[doc = "Bits 24:26 - Inter-character Spacing"]
    #[inline(always)]
    pub fn ics(&mut self) -> ICS_W {
        ICS_W { w: self }
    }
    #[doc = "Bits 28:30 - Chip Select Hold"]
    #[inline(always)]
    pub fn cshold(&mut self) -> CSHOLD_W {
        CSHOLD_W { w: self }
    }
}
