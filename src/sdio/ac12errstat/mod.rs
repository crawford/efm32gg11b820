#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AC12ERRSTAT {
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
pub struct AC12NOTEXER {
    bits: bool,
}
impl AC12NOTEXER {
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
pub struct AC12TOER {
    bits: bool,
}
impl AC12TOER {
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
pub struct AC12CRCERRR {
    bits: bool,
}
impl AC12CRCERRR {
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
pub struct AC12ENDBITERRR {
    bits: bool,
}
impl AC12ENDBITERRR {
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
pub struct AC12INDEXERRR {
    bits: bool,
}
impl AC12INDEXERRR {
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
pub struct CNIBAC12ERRR {
    bits: bool,
}
impl CNIBAC12ERRR {
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
#[doc = "Possible values of the field `UHSMODESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UHSMODESELR {
    #[doc = "SDR12"]
    SDR12,
    #[doc = "SDR25"]
    SDR25,
    #[doc = "SDR50"]
    SDR50,
    #[doc = "SDR104"]
    SDR104,
    #[doc = "DDR50"]
    DDR50,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UHSMODESELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UHSMODESELR::SDR12 => 0,
            UHSMODESELR::SDR25 => 1,
            UHSMODESELR::SDR50 => 2,
            UHSMODESELR::SDR104 => 3,
            UHSMODESELR::DDR50 => 4,
            UHSMODESELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UHSMODESELR {
        match value {
            0 => UHSMODESELR::SDR12,
            1 => UHSMODESELR::SDR25,
            2 => UHSMODESELR::SDR50,
            3 => UHSMODESELR::SDR104,
            4 => UHSMODESELR::DDR50,
            i => UHSMODESELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDR12`"]
    #[inline]
    pub fn is_sdr12(&self) -> bool {
        *self == UHSMODESELR::SDR12
    }
    #[doc = "Checks if the value of the field is `SDR25`"]
    #[inline]
    pub fn is_sdr25(&self) -> bool {
        *self == UHSMODESELR::SDR25
    }
    #[doc = "Checks if the value of the field is `SDR50`"]
    #[inline]
    pub fn is_sdr50(&self) -> bool {
        *self == UHSMODESELR::SDR50
    }
    #[doc = "Checks if the value of the field is `SDR104`"]
    #[inline]
    pub fn is_sdr104(&self) -> bool {
        *self == UHSMODESELR::SDR104
    }
    #[doc = "Checks if the value of the field is `DDR50`"]
    #[inline]
    pub fn is_ddr50(&self) -> bool {
        *self == UHSMODESELR::DDR50
    }
}
#[doc = r" Value of the field"]
pub struct SIGEN1P8VR {
    bits: bool,
}
impl SIGEN1P8VR {
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
#[doc = "Possible values of the field `DRVSTNSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRVSTNSELR {
    #[doc = "Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "Driver Type A is selected"]
    TYPEA,
    #[doc = "Driver Type C is selected"]
    TYPEC,
    #[doc = "Driver Type D is selected"]
    TYPED,
}
impl DRVSTNSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DRVSTNSELR::TYPEB => 0,
            DRVSTNSELR::TYPEA => 1,
            DRVSTNSELR::TYPEC => 2,
            DRVSTNSELR::TYPED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DRVSTNSELR {
        match value {
            0 => DRVSTNSELR::TYPEB,
            1 => DRVSTNSELR::TYPEA,
            2 => DRVSTNSELR::TYPEC,
            3 => DRVSTNSELR::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline]
    pub fn is_typeb(&self) -> bool {
        *self == DRVSTNSELR::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline]
    pub fn is_typea(&self) -> bool {
        *self == DRVSTNSELR::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline]
    pub fn is_typec(&self) -> bool {
        *self == DRVSTNSELR::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline]
    pub fn is_typed(&self) -> bool {
        *self == DRVSTNSELR::TYPED
    }
}
#[doc = r" Value of the field"]
pub struct EXETUNINGR {
    bits: bool,
}
impl EXETUNINGR {
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
pub struct SAMPCLKSELR {
    bits: bool,
}
impl SAMPCLKSELR {
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
pub struct ASYNCINTENR {
    bits: bool,
}
impl ASYNCINTENR {
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
pub struct PRSTVALENR {
    bits: bool,
}
impl PRSTVALENR {
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
#[doc = "Values that can be written to the field `UHSMODESEL`"]
pub enum UHSMODESELW {
    #[doc = "SDR12"]
    SDR12,
    #[doc = "SDR25"]
    SDR25,
    #[doc = "SDR50"]
    SDR50,
    #[doc = "SDR104"]
    SDR104,
    #[doc = "DDR50"]
    DDR50,
}
impl UHSMODESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UHSMODESELW::SDR12 => 0,
            UHSMODESELW::SDR25 => 1,
            UHSMODESELW::SDR50 => 2,
            UHSMODESELW::SDR104 => 3,
            UHSMODESELW::DDR50 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UHSMODESELW<'a> {
    w: &'a mut W,
}
impl<'a> _UHSMODESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UHSMODESELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SDR12"]
    #[inline]
    pub fn sdr12(self) -> &'a mut W {
        self.variant(UHSMODESELW::SDR12)
    }
    #[doc = "SDR25"]
    #[inline]
    pub fn sdr25(self) -> &'a mut W {
        self.variant(UHSMODESELW::SDR25)
    }
    #[doc = "SDR50"]
    #[inline]
    pub fn sdr50(self) -> &'a mut W {
        self.variant(UHSMODESELW::SDR50)
    }
    #[doc = "SDR104"]
    #[inline]
    pub fn sdr104(self) -> &'a mut W {
        self.variant(UHSMODESELW::SDR104)
    }
    #[doc = "DDR50"]
    #[inline]
    pub fn ddr50(self) -> &'a mut W {
        self.variant(UHSMODESELW::DDR50)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SIGEN1P8VW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGEN1P8VW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DRVSTNSEL`"]
pub enum DRVSTNSELW {
    #[doc = "Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "Driver Type A is selected"]
    TYPEA,
    #[doc = "Driver Type C is selected"]
    TYPEC,
    #[doc = "Driver Type D is selected"]
    TYPED,
}
impl DRVSTNSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DRVSTNSELW::TYPEB => 0,
            DRVSTNSELW::TYPEA => 1,
            DRVSTNSELW::TYPEC => 2,
            DRVSTNSELW::TYPED => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRVSTNSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DRVSTNSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRVSTNSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline]
    pub fn typeb(self) -> &'a mut W {
        self.variant(DRVSTNSELW::TYPEB)
    }
    #[doc = "Driver Type A is selected"]
    #[inline]
    pub fn typea(self) -> &'a mut W {
        self.variant(DRVSTNSELW::TYPEA)
    }
    #[doc = "Driver Type C is selected"]
    #[inline]
    pub fn typec(self) -> &'a mut W {
        self.variant(DRVSTNSELW::TYPEC)
    }
    #[doc = "Driver Type D is selected"]
    #[inline]
    pub fn typed(self) -> &'a mut W {
        self.variant(DRVSTNSELW::TYPED)
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
pub struct _EXETUNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _EXETUNINGW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SAMPCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPCLKSELW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ASYNCINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ASYNCINTENW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRSTVALENW<'a> {
    w: &'a mut W,
}
impl<'a> _PRSTVALENW<'a> {
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
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline]
    pub fn ac12notexe(&self) -> AC12NOTEXER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AC12NOTEXER { bits }
    }
    #[doc = "Bit 1 - Auto CMD12 Timeout Error"]
    #[inline]
    pub fn ac12toe(&self) -> AC12TOER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AC12TOER { bits }
    }
    #[doc = "Bit 2 - Auto CMD CRC Error"]
    #[inline]
    pub fn ac12crcerr(&self) -> AC12CRCERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AC12CRCERRR { bits }
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error"]
    #[inline]
    pub fn ac12endbiterr(&self) -> AC12ENDBITERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AC12ENDBITERRR { bits }
    }
    #[doc = "Bit 4 - Auto CMD Index Error"]
    #[inline]
    pub fn ac12indexerr(&self) -> AC12INDEXERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AC12INDEXERRR { bits }
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
    #[inline]
    pub fn cnibac12err(&self) -> CNIBAC12ERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CNIBAC12ERRR { bits }
    }
    #[doc = "Bits 16:18 - UHS Mode Select"]
    #[inline]
    pub fn uhsmodesel(&self) -> UHSMODESELR {
        UHSMODESELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Voltage 1.8V Signal Enable"]
    #[inline]
    pub fn sigen1p8v(&self) -> SIGEN1P8VR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SIGEN1P8VR { bits }
    }
    #[doc = "Bits 20:21 - Driver Strength Select"]
    #[inline]
    pub fn drvstnsel(&self) -> DRVSTNSELR {
        DRVSTNSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 22 - Execute Tuning"]
    #[inline]
    pub fn exetuning(&self) -> EXETUNINGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXETUNINGR { bits }
    }
    #[doc = "Bit 23 - Sampling Clock Select"]
    #[inline]
    pub fn sampclksel(&self) -> SAMPCLKSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SAMPCLKSELR { bits }
    }
    #[doc = "Bit 30 - Asynchronous Interrupt Enable"]
    #[inline]
    pub fn asyncinten(&self) -> ASYNCINTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASYNCINTENR { bits }
    }
    #[doc = "Bit 31 - Preset Value Enable"]
    #[inline]
    pub fn prstvalen(&self) -> PRSTVALENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PRSTVALENR { bits }
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
    #[doc = "Bits 16:18 - UHS Mode Select"]
    #[inline]
    pub fn uhsmodesel(&mut self) -> _UHSMODESELW {
        _UHSMODESELW { w: self }
    }
    #[doc = "Bit 19 - Voltage 1.8V Signal Enable"]
    #[inline]
    pub fn sigen1p8v(&mut self) -> _SIGEN1P8VW {
        _SIGEN1P8VW { w: self }
    }
    #[doc = "Bits 20:21 - Driver Strength Select"]
    #[inline]
    pub fn drvstnsel(&mut self) -> _DRVSTNSELW {
        _DRVSTNSELW { w: self }
    }
    #[doc = "Bit 22 - Execute Tuning"]
    #[inline]
    pub fn exetuning(&mut self) -> _EXETUNINGW {
        _EXETUNINGW { w: self }
    }
    #[doc = "Bit 23 - Sampling Clock Select"]
    #[inline]
    pub fn sampclksel(&mut self) -> _SAMPCLKSELW {
        _SAMPCLKSELW { w: self }
    }
    #[doc = "Bit 30 - Asynchronous Interrupt Enable"]
    #[inline]
    pub fn asyncinten(&mut self) -> _ASYNCINTENW {
        _ASYNCINTENW { w: self }
    }
    #[doc = "Bit 31 - Preset Value Enable"]
    #[inline]
    pub fn prstvalen(&mut self) -> _PRSTVALENW {
        _PRSTVALENW { w: self }
    }
}
