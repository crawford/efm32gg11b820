#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8A8,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16A16ALE,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8A24ALE,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16,
}
impl crate::ToBits<u8> for MODE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODE_A::D8A8 => 0,
            MODE_A::D16A16ALE => 1,
            MODE_A::D8A24ALE => 2,
            MODE_A::D16 => 3,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::D8A8,
            1 => MODE_A::D16A16ALE,
            2 => MODE_A::D8A24ALE,
            3 => MODE_A::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE_A::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE_A::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE_A::D8A24ALE
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == MODE_A::D16
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODE_A::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODE_A::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODE_A::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(MODE_A::D16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `MODE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE1_A {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8A8,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16A16ALE,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8A24ALE,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16,
}
impl crate::ToBits<u8> for MODE1_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODE1_A::D8A8 => 0,
            MODE1_A::D16A16ALE => 1,
            MODE1_A::D8A24ALE => 2,
            MODE1_A::D16 => 3,
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
            0 => MODE1_A::D8A8,
            1 => MODE1_A::D16A16ALE,
            2 => MODE1_A::D8A24ALE,
            3 => MODE1_A::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE1_A::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE1_A::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE1_A::D8A24ALE
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == MODE1_A::D16
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
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODE1_A::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODE1_A::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODE1_A::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(MODE1_A::D16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `MODE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE2_A {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8A8,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16A16ALE,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8A24ALE,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16,
}
impl crate::ToBits<u8> for MODE2_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODE2_A::D8A8 => 0,
            MODE2_A::D16A16ALE => 1,
            MODE2_A::D8A24ALE => 2,
            MODE2_A::D16 => 3,
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
            0 => MODE2_A::D8A8,
            1 => MODE2_A::D16A16ALE,
            2 => MODE2_A::D8A24ALE,
            3 => MODE2_A::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE2_A::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE2_A::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE2_A::D8A24ALE
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == MODE2_A::D16
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
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODE2_A::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODE2_A::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODE2_A::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(MODE2_A::D16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `MODE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE3_A {
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    D8A8,
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D16A16ALE,
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    D8A24ALE,
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    D16,
}
impl crate::ToBits<u8> for MODE3_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODE3_A::D8A8 => 0,
            MODE3_A::D16A16ALE => 1,
            MODE3_A::D8A24ALE => 2,
            MODE3_A::D16 => 3,
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
            0 => MODE3_A::D8A8,
            1 => MODE3_A::D16A16ALE,
            2 => MODE3_A::D8A24ALE,
            3 => MODE3_A::D16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `D8A8`"]
    #[inline(always)]
    pub fn is_d8a8(&self) -> bool {
        *self == MODE3_A::D8A8
    }
    #[doc = "Checks if the value of the field is `D16A16ALE`"]
    #[inline(always)]
    pub fn is_d16a16ale(&self) -> bool {
        *self == MODE3_A::D16A16ALE
    }
    #[doc = "Checks if the value of the field is `D8A24ALE`"]
    #[inline(always)]
    pub fn is_d8a24ale(&self) -> bool {
        *self == MODE3_A::D8A24ALE
    }
    #[doc = "Checks if the value of the field is `D16`"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == MODE3_A::D16
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
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "EBI_AD drives 8 bit data, 8 bit address, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a8(self) -> &'a mut W {
        self.variant(MODE3_A::D8A8)
    }
    #[doc = "EBI_AD drives 16 bit data, 16 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16a16ale(self) -> &'a mut W {
        self.variant(MODE3_A::D16A16ALE)
    }
    #[doc = "EBI_AD drives 8 bit data, 24 bit address, ALE is used for address latching. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d8a24ale(self) -> &'a mut W {
        self.variant(MODE3_A::D8A24ALE)
    }
    #[doc = "EBI_AD drives 16 bit data, ALE not used. Extended address bits can be enabled."]
    #[inline(always)]
    pub fn d16(self) -> &'a mut W {
        self.variant(MODE3_A::D16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `BANK0EN`"]
pub type BANK0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BANK0EN`"]
pub struct BANK0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK0EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `BANK1EN`"]
pub type BANK1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BANK1EN`"]
pub struct BANK1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK1EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `BANK2EN`"]
pub type BANK2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BANK2EN`"]
pub struct BANK2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK2EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `BANK3EN`"]
pub type BANK3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BANK3EN`"]
pub struct BANK3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BANK3EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `NOIDLE`"]
pub type NOIDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOIDLE`"]
pub struct NOIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NOIDLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `NOIDLE1`"]
pub type NOIDLE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOIDLE1`"]
pub struct NOIDLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> NOIDLE1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `NOIDLE2`"]
pub type NOIDLE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOIDLE2`"]
pub struct NOIDLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> NOIDLE2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `NOIDLE3`"]
pub type NOIDLE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOIDLE3`"]
pub struct NOIDLE3_W<'a> {
    w: &'a mut W,
}
impl<'a> NOIDLE3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `ARDYEN`"]
pub type ARDYEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDYEN`"]
pub struct ARDYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDYEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ARDYTODIS`"]
pub type ARDYTODIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDYTODIS`"]
pub struct ARDYTODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDYTODIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ARDY1EN`"]
pub type ARDY1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDY1EN`"]
pub struct ARDY1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDY1EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `ARDYTO1DIS`"]
pub type ARDYTO1DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDYTO1DIS`"]
pub struct ARDYTO1DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDYTO1DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `ARDY2EN`"]
pub type ARDY2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDY2EN`"]
pub struct ARDY2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDY2EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `ARDYTO2DIS`"]
pub type ARDYTO2DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDYTO2DIS`"]
pub struct ARDYTO2DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDYTO2DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ARDY3EN`"]
pub type ARDY3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDY3EN`"]
pub struct ARDY3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDY3EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ARDYTO3DIS`"]
pub type ARDYTO3DIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARDYTO3DIS`"]
pub struct ARDYTO3DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ARDYTO3DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `BL`"]
pub type BL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BL`"]
pub struct BL_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `BL1`"]
pub type BL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BL1`"]
pub struct BL1_W<'a> {
    w: &'a mut W,
}
impl<'a> BL1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `BL2`"]
pub type BL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BL2`"]
pub struct BL2_W<'a> {
    w: &'a mut W,
}
impl<'a> BL2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `BL3`"]
pub type BL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BL3`"]
pub struct BL3_W<'a> {
    w: &'a mut W,
}
impl<'a> BL3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `ITS`"]
pub type ITS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ITS`"]
pub struct ITS_W<'a> {
    w: &'a mut W,
}
impl<'a> ITS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `ALTMAP`"]
pub type ALTMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTMAP`"]
pub struct ALTMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Mode 1"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Mode 2"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Mode 3"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Bank 0 Enable"]
    #[inline(always)]
    pub fn bank0en(&self) -> BANK0EN_R {
        BANK0EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Bank 1 Enable"]
    #[inline(always)]
    pub fn bank1en(&self) -> BANK1EN_R {
        BANK1EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Bank 2 Enable"]
    #[inline(always)]
    pub fn bank2en(&self) -> BANK2EN_R {
        BANK2EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Bank 3 Enable"]
    #[inline(always)]
    pub fn bank3en(&self) -> BANK3EN_R {
        BANK3EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - No Idle Cycle Insertion on Bank 0"]
    #[inline(always)]
    pub fn noidle(&self) -> NOIDLE_R {
        NOIDLE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - No Idle Cycle Insertion on Bank 1"]
    #[inline(always)]
    pub fn noidle1(&self) -> NOIDLE1_R {
        NOIDLE1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - No Idle Cycle Insertion on Bank 2"]
    #[inline(always)]
    pub fn noidle2(&self) -> NOIDLE2_R {
        NOIDLE2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - No Idle Cycle Insertion on Bank 3"]
    #[inline(always)]
    pub fn noidle3(&self) -> NOIDLE3_R {
        NOIDLE3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ARDY Enable"]
    #[inline(always)]
    pub fn ardyen(&self) -> ARDYEN_R {
        ARDYEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ARDY Timeout Disable"]
    #[inline(always)]
    pub fn ardytodis(&self) -> ARDYTODIS_R {
        ARDYTODIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ARDY Enable for Bank 1"]
    #[inline(always)]
    pub fn ardy1en(&self) -> ARDY1EN_R {
        ARDY1EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ARDY Timeout Disable for Bank 1"]
    #[inline(always)]
    pub fn ardyto1dis(&self) -> ARDYTO1DIS_R {
        ARDYTO1DIS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ARDY Enable for Bank 2"]
    #[inline(always)]
    pub fn ardy2en(&self) -> ARDY2EN_R {
        ARDY2EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ARDY Timeout Disable for Bank 2"]
    #[inline(always)]
    pub fn ardyto2dis(&self) -> ARDYTO2DIS_R {
        ARDYTO2DIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ARDY Enable for Bank 3"]
    #[inline(always)]
    pub fn ardy3en(&self) -> ARDY3EN_R {
        ARDY3EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ARDY Timeout Disable for Bank 3"]
    #[inline(always)]
    pub fn ardyto3dis(&self) -> ARDYTO3DIS_R {
        ARDYTO3DIS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Byte Lane Enable for Bank 0"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Byte Lane Enable for Bank 1"]
    #[inline(always)]
    pub fn bl1(&self) -> BL1_R {
        BL1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Byte Lane Enable for Bank 2"]
    #[inline(always)]
    pub fn bl2(&self) -> BL2_R {
        BL2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Byte Lane Enable for Bank 3"]
    #[inline(always)]
    pub fn bl3(&self) -> BL3_R {
        BL3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Individual Timing Set, Line Polarity and Mode Definition Enable"]
    #[inline(always)]
    pub fn its(&self) -> ITS_R {
        ITS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Alternative Address Map Enable"]
    #[inline(always)]
    pub fn altmap(&self) -> ALTMAP_R {
        ALTMAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Mode 1"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    #[doc = "Bits 4:5 - Mode 2"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W { w: self }
    }
    #[doc = "Bits 6:7 - Mode 3"]
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W {
        MODE3_W { w: self }
    }
    #[doc = "Bit 8 - Bank 0 Enable"]
    #[inline(always)]
    pub fn bank0en(&mut self) -> BANK0EN_W {
        BANK0EN_W { w: self }
    }
    #[doc = "Bit 9 - Bank 1 Enable"]
    #[inline(always)]
    pub fn bank1en(&mut self) -> BANK1EN_W {
        BANK1EN_W { w: self }
    }
    #[doc = "Bit 10 - Bank 2 Enable"]
    #[inline(always)]
    pub fn bank2en(&mut self) -> BANK2EN_W {
        BANK2EN_W { w: self }
    }
    #[doc = "Bit 11 - Bank 3 Enable"]
    #[inline(always)]
    pub fn bank3en(&mut self) -> BANK3EN_W {
        BANK3EN_W { w: self }
    }
    #[doc = "Bit 12 - No Idle Cycle Insertion on Bank 0"]
    #[inline(always)]
    pub fn noidle(&mut self) -> NOIDLE_W {
        NOIDLE_W { w: self }
    }
    #[doc = "Bit 13 - No Idle Cycle Insertion on Bank 1"]
    #[inline(always)]
    pub fn noidle1(&mut self) -> NOIDLE1_W {
        NOIDLE1_W { w: self }
    }
    #[doc = "Bit 14 - No Idle Cycle Insertion on Bank 2"]
    #[inline(always)]
    pub fn noidle2(&mut self) -> NOIDLE2_W {
        NOIDLE2_W { w: self }
    }
    #[doc = "Bit 15 - No Idle Cycle Insertion on Bank 3"]
    #[inline(always)]
    pub fn noidle3(&mut self) -> NOIDLE3_W {
        NOIDLE3_W { w: self }
    }
    #[doc = "Bit 16 - ARDY Enable"]
    #[inline(always)]
    pub fn ardyen(&mut self) -> ARDYEN_W {
        ARDYEN_W { w: self }
    }
    #[doc = "Bit 17 - ARDY Timeout Disable"]
    #[inline(always)]
    pub fn ardytodis(&mut self) -> ARDYTODIS_W {
        ARDYTODIS_W { w: self }
    }
    #[doc = "Bit 18 - ARDY Enable for Bank 1"]
    #[inline(always)]
    pub fn ardy1en(&mut self) -> ARDY1EN_W {
        ARDY1EN_W { w: self }
    }
    #[doc = "Bit 19 - ARDY Timeout Disable for Bank 1"]
    #[inline(always)]
    pub fn ardyto1dis(&mut self) -> ARDYTO1DIS_W {
        ARDYTO1DIS_W { w: self }
    }
    #[doc = "Bit 20 - ARDY Enable for Bank 2"]
    #[inline(always)]
    pub fn ardy2en(&mut self) -> ARDY2EN_W {
        ARDY2EN_W { w: self }
    }
    #[doc = "Bit 21 - ARDY Timeout Disable for Bank 2"]
    #[inline(always)]
    pub fn ardyto2dis(&mut self) -> ARDYTO2DIS_W {
        ARDYTO2DIS_W { w: self }
    }
    #[doc = "Bit 22 - ARDY Enable for Bank 3"]
    #[inline(always)]
    pub fn ardy3en(&mut self) -> ARDY3EN_W {
        ARDY3EN_W { w: self }
    }
    #[doc = "Bit 23 - ARDY Timeout Disable for Bank 3"]
    #[inline(always)]
    pub fn ardyto3dis(&mut self) -> ARDYTO3DIS_W {
        ARDYTO3DIS_W { w: self }
    }
    #[doc = "Bit 24 - Byte Lane Enable for Bank 0"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W {
        BL_W { w: self }
    }
    #[doc = "Bit 25 - Byte Lane Enable for Bank 1"]
    #[inline(always)]
    pub fn bl1(&mut self) -> BL1_W {
        BL1_W { w: self }
    }
    #[doc = "Bit 26 - Byte Lane Enable for Bank 2"]
    #[inline(always)]
    pub fn bl2(&mut self) -> BL2_W {
        BL2_W { w: self }
    }
    #[doc = "Bit 27 - Byte Lane Enable for Bank 3"]
    #[inline(always)]
    pub fn bl3(&mut self) -> BL3_W {
        BL3_W { w: self }
    }
    #[doc = "Bit 30 - Individual Timing Set, Line Polarity and Mode Definition Enable"]
    #[inline(always)]
    pub fn its(&mut self) -> ITS_W {
        ITS_W { w: self }
    }
    #[doc = "Bit 31 - Alternative Address Map Enable"]
    #[inline(always)]
    pub fn altmap(&mut self) -> ALTMAP_W {
        ALTMAP_W { w: self }
    }
}
