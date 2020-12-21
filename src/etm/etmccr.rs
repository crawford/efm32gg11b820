#[doc = "Reader of register ETMCCR"]
pub type R = crate::R<u32, super::ETMCCR>;
#[doc = "Reader of field `ADRCMPPAIR`"]
pub type ADRCMPPAIR_R = crate::R<u8, u8>;
#[doc = "Reader of field `DATACMPNUM`"]
pub type DATACMPNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `MMDECCNT`"]
pub type MMDECCNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `COUNTNUM`"]
pub type COUNTNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `SEQPRES`"]
pub type SEQPRES_R = crate::R<bool, bool>;
#[doc = "Number of External Inputs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTINPNUM_A {
    #[doc = "0: Zero inputs presents"]
    ZERO = 0,
    #[doc = "1: One inputs presents"]
    ONE = 1,
    #[doc = "2: Two inputs presents"]
    TWO = 2,
}
impl From<EXTINPNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTINPNUM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTINPNUM`"]
pub type EXTINPNUM_R = crate::R<u8, EXTINPNUM_A>;
impl EXTINPNUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTINPNUM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTINPNUM_A::ZERO),
            1 => Val(EXTINPNUM_A::ONE),
            2 => Val(EXTINPNUM_A::TWO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == EXTINPNUM_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == EXTINPNUM_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == EXTINPNUM_A::TWO
    }
}
#[doc = "Reader of field `EXTOUTNUM`"]
pub type EXTOUTNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIFOFULLPRES`"]
pub type FIFOFULLPRES_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDCOMPNUM`"]
pub type IDCOMPNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRACESS`"]
pub type TRACESS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMACCESS`"]
pub type MMACCESS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ETMID`"]
pub type ETMID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Number of Address Comparator Pairs"]
    #[inline(always)]
    pub fn adrcmppair(&self) -> ADRCMPPAIR_R {
        ADRCMPPAIR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Number of Data Value Comparators"]
    #[inline(always)]
    pub fn datacmpnum(&self) -> DATACMPNUM_R {
        DATACMPNUM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Number of Memeory Map Decoders"]
    #[inline(always)]
    pub fn mmdeccnt(&self) -> MMDECCNT_R {
        MMDECCNT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Number of Counters"]
    #[inline(always)]
    pub fn countnum(&self) -> COUNTNUM_R {
        COUNTNUM_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 16 - Sequencer Present"]
    #[inline(always)]
    pub fn seqpres(&self) -> SEQPRES_R {
        SEQPRES_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - Number of External Inputs"]
    #[inline(always)]
    pub fn extinpnum(&self) -> EXTINPNUM_R {
        EXTINPNUM_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Number of External Output"]
    #[inline(always)]
    pub fn extoutnum(&self) -> EXTOUTNUM_R {
        EXTOUTNUM_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - FIFIO FULL present"]
    #[inline(always)]
    pub fn fifofullpres(&self) -> FIFOFULLPRES_R {
        FIFOFULLPRES_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Number of context ID Comparators"]
    #[inline(always)]
    pub fn idcompnum(&self) -> IDCOMPNUM_R {
        IDCOMPNUM_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Trace Start/Stop Block Present"]
    #[inline(always)]
    pub fn tracess(&self) -> TRACESS_R {
        TRACESS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Coprocessor and Memeory Access"]
    #[inline(always)]
    pub fn mmaccess(&self) -> MMACCESS_R {
        MMACCESS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ETM ID Register Present"]
    #[inline(always)]
    pub fn etmid(&self) -> ETMID_R {
        ETMID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
