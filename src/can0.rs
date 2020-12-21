#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Error Count Register"]
    pub errcnt: ERRCNT,
    #[doc = "0x0c - Bit Timing Register"]
    pub bittiming: BITTIMING,
    #[doc = "0x10 - Interrupt Identification Register"]
    pub intid: INTID,
    #[doc = "0x14 - Test Register"]
    pub test: TEST,
    #[doc = "0x18 - BRP Extension Register"]
    pub brpe: BRPE,
    #[doc = "0x1c - Transmission Request Register"]
    pub transreq: TRANSREQ,
    #[doc = "0x20 - New Data Register"]
    pub messagedata: MESSAGEDATA,
    _reserved9: [u8; 4usize],
    #[doc = "0x28 - Message Valid Register"]
    pub messagestate: MESSAGESTATE,
    #[doc = "0x2c - Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x30 - Message Object Interrupt Flag Register"]
    pub if0if: IF0IF,
    #[doc = "0x34 - Message Object Interrupt Flag Set Register"]
    pub if0ifs: IF0IFS,
    #[doc = "0x38 - Message Object Interrupt Flag Clear Register"]
    pub if0ifc: IF0IFC,
    #[doc = "0x3c - Message Object Interrupt Enable Register"]
    pub if0ien: IF0IEN,
    #[doc = "0x40 - Status Interrupt Flag Register"]
    pub if1if: IF1IF,
    #[doc = "0x44 - Message Object Interrupt Flag Set Register"]
    pub if1ifs: IF1IFS,
    #[doc = "0x48 - Message Object Interrupt Flag Clear Register"]
    pub if1ifc: IF1IFC,
    #[doc = "0x4c - Status Interrupt Enable Register"]
    pub if1ien: IF1IEN,
    #[doc = "0x50 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved20: [u8; 12usize],
    #[doc = "0x60 - Interface Command Mask Register"]
    pub mir0_cmdmask: MIR0_CMDMASK,
    #[doc = "0x64 - Interface Mask Register"]
    pub mir0_mask: MIR0_MASK,
    #[doc = "0x68 - Interface Arbitration Register"]
    pub mir0_arb: MIR0_ARB,
    #[doc = "0x6c - Interface Message Control Register"]
    pub mir0_ctrl: MIR0_CTRL,
    #[doc = "0x70 - Interface Data a Register"]
    pub mir0_datal: MIR0_DATAL,
    #[doc = "0x74 - Interface Data B Register"]
    pub mir0_datah: MIR0_DATAH,
    #[doc = "0x78 - Interface Command Request Register"]
    pub mir0_cmdreq: MIR0_CMDREQ,
    _reserved27: [u8; 4usize],
    #[doc = "0x80 - Interface Command Mask Register"]
    pub mir1_cmdmask: MIR1_CMDMASK,
    #[doc = "0x84 - Interface Mask Register"]
    pub mir1_mask: MIR1_MASK,
    #[doc = "0x88 - Interface Arbitration Register"]
    pub mir1_arb: MIR1_ARB,
    #[doc = "0x8c - Interface Message Control Register"]
    pub mir1_ctrl: MIR1_CTRL,
    #[doc = "0x90 - Interface Data a Register"]
    pub mir1_datal: MIR1_DATAL,
    #[doc = "0x94 - Interface Data B Register"]
    pub mir1_datah: MIR1_DATAH,
    #[doc = "0x98 - Interface Command Request Register"]
    pub mir1_cmdreq: MIR1_CMDREQ,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Error Count Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errcnt](errcnt) module"]
pub type ERRCNT = crate::Reg<u32, _ERRCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRCNT;
#[doc = "`read()` method returns [errcnt::R](errcnt::R) reader structure"]
impl crate::Readable for ERRCNT {}
#[doc = "Error Count Register"]
pub mod errcnt;
#[doc = "Bit Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bittiming](bittiming) module"]
pub type BITTIMING = crate::Reg<u32, _BITTIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BITTIMING;
#[doc = "`read()` method returns [bittiming::R](bittiming::R) reader structure"]
impl crate::Readable for BITTIMING {}
#[doc = "`write(|w| ..)` method takes [bittiming::W](bittiming::W) writer structure"]
impl crate::Writable for BITTIMING {}
#[doc = "Bit Timing Register"]
pub mod bittiming;
#[doc = "Interrupt Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intid](intid) module"]
pub type INTID = crate::Reg<u32, _INTID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTID;
#[doc = "`read()` method returns [intid::R](intid::R) reader structure"]
impl crate::Readable for INTID {}
#[doc = "Interrupt Identification Register"]
pub mod intid;
#[doc = "Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](test) module"]
pub type TEST = crate::Reg<u32, _TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEST;
#[doc = "`read()` method returns [test::R](test::R) reader structure"]
impl crate::Readable for TEST {}
#[doc = "`write(|w| ..)` method takes [test::W](test::W) writer structure"]
impl crate::Writable for TEST {}
#[doc = "Test Register"]
pub mod test;
#[doc = "BRP Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brpe](brpe) module"]
pub type BRPE = crate::Reg<u32, _BRPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRPE;
#[doc = "`read()` method returns [brpe::R](brpe::R) reader structure"]
impl crate::Readable for BRPE {}
#[doc = "`write(|w| ..)` method takes [brpe::W](brpe::W) writer structure"]
impl crate::Writable for BRPE {}
#[doc = "BRP Extension Register"]
pub mod brpe;
#[doc = "Transmission Request Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transreq](transreq) module"]
pub type TRANSREQ = crate::Reg<u32, _TRANSREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRANSREQ;
#[doc = "`read()` method returns [transreq::R](transreq::R) reader structure"]
impl crate::Readable for TRANSREQ {}
#[doc = "Transmission Request Register"]
pub mod transreq;
#[doc = "New Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [messagedata](messagedata) module"]
pub type MESSAGEDATA = crate::Reg<u32, _MESSAGEDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MESSAGEDATA;
#[doc = "`read()` method returns [messagedata::R](messagedata::R) reader structure"]
impl crate::Readable for MESSAGEDATA {}
#[doc = "New Data Register"]
pub mod messagedata;
#[doc = "Message Valid Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [messagestate](messagestate) module"]
pub type MESSAGESTATE = crate::Reg<u32, _MESSAGESTATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MESSAGESTATE;
#[doc = "`read()` method returns [messagestate::R](messagestate::R) reader structure"]
impl crate::Readable for MESSAGESTATE {}
#[doc = "Message Valid Register"]
pub mod messagestate;
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configuration Register"]
pub mod config;
#[doc = "Message Object Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if0if](if0if) module"]
pub type IF0IF = crate::Reg<u32, _IF0IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF0IF;
#[doc = "`read()` method returns [if0if::R](if0if::R) reader structure"]
impl crate::Readable for IF0IF {}
#[doc = "Message Object Interrupt Flag Register"]
pub mod if0if;
#[doc = "Message Object Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if0ifs](if0ifs) module"]
pub type IF0IFS = crate::Reg<u32, _IF0IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF0IFS;
#[doc = "`write(|w| ..)` method takes [if0ifs::W](if0ifs::W) writer structure"]
impl crate::Writable for IF0IFS {}
#[doc = "Message Object Interrupt Flag Set Register"]
pub mod if0ifs;
#[doc = "Message Object Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if0ifc](if0ifc) module"]
pub type IF0IFC = crate::Reg<u32, _IF0IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF0IFC;
#[doc = "`write(|w| ..)` method takes [if0ifc::W](if0ifc::W) writer structure"]
impl crate::Writable for IF0IFC {}
#[doc = "Message Object Interrupt Flag Clear Register"]
pub mod if0ifc;
#[doc = "Message Object Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if0ien](if0ien) module"]
pub type IF0IEN = crate::Reg<u32, _IF0IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF0IEN;
#[doc = "`read()` method returns [if0ien::R](if0ien::R) reader structure"]
impl crate::Readable for IF0IEN {}
#[doc = "`write(|w| ..)` method takes [if0ien::W](if0ien::W) writer structure"]
impl crate::Writable for IF0IEN {}
#[doc = "Message Object Interrupt Enable Register"]
pub mod if0ien;
#[doc = "Status Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1if](if1if) module"]
pub type IF1IF = crate::Reg<u32, _IF1IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1IF;
#[doc = "`read()` method returns [if1if::R](if1if::R) reader structure"]
impl crate::Readable for IF1IF {}
#[doc = "Status Interrupt Flag Register"]
pub mod if1if;
#[doc = "Message Object Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1ifs](if1ifs) module"]
pub type IF1IFS = crate::Reg<u32, _IF1IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1IFS;
#[doc = "`write(|w| ..)` method takes [if1ifs::W](if1ifs::W) writer structure"]
impl crate::Writable for IF1IFS {}
#[doc = "Message Object Interrupt Flag Set Register"]
pub mod if1ifs;
#[doc = "Message Object Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1ifc](if1ifc) module"]
pub type IF1IFC = crate::Reg<u32, _IF1IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1IFC;
#[doc = "`write(|w| ..)` method takes [if1ifc::W](if1ifc::W) writer structure"]
impl crate::Writable for IF1IFC {}
#[doc = "Message Object Interrupt Flag Clear Register"]
pub mod if1ifc;
#[doc = "Status Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1ien](if1ien) module"]
pub type IF1IEN = crate::Reg<u32, _IF1IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF1IEN;
#[doc = "`read()` method returns [if1ien::R](if1ien::R) reader structure"]
impl crate::Readable for IF1IEN {}
#[doc = "`write(|w| ..)` method takes [if1ien::W](if1ien::W) writer structure"]
impl crate::Writable for IF1IEN {}
#[doc = "Status Interrupt Enable Register"]
pub mod if1ien;
#[doc = "I/O Routing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [route](route) module"]
pub type ROUTE = crate::Reg<u32, _ROUTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTE;
#[doc = "`read()` method returns [route::R](route::R) reader structure"]
impl crate::Readable for ROUTE {}
#[doc = "`write(|w| ..)` method takes [route::W](route::W) writer structure"]
impl crate::Writable for ROUTE {}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "Interface Command Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir0_cmdmask](mir0_cmdmask) module"]
pub type MIR0_CMDMASK = crate::Reg<u32, _MIR0_CMDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR0_CMDMASK;
#[doc = "`read()` method returns [mir0_cmdmask::R](mir0_cmdmask::R) reader structure"]
impl crate::Readable for MIR0_CMDMASK {}
#[doc = "`write(|w| ..)` method takes [mir0_cmdmask::W](mir0_cmdmask::W) writer structure"]
impl crate::Writable for MIR0_CMDMASK {}
#[doc = "Interface Command Mask Register"]
pub mod mir0_cmdmask;
#[doc = "Interface Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir0_mask](mir0_mask) module"]
pub type MIR0_MASK = crate::Reg<u32, _MIR0_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR0_MASK;
#[doc = "`read()` method returns [mir0_mask::R](mir0_mask::R) reader structure"]
impl crate::Readable for MIR0_MASK {}
#[doc = "`write(|w| ..)` method takes [mir0_mask::W](mir0_mask::W) writer structure"]
impl crate::Writable for MIR0_MASK {}
#[doc = "Interface Mask Register"]
pub mod mir0_mask;
#[doc = "Interface Arbitration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir0_arb](mir0_arb) module"]
pub type MIR0_ARB = crate::Reg<u32, _MIR0_ARB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR0_ARB;
#[doc = "`read()` method returns [mir0_arb::R](mir0_arb::R) reader structure"]
impl crate::Readable for MIR0_ARB {}
#[doc = "`write(|w| ..)` method takes [mir0_arb::W](mir0_arb::W) writer structure"]
impl crate::Writable for MIR0_ARB {}
#[doc = "Interface Arbitration Register"]
pub mod mir0_arb;
#[doc = "Interface Message Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir0_ctrl](mir0_ctrl) module"]
pub type MIR0_CTRL = crate::Reg<u32, _MIR0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR0_CTRL;
#[doc = "`read()` method returns [mir0_ctrl::R](mir0_ctrl::R) reader structure"]
impl crate::Readable for MIR0_CTRL {}
#[doc = "`write(|w| ..)` method takes [mir0_ctrl::W](mir0_ctrl::W) writer structure"]
impl crate::Writable for MIR0_CTRL {}
#[doc = "Interface Message Control Register"]
pub mod mir0_ctrl;
#[doc = "Interface Data a Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir0_datal](mir0_datal) module"]
pub type MIR0_DATAL = crate::Reg<u32, _MIR0_DATAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR0_DATAL;
#[doc = "`read()` method returns [mir0_datal::R](mir0_datal::R) reader structure"]
impl crate::Readable for MIR0_DATAL {}
#[doc = "`write(|w| ..)` method takes [mir0_datal::W](mir0_datal::W) writer structure"]
impl crate::Writable for MIR0_DATAL {}
#[doc = "Interface Data a Register"]
pub mod mir0_datal;
#[doc = "Interface Data B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir0_datah](mir0_datah) module"]
pub type MIR0_DATAH = crate::Reg<u32, _MIR0_DATAH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR0_DATAH;
#[doc = "`read()` method returns [mir0_datah::R](mir0_datah::R) reader structure"]
impl crate::Readable for MIR0_DATAH {}
#[doc = "`write(|w| ..)` method takes [mir0_datah::W](mir0_datah::W) writer structure"]
impl crate::Writable for MIR0_DATAH {}
#[doc = "Interface Data B Register"]
pub mod mir0_datah;
#[doc = "Interface Command Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir0_cmdreq](mir0_cmdreq) module"]
pub type MIR0_CMDREQ = crate::Reg<u32, _MIR0_CMDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR0_CMDREQ;
#[doc = "`read()` method returns [mir0_cmdreq::R](mir0_cmdreq::R) reader structure"]
impl crate::Readable for MIR0_CMDREQ {}
#[doc = "`write(|w| ..)` method takes [mir0_cmdreq::W](mir0_cmdreq::W) writer structure"]
impl crate::Writable for MIR0_CMDREQ {}
#[doc = "Interface Command Request Register"]
pub mod mir0_cmdreq;
#[doc = "Interface Command Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir1_cmdmask](mir1_cmdmask) module"]
pub type MIR1_CMDMASK = crate::Reg<u32, _MIR1_CMDMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR1_CMDMASK;
#[doc = "`read()` method returns [mir1_cmdmask::R](mir1_cmdmask::R) reader structure"]
impl crate::Readable for MIR1_CMDMASK {}
#[doc = "`write(|w| ..)` method takes [mir1_cmdmask::W](mir1_cmdmask::W) writer structure"]
impl crate::Writable for MIR1_CMDMASK {}
#[doc = "Interface Command Mask Register"]
pub mod mir1_cmdmask;
#[doc = "Interface Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir1_mask](mir1_mask) module"]
pub type MIR1_MASK = crate::Reg<u32, _MIR1_MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR1_MASK;
#[doc = "`read()` method returns [mir1_mask::R](mir1_mask::R) reader structure"]
impl crate::Readable for MIR1_MASK {}
#[doc = "`write(|w| ..)` method takes [mir1_mask::W](mir1_mask::W) writer structure"]
impl crate::Writable for MIR1_MASK {}
#[doc = "Interface Mask Register"]
pub mod mir1_mask;
#[doc = "Interface Arbitration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir1_arb](mir1_arb) module"]
pub type MIR1_ARB = crate::Reg<u32, _MIR1_ARB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR1_ARB;
#[doc = "`read()` method returns [mir1_arb::R](mir1_arb::R) reader structure"]
impl crate::Readable for MIR1_ARB {}
#[doc = "`write(|w| ..)` method takes [mir1_arb::W](mir1_arb::W) writer structure"]
impl crate::Writable for MIR1_ARB {}
#[doc = "Interface Arbitration Register"]
pub mod mir1_arb;
#[doc = "Interface Message Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir1_ctrl](mir1_ctrl) module"]
pub type MIR1_CTRL = crate::Reg<u32, _MIR1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR1_CTRL;
#[doc = "`read()` method returns [mir1_ctrl::R](mir1_ctrl::R) reader structure"]
impl crate::Readable for MIR1_CTRL {}
#[doc = "`write(|w| ..)` method takes [mir1_ctrl::W](mir1_ctrl::W) writer structure"]
impl crate::Writable for MIR1_CTRL {}
#[doc = "Interface Message Control Register"]
pub mod mir1_ctrl;
#[doc = "Interface Data a Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir1_datal](mir1_datal) module"]
pub type MIR1_DATAL = crate::Reg<u32, _MIR1_DATAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR1_DATAL;
#[doc = "`read()` method returns [mir1_datal::R](mir1_datal::R) reader structure"]
impl crate::Readable for MIR1_DATAL {}
#[doc = "`write(|w| ..)` method takes [mir1_datal::W](mir1_datal::W) writer structure"]
impl crate::Writable for MIR1_DATAL {}
#[doc = "Interface Data a Register"]
pub mod mir1_datal;
#[doc = "Interface Data B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir1_datah](mir1_datah) module"]
pub type MIR1_DATAH = crate::Reg<u32, _MIR1_DATAH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR1_DATAH;
#[doc = "`read()` method returns [mir1_datah::R](mir1_datah::R) reader structure"]
impl crate::Readable for MIR1_DATAH {}
#[doc = "`write(|w| ..)` method takes [mir1_datah::W](mir1_datah::W) writer structure"]
impl crate::Writable for MIR1_DATAH {}
#[doc = "Interface Data B Register"]
pub mod mir1_datah;
#[doc = "Interface Command Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir1_cmdreq](mir1_cmdreq) module"]
pub type MIR1_CMDREQ = crate::Reg<u32, _MIR1_CMDREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIR1_CMDREQ;
#[doc = "`read()` method returns [mir1_cmdreq::R](mir1_cmdreq::R) reader structure"]
impl crate::Readable for MIR1_CMDREQ {}
#[doc = "`write(|w| ..)` method takes [mir1_cmdreq::W](mir1_cmdreq::W) writer structure"]
impl crate::Writable for MIR1_CMDREQ {}
#[doc = "Interface Command Request Register"]
pub mod mir1_cmdreq;
