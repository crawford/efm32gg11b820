#[doc = "Reader of register SCANNEGSEL"]
pub type R = crate::R<u32, super::SCANNEGSEL>;
#[doc = "Writer for register SCANNEGSEL"]
pub type W = crate::W<u32, super::SCANNEGSEL>;
#[doc = "Register SCANNEGSEL `reset()`'s with value 0x39e4"]
impl crate::ResetValue for super::SCANNEGSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x39e4
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT0NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT1 as negative channel input"]
    INPUT1,
    #[doc = "1: Selects ADCn_INPUT3 as negative channel input"]
    INPUT3,
    #[doc = "2: Selects ADCn_INPUT5 as negative channel input"]
    INPUT5,
    #[doc = "3: Selects ADCn_INPUT7 as negative channel input"]
    INPUT7,
}
impl From<INPUT0NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT0NEGSEL_A) -> Self {
        match variant {
            INPUT0NEGSEL_A::INPUT1 => 0,
            INPUT0NEGSEL_A::INPUT3 => 1,
            INPUT0NEGSEL_A::INPUT5 => 2,
            INPUT0NEGSEL_A::INPUT7 => 3,
        }
    }
}
#[doc = "Reader of field `INPUT0NEGSEL`"]
pub type INPUT0NEGSEL_R = crate::R<u8, INPUT0NEGSEL_A>;
impl INPUT0NEGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT0NEGSEL_A {
        match self.bits {
            0 => INPUT0NEGSEL_A::INPUT1,
            1 => INPUT0NEGSEL_A::INPUT3,
            2 => INPUT0NEGSEL_A::INPUT5,
            3 => INPUT0NEGSEL_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == INPUT0NEGSEL_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == INPUT0NEGSEL_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == INPUT0NEGSEL_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == INPUT0NEGSEL_A::INPUT7
    }
}
#[doc = "Write proxy for field `INPUT0NEGSEL`"]
pub struct INPUT0NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT0NEGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT0NEGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(INPUT0NEGSEL_A::INPUT1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(INPUT0NEGSEL_A::INPUT3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(INPUT0NEGSEL_A::INPUT5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(INPUT0NEGSEL_A::INPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT2NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT1 as negative channel input"]
    INPUT1,
    #[doc = "1: Selects ADCn_INPUT3 as negative channel input"]
    INPUT3,
    #[doc = "2: Selects ADCn_INPUT5 as negative channel input"]
    INPUT5,
    #[doc = "3: Selects ADCn_INPUT7 as negative channel input"]
    INPUT7,
}
impl From<INPUT2NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT2NEGSEL_A) -> Self {
        match variant {
            INPUT2NEGSEL_A::INPUT1 => 0,
            INPUT2NEGSEL_A::INPUT3 => 1,
            INPUT2NEGSEL_A::INPUT5 => 2,
            INPUT2NEGSEL_A::INPUT7 => 3,
        }
    }
}
#[doc = "Reader of field `INPUT2NEGSEL`"]
pub type INPUT2NEGSEL_R = crate::R<u8, INPUT2NEGSEL_A>;
impl INPUT2NEGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT2NEGSEL_A {
        match self.bits {
            0 => INPUT2NEGSEL_A::INPUT1,
            1 => INPUT2NEGSEL_A::INPUT3,
            2 => INPUT2NEGSEL_A::INPUT5,
            3 => INPUT2NEGSEL_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == INPUT2NEGSEL_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == INPUT2NEGSEL_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == INPUT2NEGSEL_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == INPUT2NEGSEL_A::INPUT7
    }
}
#[doc = "Write proxy for field `INPUT2NEGSEL`"]
pub struct INPUT2NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT2NEGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT2NEGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(INPUT2NEGSEL_A::INPUT1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(INPUT2NEGSEL_A::INPUT3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(INPUT2NEGSEL_A::INPUT5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(INPUT2NEGSEL_A::INPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT4NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT1 as negative channel input"]
    INPUT1,
    #[doc = "1: Selects ADCn_INPUT3 as negative channel input"]
    INPUT3,
    #[doc = "2: Selects ADCn_INPUT5 as negative channel input"]
    INPUT5,
    #[doc = "3: Selects ADCn_INPUT7 as negative channel input"]
    INPUT7,
}
impl From<INPUT4NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT4NEGSEL_A) -> Self {
        match variant {
            INPUT4NEGSEL_A::INPUT1 => 0,
            INPUT4NEGSEL_A::INPUT3 => 1,
            INPUT4NEGSEL_A::INPUT5 => 2,
            INPUT4NEGSEL_A::INPUT7 => 3,
        }
    }
}
#[doc = "Reader of field `INPUT4NEGSEL`"]
pub type INPUT4NEGSEL_R = crate::R<u8, INPUT4NEGSEL_A>;
impl INPUT4NEGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT4NEGSEL_A {
        match self.bits {
            0 => INPUT4NEGSEL_A::INPUT1,
            1 => INPUT4NEGSEL_A::INPUT3,
            2 => INPUT4NEGSEL_A::INPUT5,
            3 => INPUT4NEGSEL_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == INPUT4NEGSEL_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == INPUT4NEGSEL_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == INPUT4NEGSEL_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == INPUT4NEGSEL_A::INPUT7
    }
}
#[doc = "Write proxy for field `INPUT4NEGSEL`"]
pub struct INPUT4NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT4NEGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT4NEGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(INPUT4NEGSEL_A::INPUT1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(INPUT4NEGSEL_A::INPUT3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(INPUT4NEGSEL_A::INPUT5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(INPUT4NEGSEL_A::INPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT6NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT1 as negative channel input"]
    INPUT1,
    #[doc = "1: Selects ADCn_INPUT3 as negative channel input"]
    INPUT3,
    #[doc = "2: Selects ADCn_INPUT5 as negative channel input"]
    INPUT5,
    #[doc = "3: Selects ADCn_INPUT7 as negative channel input"]
    INPUT7,
}
impl From<INPUT6NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT6NEGSEL_A) -> Self {
        match variant {
            INPUT6NEGSEL_A::INPUT1 => 0,
            INPUT6NEGSEL_A::INPUT3 => 1,
            INPUT6NEGSEL_A::INPUT5 => 2,
            INPUT6NEGSEL_A::INPUT7 => 3,
        }
    }
}
#[doc = "Reader of field `INPUT6NEGSEL`"]
pub type INPUT6NEGSEL_R = crate::R<u8, INPUT6NEGSEL_A>;
impl INPUT6NEGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT6NEGSEL_A {
        match self.bits {
            0 => INPUT6NEGSEL_A::INPUT1,
            1 => INPUT6NEGSEL_A::INPUT3,
            2 => INPUT6NEGSEL_A::INPUT5,
            3 => INPUT6NEGSEL_A::INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT1`"]
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == INPUT6NEGSEL_A::INPUT1
    }
    #[doc = "Checks if the value of the field is `INPUT3`"]
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == INPUT6NEGSEL_A::INPUT3
    }
    #[doc = "Checks if the value of the field is `INPUT5`"]
    #[inline(always)]
    pub fn is_input5(&self) -> bool {
        *self == INPUT6NEGSEL_A::INPUT5
    }
    #[doc = "Checks if the value of the field is `INPUT7`"]
    #[inline(always)]
    pub fn is_input7(&self) -> bool {
        *self == INPUT6NEGSEL_A::INPUT7
    }
}
#[doc = "Write proxy for field `INPUT6NEGSEL`"]
pub struct INPUT6NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT6NEGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT6NEGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects ADCn_INPUT1 as negative channel input"]
    #[inline(always)]
    pub fn input1(self) -> &'a mut W {
        self.variant(INPUT6NEGSEL_A::INPUT1)
    }
    #[doc = "Selects ADCn_INPUT3 as negative channel input"]
    #[inline(always)]
    pub fn input3(self) -> &'a mut W {
        self.variant(INPUT6NEGSEL_A::INPUT3)
    }
    #[doc = "Selects ADCn_INPUT5 as negative channel input"]
    #[inline(always)]
    pub fn input5(self) -> &'a mut W {
        self.variant(INPUT6NEGSEL_A::INPUT5)
    }
    #[doc = "Selects ADCn_INPUT7 as negative channel input"]
    #[inline(always)]
    pub fn input7(self) -> &'a mut W {
        self.variant(INPUT6NEGSEL_A::INPUT7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT9NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT8 as negative channel input"]
    INPUT8,
    #[doc = "1: Selects ADCn_INPUT10 as negative channel input"]
    INPUT10,
    #[doc = "2: Selects ADCn_INPUT12 as negative channel input"]
    INPUT12,
    #[doc = "3: Selects ADCn_INPUT14 as negative channel input"]
    INPUT14,
}
impl From<INPUT9NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT9NEGSEL_A) -> Self {
        match variant {
            INPUT9NEGSEL_A::INPUT8 => 0,
            INPUT9NEGSEL_A::INPUT10 => 1,
            INPUT9NEGSEL_A::INPUT12 => 2,
            INPUT9NEGSEL_A::INPUT14 => 3,
        }
    }
}
#[doc = "Reader of field `INPUT9NEGSEL`"]
pub type INPUT9NEGSEL_R = crate::R<u8, INPUT9NEGSEL_A>;
impl INPUT9NEGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT9NEGSEL_A {
        match self.bits {
            0 => INPUT9NEGSEL_A::INPUT8,
            1 => INPUT9NEGSEL_A::INPUT10,
            2 => INPUT9NEGSEL_A::INPUT12,
            3 => INPUT9NEGSEL_A::INPUT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT8`"]
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == INPUT9NEGSEL_A::INPUT8
    }
    #[doc = "Checks if the value of the field is `INPUT10`"]
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == INPUT9NEGSEL_A::INPUT10
    }
    #[doc = "Checks if the value of the field is `INPUT12`"]
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == INPUT9NEGSEL_A::INPUT12
    }
    #[doc = "Checks if the value of the field is `INPUT14`"]
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == INPUT9NEGSEL_A::INPUT14
    }
}
#[doc = "Write proxy for field `INPUT9NEGSEL`"]
pub struct INPUT9NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT9NEGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT9NEGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn input8(self) -> &'a mut W {
        self.variant(INPUT9NEGSEL_A::INPUT8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn input10(self) -> &'a mut W {
        self.variant(INPUT9NEGSEL_A::INPUT10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn input12(self) -> &'a mut W {
        self.variant(INPUT9NEGSEL_A::INPUT12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn input14(self) -> &'a mut W {
        self.variant(INPUT9NEGSEL_A::INPUT14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT11NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT8 as negative channel input"]
    INPUT8,
    #[doc = "1: Selects ADCn_INPUT10 as negative channel input"]
    INPUT10,
    #[doc = "2: Selects ADCn_INPUT12 as negative channel input"]
    INPUT12,
    #[doc = "3: Selects ADCn_INPUT14 as negative channel input"]
    INPUT14,
}
impl From<INPUT11NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT11NEGSEL_A) -> Self {
        match variant {
            INPUT11NEGSEL_A::INPUT8 => 0,
            INPUT11NEGSEL_A::INPUT10 => 1,
            INPUT11NEGSEL_A::INPUT12 => 2,
            INPUT11NEGSEL_A::INPUT14 => 3,
        }
    }
}
#[doc = "Reader of field `INPUT11NEGSEL`"]
pub type INPUT11NEGSEL_R = crate::R<u8, INPUT11NEGSEL_A>;
impl INPUT11NEGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT11NEGSEL_A {
        match self.bits {
            0 => INPUT11NEGSEL_A::INPUT8,
            1 => INPUT11NEGSEL_A::INPUT10,
            2 => INPUT11NEGSEL_A::INPUT12,
            3 => INPUT11NEGSEL_A::INPUT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT8`"]
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == INPUT11NEGSEL_A::INPUT8
    }
    #[doc = "Checks if the value of the field is `INPUT10`"]
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == INPUT11NEGSEL_A::INPUT10
    }
    #[doc = "Checks if the value of the field is `INPUT12`"]
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == INPUT11NEGSEL_A::INPUT12
    }
    #[doc = "Checks if the value of the field is `INPUT14`"]
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == INPUT11NEGSEL_A::INPUT14
    }
}
#[doc = "Write proxy for field `INPUT11NEGSEL`"]
pub struct INPUT11NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT11NEGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT11NEGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn input8(self) -> &'a mut W {
        self.variant(INPUT11NEGSEL_A::INPUT8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn input10(self) -> &'a mut W {
        self.variant(INPUT11NEGSEL_A::INPUT10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn input12(self) -> &'a mut W {
        self.variant(INPUT11NEGSEL_A::INPUT12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn input14(self) -> &'a mut W {
        self.variant(INPUT11NEGSEL_A::INPUT14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT13NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT8 as negative channel input"]
    INPUT8,
    #[doc = "1: Selects ADCn_INPUT10 as negative channel input"]
    INPUT10,
    #[doc = "2: Selects ADCn_INPUT12 as negative channel input"]
    INPUT12,
    #[doc = "3: Selects ADCn_INPUT14 as negative channel input"]
    INPUT14,
}
impl From<INPUT13NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT13NEGSEL_A) -> Self {
        match variant {
            INPUT13NEGSEL_A::INPUT8 => 0,
            INPUT13NEGSEL_A::INPUT10 => 1,
            INPUT13NEGSEL_A::INPUT12 => 2,
            INPUT13NEGSEL_A::INPUT14 => 3,
        }
    }
}
#[doc = "Reader of field `INPUT13NEGSEL`"]
pub type INPUT13NEGSEL_R = crate::R<u8, INPUT13NEGSEL_A>;
impl INPUT13NEGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT13NEGSEL_A {
        match self.bits {
            0 => INPUT13NEGSEL_A::INPUT8,
            1 => INPUT13NEGSEL_A::INPUT10,
            2 => INPUT13NEGSEL_A::INPUT12,
            3 => INPUT13NEGSEL_A::INPUT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT8`"]
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == INPUT13NEGSEL_A::INPUT8
    }
    #[doc = "Checks if the value of the field is `INPUT10`"]
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == INPUT13NEGSEL_A::INPUT10
    }
    #[doc = "Checks if the value of the field is `INPUT12`"]
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == INPUT13NEGSEL_A::INPUT12
    }
    #[doc = "Checks if the value of the field is `INPUT14`"]
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == INPUT13NEGSEL_A::INPUT14
    }
}
#[doc = "Write proxy for field `INPUT13NEGSEL`"]
pub struct INPUT13NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT13NEGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT13NEGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn input8(self) -> &'a mut W {
        self.variant(INPUT13NEGSEL_A::INPUT8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn input10(self) -> &'a mut W {
        self.variant(INPUT13NEGSEL_A::INPUT10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn input12(self) -> &'a mut W {
        self.variant(INPUT13NEGSEL_A::INPUT12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn input14(self) -> &'a mut W {
        self.variant(INPUT13NEGSEL_A::INPUT14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT15NEGSEL_A {
    #[doc = "0: Selects ADCn_INPUT8 as negative channel input"]
    INPUT8,
    #[doc = "1: Selects ADCn_INPUT10 as negative channel input"]
    INPUT10,
    #[doc = "2: Selects ADCn_INPUT12 as negative channel input"]
    INPUT12,
    #[doc = "3: Selects ADCn_INPUT14 as negative channel input"]
    INPUT14,
}
impl From<INPUT15NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT15NEGSEL_A) -> Self {
        match variant {
            INPUT15NEGSEL_A::INPUT8 => 0,
            INPUT15NEGSEL_A::INPUT10 => 1,
            INPUT15NEGSEL_A::INPUT12 => 2,
            INPUT15NEGSEL_A::INPUT14 => 3,
        }
    }
}
#[doc = "Reader of field `INPUT15NEGSEL`"]
pub type INPUT15NEGSEL_R = crate::R<u8, INPUT15NEGSEL_A>;
impl INPUT15NEGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT15NEGSEL_A {
        match self.bits {
            0 => INPUT15NEGSEL_A::INPUT8,
            1 => INPUT15NEGSEL_A::INPUT10,
            2 => INPUT15NEGSEL_A::INPUT12,
            3 => INPUT15NEGSEL_A::INPUT14,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INPUT8`"]
    #[inline(always)]
    pub fn is_input8(&self) -> bool {
        *self == INPUT15NEGSEL_A::INPUT8
    }
    #[doc = "Checks if the value of the field is `INPUT10`"]
    #[inline(always)]
    pub fn is_input10(&self) -> bool {
        *self == INPUT15NEGSEL_A::INPUT10
    }
    #[doc = "Checks if the value of the field is `INPUT12`"]
    #[inline(always)]
    pub fn is_input12(&self) -> bool {
        *self == INPUT15NEGSEL_A::INPUT12
    }
    #[doc = "Checks if the value of the field is `INPUT14`"]
    #[inline(always)]
    pub fn is_input14(&self) -> bool {
        *self == INPUT15NEGSEL_A::INPUT14
    }
}
#[doc = "Write proxy for field `INPUT15NEGSEL`"]
pub struct INPUT15NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT15NEGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT15NEGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Selects ADCn_INPUT8 as negative channel input"]
    #[inline(always)]
    pub fn input8(self) -> &'a mut W {
        self.variant(INPUT15NEGSEL_A::INPUT8)
    }
    #[doc = "Selects ADCn_INPUT10 as negative channel input"]
    #[inline(always)]
    pub fn input10(self) -> &'a mut W {
        self.variant(INPUT15NEGSEL_A::INPUT10)
    }
    #[doc = "Selects ADCn_INPUT12 as negative channel input"]
    #[inline(always)]
    pub fn input12(self) -> &'a mut W {
        self.variant(INPUT15NEGSEL_A::INPUT12)
    }
    #[doc = "Selects ADCn_INPUT14 as negative channel input"]
    #[inline(always)]
    pub fn input14(self) -> &'a mut W {
        self.variant(INPUT15NEGSEL_A::INPUT14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input0negsel(&self) -> INPUT0NEGSEL_R {
        INPUT0NEGSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input2negsel(&self) -> INPUT2NEGSEL_R {
        INPUT2NEGSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input4negsel(&self) -> INPUT4NEGSEL_R {
        INPUT4NEGSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input6negsel(&self) -> INPUT6NEGSEL_R {
        INPUT6NEGSEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input9negsel(&self) -> INPUT9NEGSEL_R {
        INPUT9NEGSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input11negsel(&self) -> INPUT11NEGSEL_R {
        INPUT11NEGSEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input13negsel(&self) -> INPUT13NEGSEL_R {
        INPUT13NEGSEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input15negsel(&self) -> INPUT15NEGSEL_R {
        INPUT15NEGSEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Negative Input Select Register for ADCn_INPUT0 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input0negsel(&mut self) -> INPUT0NEGSEL_W {
        INPUT0NEGSEL_W { w: self }
    }
    #[doc = "Bits 2:3 - Negative Input Select Register for ADCn_INPUT2 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input2negsel(&mut self) -> INPUT2NEGSEL_W {
        INPUT2NEGSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - Negative Input Select Register for ADCn_INPUT4 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input4negsel(&mut self) -> INPUT4NEGSEL_W {
        INPUT4NEGSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - Negative Input Select Register for ADCn_INPUT1 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input6negsel(&mut self) -> INPUT6NEGSEL_W {
        INPUT6NEGSEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Negative Input Select Register for ADCn_INPUT9 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input9negsel(&mut self) -> INPUT9NEGSEL_W {
        INPUT9NEGSEL_W { w: self }
    }
    #[doc = "Bits 10:11 - Negative Input Select Register for ADCn_INPUT11 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input11negsel(&mut self) -> INPUT11NEGSEL_W {
        INPUT11NEGSEL_W { w: self }
    }
    #[doc = "Bits 12:13 - Negative Input Select Register for ADCn_INPUT13 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input13negsel(&mut self) -> INPUT13NEGSEL_W {
        INPUT13NEGSEL_W { w: self }
    }
    #[doc = "Bits 14:15 - Negative Input Select Register for ADCn_INPUT15 in Differential Scan Mode"]
    #[inline(always)]
    pub fn input15negsel(&mut self) -> INPUT15NEGSEL_W {
        INPUT15NEGSEL_W { w: self }
    }
}
