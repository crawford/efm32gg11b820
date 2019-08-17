#[doc = "Reader of register PPUPATD0"]
pub type R = crate::R<u32, super::PPUPATD0>;
#[doc = "Writer for register PPUPATD0"]
pub type W = crate::W<u32, super::PPUPATD0>;
#[doc = "Register PPUPATD0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PPUPATD0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACMP0`"]
pub type ACMP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP0`"]
pub struct ACMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ACMP1`"]
pub type ACMP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP1`"]
pub struct ACMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ACMP2`"]
pub type ACMP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP2`"]
pub struct ACMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ACMP3`"]
pub type ACMP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP3`"]
pub struct ACMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ADC0`"]
pub type ADC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0`"]
pub struct ADC0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ADC1`"]
pub type ADC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC1`"]
pub struct ADC1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CAN0`"]
pub type CAN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN0`"]
pub struct CAN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CAN1`"]
pub type CAN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN1`"]
pub struct CAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CMU`"]
pub type CMU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMU`"]
pub struct CMU_W<'a> {
    w: &'a mut W,
}
impl<'a> CMU_W<'a> {
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
#[doc = "Reader of field `CRYOTIMER`"]
pub type CRYOTIMER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYOTIMER`"]
pub struct CRYOTIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYOTIMER_W<'a> {
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
#[doc = "Reader of field `CRYPTO0`"]
pub type CRYPTO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYPTO0`"]
pub struct CRYPTO0_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO0_W<'a> {
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
#[doc = "Reader of field `CSEN`"]
pub type CSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSEN`"]
pub struct CSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEN_W<'a> {
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
#[doc = "Reader of field `VDAC0`"]
pub type VDAC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDAC0`"]
pub struct VDAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> VDAC0_W<'a> {
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
#[doc = "Reader of field `PRS`"]
pub type PRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRS`"]
pub struct PRS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRS_W<'a> {
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
#[doc = "Reader of field `EBI`"]
pub type EBI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EBI`"]
pub struct EBI_W<'a> {
    w: &'a mut W,
}
impl<'a> EBI_W<'a> {
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
#[doc = "Reader of field `EMU`"]
pub type EMU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMU`"]
pub struct EMU_W<'a> {
    w: &'a mut W,
}
impl<'a> EMU_W<'a> {
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
#[doc = "Reader of field `ETH`"]
pub type ETH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETH`"]
pub struct ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH_W<'a> {
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
#[doc = "Reader of field `FPUEH`"]
pub type FPUEH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPUEH`"]
pub struct FPUEH_W<'a> {
    w: &'a mut W,
}
impl<'a> FPUEH_W<'a> {
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
#[doc = "Reader of field `GPCRC`"]
pub type GPCRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPCRC`"]
pub struct GPCRC_W<'a> {
    w: &'a mut W,
}
impl<'a> GPCRC_W<'a> {
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
#[doc = "Reader of field `GPIO`"]
pub type GPIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO`"]
pub struct GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_W<'a> {
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
#[doc = "Reader of field `I2C0`"]
pub type I2C0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0`"]
pub struct I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_W<'a> {
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
#[doc = "Reader of field `I2C1`"]
pub type I2C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1`"]
pub struct I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_W<'a> {
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
#[doc = "Reader of field `I2C2`"]
pub type I2C2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2`"]
pub struct I2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_W<'a> {
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
#[doc = "Reader of field `IDAC0`"]
pub type IDAC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDAC0`"]
pub struct IDAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAC0_W<'a> {
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
#[doc = "Reader of field `MSC`"]
pub type MSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MSC`"]
pub struct MSC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSC_W<'a> {
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
#[doc = "Reader of field `LCD`"]
pub type LCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCD`"]
pub struct LCD_W<'a> {
    w: &'a mut W,
}
impl<'a> LCD_W<'a> {
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
#[doc = "Reader of field `LDMA`"]
pub type LDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDMA`"]
pub struct LDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> LDMA_W<'a> {
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
#[doc = "Reader of field `LESENSE`"]
pub type LESENSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LESENSE`"]
pub struct LESENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LESENSE_W<'a> {
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
#[doc = "Reader of field `LETIMER0`"]
pub type LETIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LETIMER0`"]
pub struct LETIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> LETIMER0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `LETIMER1`"]
pub type LETIMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LETIMER1`"]
pub struct LETIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> LETIMER1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `LEUART0`"]
pub type LEUART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEUART0`"]
pub struct LEUART0_W<'a> {
    w: &'a mut W,
}
impl<'a> LEUART0_W<'a> {
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
#[doc = "Reader of field `LEUART1`"]
pub type LEUART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEUART1`"]
pub struct LEUART1_W<'a> {
    w: &'a mut W,
}
impl<'a> LEUART1_W<'a> {
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
    #[doc = "Bit 0 - Analog Comparator 0 access control bit"]
    #[inline(always)]
    pub fn acmp0(&self) -> ACMP0_R {
        ACMP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp1(&self) -> ACMP1_R {
        ACMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp2(&self) -> ACMP2_R {
        ACMP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator 3 access control bit"]
    #[inline(always)]
    pub fn acmp3(&self) -> ACMP3_R {
        ACMP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CAN 0 access control bit"]
    #[inline(always)]
    pub fn can0(&self) -> CAN0_R {
        CAN0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CAN 1 access control bit"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clock Management Unit access control bit"]
    #[inline(always)]
    pub fn cmu(&self) -> CMU_R {
        CMU_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CryoTimer access control bit"]
    #[inline(always)]
    pub fn cryotimer(&self) -> CRYOTIMER_R {
        CRYOTIMER_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Advanced Encryption Standard Accelerator access control bit"]
    #[inline(always)]
    pub fn crypto0(&self) -> CRYPTO0_R {
        CRYPTO0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Capacitive touch sense module access control bit"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn vdac0(&self) -> VDAC0_R {
        VDAC0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Peripheral Reflex System access control bit"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Bus Interface access control bit"]
    #[inline(always)]
    pub fn ebi(&self) -> EBI_R {
        EBI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Energy Management Unit access control bit"]
    #[inline(always)]
    pub fn emu(&self) -> EMU_R {
        EMU_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Ethernet Controller access control bit"]
    #[inline(always)]
    pub fn eth(&self) -> ETH_R {
        ETH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FPU Exception Handler access control bit"]
    #[inline(always)]
    pub fn fpueh(&self) -> FPUEH_R {
        FPUEH_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - General Purpose CRC access control bit"]
    #[inline(always)]
    pub fn gpcrc(&self) -> GPCRC_R {
        GPCRC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - General purpose Input/Output access control bit"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - I2C 0 access control bit"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C 1 access control bit"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C 2 access control bit"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Current Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn idac0(&self) -> IDAC0_R {
        IDAC0_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Memory System Controller access control bit"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Liquid Crystal Display Controller access control bit"]
    #[inline(always)]
    pub fn lcd(&self) -> LCD_R {
        LCD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Linked Direct Memory Access Controller access control bit"]
    #[inline(always)]
    pub fn ldma(&self) -> LDMA_R {
        LDMA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Low Energy Sensor Interface access control bit"]
    #[inline(always)]
    pub fn lesense(&self) -> LESENSE_R {
        LESENSE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Low Energy Timer 0 access control bit"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Low Energy Timer 1 access control bit"]
    #[inline(always)]
    pub fn letimer1(&self) -> LETIMER1_R {
        LETIMER1_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Low Energy UART 0 access control bit"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Low Energy UART 1 access control bit"]
    #[inline(always)]
    pub fn leuart1(&self) -> LEUART1_R {
        LEUART1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator 0 access control bit"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> ACMP0_W {
        ACMP0_W { w: self }
    }
    #[doc = "Bit 1 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp1(&mut self) -> ACMP1_W {
        ACMP1_W { w: self }
    }
    #[doc = "Bit 2 - Analog Comparator 1 access control bit"]
    #[inline(always)]
    pub fn acmp2(&mut self) -> ACMP2_W {
        ACMP2_W { w: self }
    }
    #[doc = "Bit 3 - Analog Comparator 3 access control bit"]
    #[inline(always)]
    pub fn acmp3(&mut self) -> ACMP3_W {
        ACMP3_W { w: self }
    }
    #[doc = "Bit 4 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc0(&mut self) -> ADC0_W {
        ADC0_W { w: self }
    }
    #[doc = "Bit 5 - Analog to Digital Converter 0 access control bit"]
    #[inline(always)]
    pub fn adc1(&mut self) -> ADC1_W {
        ADC1_W { w: self }
    }
    #[doc = "Bit 6 - CAN 0 access control bit"]
    #[inline(always)]
    pub fn can0(&mut self) -> CAN0_W {
        CAN0_W { w: self }
    }
    #[doc = "Bit 7 - CAN 1 access control bit"]
    #[inline(always)]
    pub fn can1(&mut self) -> CAN1_W {
        CAN1_W { w: self }
    }
    #[doc = "Bit 8 - Clock Management Unit access control bit"]
    #[inline(always)]
    pub fn cmu(&mut self) -> CMU_W {
        CMU_W { w: self }
    }
    #[doc = "Bit 9 - CryoTimer access control bit"]
    #[inline(always)]
    pub fn cryotimer(&mut self) -> CRYOTIMER_W {
        CRYOTIMER_W { w: self }
    }
    #[doc = "Bit 10 - Advanced Encryption Standard Accelerator access control bit"]
    #[inline(always)]
    pub fn crypto0(&mut self) -> CRYPTO0_W {
        CRYPTO0_W { w: self }
    }
    #[doc = "Bit 11 - Capacitive touch sense module access control bit"]
    #[inline(always)]
    pub fn csen(&mut self) -> CSEN_W {
        CSEN_W { w: self }
    }
    #[doc = "Bit 12 - Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn vdac0(&mut self) -> VDAC0_W {
        VDAC0_W { w: self }
    }
    #[doc = "Bit 13 - Peripheral Reflex System access control bit"]
    #[inline(always)]
    pub fn prs(&mut self) -> PRS_W {
        PRS_W { w: self }
    }
    #[doc = "Bit 14 - External Bus Interface access control bit"]
    #[inline(always)]
    pub fn ebi(&mut self) -> EBI_W {
        EBI_W { w: self }
    }
    #[doc = "Bit 15 - Energy Management Unit access control bit"]
    #[inline(always)]
    pub fn emu(&mut self) -> EMU_W {
        EMU_W { w: self }
    }
    #[doc = "Bit 16 - Ethernet Controller access control bit"]
    #[inline(always)]
    pub fn eth(&mut self) -> ETH_W {
        ETH_W { w: self }
    }
    #[doc = "Bit 17 - FPU Exception Handler access control bit"]
    #[inline(always)]
    pub fn fpueh(&mut self) -> FPUEH_W {
        FPUEH_W { w: self }
    }
    #[doc = "Bit 18 - General Purpose CRC access control bit"]
    #[inline(always)]
    pub fn gpcrc(&mut self) -> GPCRC_W {
        GPCRC_W { w: self }
    }
    #[doc = "Bit 19 - General purpose Input/Output access control bit"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
    }
    #[doc = "Bit 20 - I2C 0 access control bit"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W {
        I2C0_W { w: self }
    }
    #[doc = "Bit 21 - I2C 1 access control bit"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W {
        I2C1_W { w: self }
    }
    #[doc = "Bit 22 - I2C 2 access control bit"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2C2_W {
        I2C2_W { w: self }
    }
    #[doc = "Bit 23 - Current Digital to Analog Converter 0 access control bit"]
    #[inline(always)]
    pub fn idac0(&mut self) -> IDAC0_W {
        IDAC0_W { w: self }
    }
    #[doc = "Bit 24 - Memory System Controller access control bit"]
    #[inline(always)]
    pub fn msc(&mut self) -> MSC_W {
        MSC_W { w: self }
    }
    #[doc = "Bit 25 - Liquid Crystal Display Controller access control bit"]
    #[inline(always)]
    pub fn lcd(&mut self) -> LCD_W {
        LCD_W { w: self }
    }
    #[doc = "Bit 26 - Linked Direct Memory Access Controller access control bit"]
    #[inline(always)]
    pub fn ldma(&mut self) -> LDMA_W {
        LDMA_W { w: self }
    }
    #[doc = "Bit 27 - Low Energy Sensor Interface access control bit"]
    #[inline(always)]
    pub fn lesense(&mut self) -> LESENSE_W {
        LESENSE_W { w: self }
    }
    #[doc = "Bit 28 - Low Energy Timer 0 access control bit"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> LETIMER0_W {
        LETIMER0_W { w: self }
    }
    #[doc = "Bit 29 - Low Energy Timer 1 access control bit"]
    #[inline(always)]
    pub fn letimer1(&mut self) -> LETIMER1_W {
        LETIMER1_W { w: self }
    }
    #[doc = "Bit 30 - Low Energy UART 0 access control bit"]
    #[inline(always)]
    pub fn leuart0(&mut self) -> LEUART0_W {
        LEUART0_W { w: self }
    }
    #[doc = "Bit 31 - Low Energy UART 1 access control bit"]
    #[inline(always)]
    pub fn leuart1(&mut self) -> LEUART1_W {
        LEUART1_W { w: self }
    }
}
