#[doc = "Reader of register STATE"]
pub type R = crate::R<u32, super::STATE>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASTER`"]
pub type MASTER_R = crate::R<bool, bool>;
#[doc = "Reader of field `TRANSMITTER`"]
pub type TRANSMITTER_R = crate::R<bool, bool>;
#[doc = "Reader of field `NACKED`"]
pub type NACKED_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSHOLD`"]
pub type BUSHOLD_R = crate::R<bool, bool>;
#[doc = "Transmission State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: No transmission is being performed."]
    IDLE,
    #[doc = "1: Waiting for idle. Will send a start condition as soon as the bus is idle."]
    WAIT,
    #[doc = "2: Start transmitted or received"]
    START,
    #[doc = "3: Address transmitted or received"]
    ADDR,
    #[doc = "4: Address ack/nack transmitted or received"]
    ADDRACK,
    #[doc = "5: Data transmitted or received"]
    DATA,
    #[doc = "6: Data ack/nack transmitted or received"]
    DATAACK,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        match variant {
            STATE_A::IDLE => 0,
            STATE_A::WAIT => 1,
            STATE_A::START => 2,
            STATE_A::ADDR => 3,
            STATE_A::ADDRACK => 4,
            STATE_A::DATA => 5,
            STATE_A::DATAACK => 6,
        }
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::R<u8, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STATE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STATE_A::IDLE),
            1 => Val(STATE_A::WAIT),
            2 => Val(STATE_A::START),
            3 => Val(STATE_A::ADDR),
            4 => Val(STATE_A::ADDRACK),
            5 => Val(STATE_A::DATA),
            6 => Val(STATE_A::DATAACK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == STATE_A::WAIT
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STATE_A::START
    }
    #[doc = "Checks if the value of the field is `ADDR`"]
    #[inline(always)]
    pub fn is_addr(&self) -> bool {
        *self == STATE_A::ADDR
    }
    #[doc = "Checks if the value of the field is `ADDRACK`"]
    #[inline(always)]
    pub fn is_addrack(&self) -> bool {
        *self == STATE_A::ADDRACK
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == STATE_A::DATA
    }
    #[doc = "Checks if the value of the field is `DATAACK`"]
    #[inline(always)]
    pub fn is_dataack(&self) -> bool {
        *self == STATE_A::DATAACK
    }
}
impl R {
    #[doc = "Bit 0 - Bus Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Master"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitter"]
    #[inline(always)]
    pub fn transmitter(&self) -> TRANSMITTER_R {
        TRANSMITTER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Nack Received"]
    #[inline(always)]
    pub fn nacked(&self) -> NACKED_R {
        NACKED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bus Held"]
    #[inline(always)]
    pub fn bushold(&self) -> BUSHOLD_R {
        BUSHOLD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Transmission State"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
