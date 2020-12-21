#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Control Register"]
    pub pa_ctrl: PA_CTRL,
    #[doc = "0x04 - Port Pin Mode Low Register"]
    pub pa_model: PA_MODEL,
    #[doc = "0x08 - Port Pin Mode High Register"]
    pub pa_modeh: PA_MODEH,
    #[doc = "0x0c - Port Data Out Register"]
    pub pa_dout: PA_DOUT,
    _reserved4: [u8; 8usize],
    #[doc = "0x18 - Port Data Out Toggle Register"]
    pub pa_douttgl: PA_DOUTTGL,
    #[doc = "0x1c - Port Data in Register"]
    pub pa_din: PA_DIN,
    #[doc = "0x20 - Port Unlocked Pins Register"]
    pub pa_pinlockn: PA_PINLOCKN,
    _reserved7: [u8; 4usize],
    #[doc = "0x28 - Over Voltage Disable for All Modes"]
    pub pa_ovtdis: PA_OVTDIS,
    _reserved8: [u8; 4usize],
    #[doc = "0x30 - Port Control Register"]
    pub pb_ctrl: PB_CTRL,
    #[doc = "0x34 - Port Pin Mode Low Register"]
    pub pb_model: PB_MODEL,
    #[doc = "0x38 - Port Pin Mode High Register"]
    pub pb_modeh: PB_MODEH,
    #[doc = "0x3c - Port Data Out Register"]
    pub pb_dout: PB_DOUT,
    _reserved12: [u8; 8usize],
    #[doc = "0x48 - Port Data Out Toggle Register"]
    pub pb_douttgl: PB_DOUTTGL,
    #[doc = "0x4c - Port Data in Register"]
    pub pb_din: PB_DIN,
    #[doc = "0x50 - Port Unlocked Pins Register"]
    pub pb_pinlockn: PB_PINLOCKN,
    _reserved15: [u8; 4usize],
    #[doc = "0x58 - Over Voltage Disable for All Modes"]
    pub pb_ovtdis: PB_OVTDIS,
    _reserved16: [u8; 4usize],
    #[doc = "0x60 - Port Control Register"]
    pub pc_ctrl: PC_CTRL,
    #[doc = "0x64 - Port Pin Mode Low Register"]
    pub pc_model: PC_MODEL,
    #[doc = "0x68 - Port Pin Mode High Register"]
    pub pc_modeh: PC_MODEH,
    #[doc = "0x6c - Port Data Out Register"]
    pub pc_dout: PC_DOUT,
    _reserved20: [u8; 8usize],
    #[doc = "0x78 - Port Data Out Toggle Register"]
    pub pc_douttgl: PC_DOUTTGL,
    #[doc = "0x7c - Port Data in Register"]
    pub pc_din: PC_DIN,
    #[doc = "0x80 - Port Unlocked Pins Register"]
    pub pc_pinlockn: PC_PINLOCKN,
    _reserved23: [u8; 4usize],
    #[doc = "0x88 - Over Voltage Disable for All Modes"]
    pub pc_ovtdis: PC_OVTDIS,
    _reserved24: [u8; 4usize],
    #[doc = "0x90 - Port Control Register"]
    pub pd_ctrl: PD_CTRL,
    #[doc = "0x94 - Port Pin Mode Low Register"]
    pub pd_model: PD_MODEL,
    #[doc = "0x98 - Port Pin Mode High Register"]
    pub pd_modeh: PD_MODEH,
    #[doc = "0x9c - Port Data Out Register"]
    pub pd_dout: PD_DOUT,
    _reserved28: [u8; 8usize],
    #[doc = "0xa8 - Port Data Out Toggle Register"]
    pub pd_douttgl: PD_DOUTTGL,
    #[doc = "0xac - Port Data in Register"]
    pub pd_din: PD_DIN,
    #[doc = "0xb0 - Port Unlocked Pins Register"]
    pub pd_pinlockn: PD_PINLOCKN,
    _reserved31: [u8; 4usize],
    #[doc = "0xb8 - Over Voltage Disable for All Modes"]
    pub pd_ovtdis: PD_OVTDIS,
    _reserved32: [u8; 4usize],
    #[doc = "0xc0 - Port Control Register"]
    pub pe_ctrl: PE_CTRL,
    #[doc = "0xc4 - Port Pin Mode Low Register"]
    pub pe_model: PE_MODEL,
    #[doc = "0xc8 - Port Pin Mode High Register"]
    pub pe_modeh: PE_MODEH,
    #[doc = "0xcc - Port Data Out Register"]
    pub pe_dout: PE_DOUT,
    _reserved36: [u8; 8usize],
    #[doc = "0xd8 - Port Data Out Toggle Register"]
    pub pe_douttgl: PE_DOUTTGL,
    #[doc = "0xdc - Port Data in Register"]
    pub pe_din: PE_DIN,
    #[doc = "0xe0 - Port Unlocked Pins Register"]
    pub pe_pinlockn: PE_PINLOCKN,
    _reserved39: [u8; 4usize],
    #[doc = "0xe8 - Over Voltage Disable for All Modes"]
    pub pe_ovtdis: PE_OVTDIS,
    _reserved40: [u8; 4usize],
    #[doc = "0xf0 - Port Control Register"]
    pub pf_ctrl: PF_CTRL,
    #[doc = "0xf4 - Port Pin Mode Low Register"]
    pub pf_model: PF_MODEL,
    #[doc = "0xf8 - Port Pin Mode High Register"]
    pub pf_modeh: PF_MODEH,
    #[doc = "0xfc - Port Data Out Register"]
    pub pf_dout: PF_DOUT,
    _reserved44: [u8; 8usize],
    #[doc = "0x108 - Port Data Out Toggle Register"]
    pub pf_douttgl: PF_DOUTTGL,
    #[doc = "0x10c - Port Data in Register"]
    pub pf_din: PF_DIN,
    #[doc = "0x110 - Port Unlocked Pins Register"]
    pub pf_pinlockn: PF_PINLOCKN,
    _reserved47: [u8; 4usize],
    #[doc = "0x118 - Over Voltage Disable for All Modes"]
    pub pf_ovtdis: PF_OVTDIS,
    _reserved48: [u8; 4usize],
    #[doc = "0x120 - Port Control Register"]
    pub pg_ctrl: PG_CTRL,
    #[doc = "0x124 - Port Pin Mode Low Register"]
    pub pg_model: PG_MODEL,
    #[doc = "0x128 - Port Pin Mode High Register"]
    pub pg_modeh: PG_MODEH,
    #[doc = "0x12c - Port Data Out Register"]
    pub pg_dout: PG_DOUT,
    _reserved52: [u8; 8usize],
    #[doc = "0x138 - Port Data Out Toggle Register"]
    pub pg_douttgl: PG_DOUTTGL,
    #[doc = "0x13c - Port Data in Register"]
    pub pg_din: PG_DIN,
    #[doc = "0x140 - Port Unlocked Pins Register"]
    pub pg_pinlockn: PG_PINLOCKN,
    _reserved55: [u8; 4usize],
    #[doc = "0x148 - Over Voltage Disable for All Modes"]
    pub pg_ovtdis: PG_OVTDIS,
    _reserved56: [u8; 4usize],
    #[doc = "0x150 - Port Control Register"]
    pub ph_ctrl: PH_CTRL,
    #[doc = "0x154 - Port Pin Mode Low Register"]
    pub ph_model: PH_MODEL,
    #[doc = "0x158 - Port Pin Mode High Register"]
    pub ph_modeh: PH_MODEH,
    #[doc = "0x15c - Port Data Out Register"]
    pub ph_dout: PH_DOUT,
    _reserved60: [u8; 8usize],
    #[doc = "0x168 - Port Data Out Toggle Register"]
    pub ph_douttgl: PH_DOUTTGL,
    #[doc = "0x16c - Port Data in Register"]
    pub ph_din: PH_DIN,
    #[doc = "0x170 - Port Unlocked Pins Register"]
    pub ph_pinlockn: PH_PINLOCKN,
    _reserved63: [u8; 4usize],
    #[doc = "0x178 - Over Voltage Disable for All Modes"]
    pub ph_ovtdis: PH_OVTDIS,
    _reserved64: [u8; 4usize],
    #[doc = "0x180 - Port Control Register"]
    pub pi_ctrl: PI_CTRL,
    #[doc = "0x184 - Port Pin Mode Low Register"]
    pub pi_model: PI_MODEL,
    #[doc = "0x188 - Port Pin Mode High Register"]
    pub pi_modeh: PI_MODEH,
    #[doc = "0x18c - Port Data Out Register"]
    pub pi_dout: PI_DOUT,
    _reserved68: [u8; 8usize],
    #[doc = "0x198 - Port Data Out Toggle Register"]
    pub pi_douttgl: PI_DOUTTGL,
    #[doc = "0x19c - Port Data in Register"]
    pub pi_din: PI_DIN,
    #[doc = "0x1a0 - Port Unlocked Pins Register"]
    pub pi_pinlockn: PI_PINLOCKN,
    _reserved71: [u8; 4usize],
    #[doc = "0x1a8 - Over Voltage Disable for All Modes"]
    pub pi_ovtdis: PI_OVTDIS,
    _reserved72: [u8; 4usize],
    #[doc = "0x1b0 - Port Control Register"]
    pub pj_ctrl: PJ_CTRL,
    #[doc = "0x1b4 - Port Pin Mode Low Register"]
    pub pj_model: PJ_MODEL,
    #[doc = "0x1b8 - Port Pin Mode High Register"]
    pub pj_modeh: PJ_MODEH,
    #[doc = "0x1bc - Port Data Out Register"]
    pub pj_dout: PJ_DOUT,
    _reserved76: [u8; 8usize],
    #[doc = "0x1c8 - Port Data Out Toggle Register"]
    pub pj_douttgl: PJ_DOUTTGL,
    #[doc = "0x1cc - Port Data in Register"]
    pub pj_din: PJ_DIN,
    #[doc = "0x1d0 - Port Unlocked Pins Register"]
    pub pj_pinlockn: PJ_PINLOCKN,
    _reserved79: [u8; 4usize],
    #[doc = "0x1d8 - Over Voltage Disable for All Modes"]
    pub pj_ovtdis: PJ_OVTDIS,
    _reserved80: [u8; 4usize],
    #[doc = "0x1e0 - Port Control Register"]
    pub pk_ctrl: PK_CTRL,
    #[doc = "0x1e4 - Port Pin Mode Low Register"]
    pub pk_model: PK_MODEL,
    #[doc = "0x1e8 - Port Pin Mode High Register"]
    pub pk_modeh: PK_MODEH,
    #[doc = "0x1ec - Port Data Out Register"]
    pub pk_dout: PK_DOUT,
    _reserved84: [u8; 8usize],
    #[doc = "0x1f8 - Port Data Out Toggle Register"]
    pub pk_douttgl: PK_DOUTTGL,
    #[doc = "0x1fc - Port Data in Register"]
    pub pk_din: PK_DIN,
    #[doc = "0x200 - Port Unlocked Pins Register"]
    pub pk_pinlockn: PK_PINLOCKN,
    _reserved87: [u8; 4usize],
    #[doc = "0x208 - Over Voltage Disable for All Modes"]
    pub pk_ovtdis: PK_OVTDIS,
    _reserved88: [u8; 4usize],
    #[doc = "0x210 - Port Control Register"]
    pub pl_ctrl: PL_CTRL,
    #[doc = "0x214 - Port Pin Mode Low Register"]
    pub pl_model: PL_MODEL,
    #[doc = "0x218 - Port Pin Mode High Register"]
    pub pl_modeh: PL_MODEH,
    #[doc = "0x21c - Port Data Out Register"]
    pub pl_dout: PL_DOUT,
    _reserved92: [u8; 8usize],
    #[doc = "0x228 - Port Data Out Toggle Register"]
    pub pl_douttgl: PL_DOUTTGL,
    #[doc = "0x22c - Port Data in Register"]
    pub pl_din: PL_DIN,
    #[doc = "0x230 - Port Unlocked Pins Register"]
    pub pl_pinlockn: PL_PINLOCKN,
    _reserved95: [u8; 4usize],
    #[doc = "0x238 - Over Voltage Disable for All Modes"]
    pub pl_ovtdis: PL_OVTDIS,
    _reserved96: [u8; 452usize],
    #[doc = "0x400 - External Interrupt Port Select Low Register"]
    pub extipsell: EXTIPSELL,
    #[doc = "0x404 - External Interrupt Port Select High Register"]
    pub extipselh: EXTIPSELH,
    #[doc = "0x408 - External Interrupt Pin Select Low Register"]
    pub extipinsell: EXTIPINSELL,
    #[doc = "0x40c - External Interrupt Pin Select High Register"]
    pub extipinselh: EXTIPINSELH,
    #[doc = "0x410 - External Interrupt Rising Edge Trigger Register"]
    pub extirise: EXTIRISE,
    #[doc = "0x414 - External Interrupt Falling Edge Trigger Register"]
    pub extifall: EXTIFALL,
    #[doc = "0x418 - External Interrupt Level Register"]
    pub extilevel: EXTILEVEL,
    #[doc = "0x41c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x420 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x424 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x428 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x42c - EM4 Wake Up Enable Register"]
    pub em4wuen: EM4WUEN,
    _reserved108: [u8; 16usize],
    #[doc = "0x440 - I/O Routing Pin Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0x444 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    _reserved110: [u8; 8usize],
    #[doc = "0x450 - Input Sense Register"]
    pub insense: INSENSE,
    #[doc = "0x454 - Configuration Lock Register"]
    pub lock: LOCK,
}
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_ctrl](pa_ctrl) module"]
pub type PA_CTRL = crate::Reg<u32, _PA_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_CTRL;
#[doc = "`read()` method returns [pa_ctrl::R](pa_ctrl::R) reader structure"]
impl crate::Readable for PA_CTRL {}
#[doc = "`write(|w| ..)` method takes [pa_ctrl::W](pa_ctrl::W) writer structure"]
impl crate::Writable for PA_CTRL {}
#[doc = "Port Control Register"]
pub mod pa_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_model](pa_model) module"]
pub type PA_MODEL = crate::Reg<u32, _PA_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_MODEL;
#[doc = "`read()` method returns [pa_model::R](pa_model::R) reader structure"]
impl crate::Readable for PA_MODEL {}
#[doc = "`write(|w| ..)` method takes [pa_model::W](pa_model::W) writer structure"]
impl crate::Writable for PA_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pa_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_modeh](pa_modeh) module"]
pub type PA_MODEH = crate::Reg<u32, _PA_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_MODEH;
#[doc = "`read()` method returns [pa_modeh::R](pa_modeh::R) reader structure"]
impl crate::Readable for PA_MODEH {}
#[doc = "`write(|w| ..)` method takes [pa_modeh::W](pa_modeh::W) writer structure"]
impl crate::Writable for PA_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pa_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_dout](pa_dout) module"]
pub type PA_DOUT = crate::Reg<u32, _PA_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_DOUT;
#[doc = "`read()` method returns [pa_dout::R](pa_dout::R) reader structure"]
impl crate::Readable for PA_DOUT {}
#[doc = "`write(|w| ..)` method takes [pa_dout::W](pa_dout::W) writer structure"]
impl crate::Writable for PA_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pa_dout;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_douttgl](pa_douttgl) module"]
pub type PA_DOUTTGL = crate::Reg<u32, _PA_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pa_douttgl::W](pa_douttgl::W) writer structure"]
impl crate::Writable for PA_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pa_douttgl;
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_din](pa_din) module"]
pub type PA_DIN = crate::Reg<u32, _PA_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_DIN;
#[doc = "`read()` method returns [pa_din::R](pa_din::R) reader structure"]
impl crate::Readable for PA_DIN {}
#[doc = "Port Data in Register"]
pub mod pa_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_pinlockn](pa_pinlockn) module"]
pub type PA_PINLOCKN = crate::Reg<u32, _PA_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_PINLOCKN;
#[doc = "`read()` method returns [pa_pinlockn::R](pa_pinlockn::R) reader structure"]
impl crate::Readable for PA_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pa_pinlockn::W](pa_pinlockn::W) writer structure"]
impl crate::Writable for PA_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pa_pinlockn;
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_ovtdis](pa_ovtdis) module"]
pub type PA_OVTDIS = crate::Reg<u32, _PA_OVTDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PA_OVTDIS;
#[doc = "`read()` method returns [pa_ovtdis::R](pa_ovtdis::R) reader structure"]
impl crate::Readable for PA_OVTDIS {}
#[doc = "`write(|w| ..)` method takes [pa_ovtdis::W](pa_ovtdis::W) writer structure"]
impl crate::Writable for PA_OVTDIS {}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pa_ovtdis;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_ctrl](pb_ctrl) module"]
pub type PB_CTRL = crate::Reg<u32, _PB_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_CTRL;
#[doc = "`read()` method returns [pb_ctrl::R](pb_ctrl::R) reader structure"]
impl crate::Readable for PB_CTRL {}
#[doc = "`write(|w| ..)` method takes [pb_ctrl::W](pb_ctrl::W) writer structure"]
impl crate::Writable for PB_CTRL {}
#[doc = "Port Control Register"]
pub mod pb_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_model](pb_model) module"]
pub type PB_MODEL = crate::Reg<u32, _PB_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_MODEL;
#[doc = "`read()` method returns [pb_model::R](pb_model::R) reader structure"]
impl crate::Readable for PB_MODEL {}
#[doc = "`write(|w| ..)` method takes [pb_model::W](pb_model::W) writer structure"]
impl crate::Writable for PB_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pb_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_modeh](pb_modeh) module"]
pub type PB_MODEH = crate::Reg<u32, _PB_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_MODEH;
#[doc = "`read()` method returns [pb_modeh::R](pb_modeh::R) reader structure"]
impl crate::Readable for PB_MODEH {}
#[doc = "`write(|w| ..)` method takes [pb_modeh::W](pb_modeh::W) writer structure"]
impl crate::Writable for PB_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pb_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_dout](pb_dout) module"]
pub type PB_DOUT = crate::Reg<u32, _PB_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_DOUT;
#[doc = "`read()` method returns [pb_dout::R](pb_dout::R) reader structure"]
impl crate::Readable for PB_DOUT {}
#[doc = "`write(|w| ..)` method takes [pb_dout::W](pb_dout::W) writer structure"]
impl crate::Writable for PB_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pb_dout;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_douttgl](pb_douttgl) module"]
pub type PB_DOUTTGL = crate::Reg<u32, _PB_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pb_douttgl::W](pb_douttgl::W) writer structure"]
impl crate::Writable for PB_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pb_douttgl;
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_din](pb_din) module"]
pub type PB_DIN = crate::Reg<u32, _PB_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_DIN;
#[doc = "`read()` method returns [pb_din::R](pb_din::R) reader structure"]
impl crate::Readable for PB_DIN {}
#[doc = "Port Data in Register"]
pub mod pb_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_pinlockn](pb_pinlockn) module"]
pub type PB_PINLOCKN = crate::Reg<u32, _PB_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_PINLOCKN;
#[doc = "`read()` method returns [pb_pinlockn::R](pb_pinlockn::R) reader structure"]
impl crate::Readable for PB_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pb_pinlockn::W](pb_pinlockn::W) writer structure"]
impl crate::Writable for PB_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pb_pinlockn;
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb_ovtdis](pb_ovtdis) module"]
pub type PB_OVTDIS = crate::Reg<u32, _PB_OVTDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB_OVTDIS;
#[doc = "`read()` method returns [pb_ovtdis::R](pb_ovtdis::R) reader structure"]
impl crate::Readable for PB_OVTDIS {}
#[doc = "`write(|w| ..)` method takes [pb_ovtdis::W](pb_ovtdis::W) writer structure"]
impl crate::Writable for PB_OVTDIS {}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pb_ovtdis;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_ctrl](pc_ctrl) module"]
pub type PC_CTRL = crate::Reg<u32, _PC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_CTRL;
#[doc = "`read()` method returns [pc_ctrl::R](pc_ctrl::R) reader structure"]
impl crate::Readable for PC_CTRL {}
#[doc = "`write(|w| ..)` method takes [pc_ctrl::W](pc_ctrl::W) writer structure"]
impl crate::Writable for PC_CTRL {}
#[doc = "Port Control Register"]
pub mod pc_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_model](pc_model) module"]
pub type PC_MODEL = crate::Reg<u32, _PC_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_MODEL;
#[doc = "`read()` method returns [pc_model::R](pc_model::R) reader structure"]
impl crate::Readable for PC_MODEL {}
#[doc = "`write(|w| ..)` method takes [pc_model::W](pc_model::W) writer structure"]
impl crate::Writable for PC_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pc_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_modeh](pc_modeh) module"]
pub type PC_MODEH = crate::Reg<u32, _PC_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_MODEH;
#[doc = "`read()` method returns [pc_modeh::R](pc_modeh::R) reader structure"]
impl crate::Readable for PC_MODEH {}
#[doc = "`write(|w| ..)` method takes [pc_modeh::W](pc_modeh::W) writer structure"]
impl crate::Writable for PC_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pc_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_dout](pc_dout) module"]
pub type PC_DOUT = crate::Reg<u32, _PC_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_DOUT;
#[doc = "`read()` method returns [pc_dout::R](pc_dout::R) reader structure"]
impl crate::Readable for PC_DOUT {}
#[doc = "`write(|w| ..)` method takes [pc_dout::W](pc_dout::W) writer structure"]
impl crate::Writable for PC_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pc_dout;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_douttgl](pc_douttgl) module"]
pub type PC_DOUTTGL = crate::Reg<u32, _PC_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pc_douttgl::W](pc_douttgl::W) writer structure"]
impl crate::Writable for PC_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pc_douttgl;
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_din](pc_din) module"]
pub type PC_DIN = crate::Reg<u32, _PC_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_DIN;
#[doc = "`read()` method returns [pc_din::R](pc_din::R) reader structure"]
impl crate::Readable for PC_DIN {}
#[doc = "Port Data in Register"]
pub mod pc_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_pinlockn](pc_pinlockn) module"]
pub type PC_PINLOCKN = crate::Reg<u32, _PC_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_PINLOCKN;
#[doc = "`read()` method returns [pc_pinlockn::R](pc_pinlockn::R) reader structure"]
impl crate::Readable for PC_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pc_pinlockn::W](pc_pinlockn::W) writer structure"]
impl crate::Writable for PC_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pc_pinlockn;
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_ovtdis](pc_ovtdis) module"]
pub type PC_OVTDIS = crate::Reg<u32, _PC_OVTDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC_OVTDIS;
#[doc = "`read()` method returns [pc_ovtdis::R](pc_ovtdis::R) reader structure"]
impl crate::Readable for PC_OVTDIS {}
#[doc = "`write(|w| ..)` method takes [pc_ovtdis::W](pc_ovtdis::W) writer structure"]
impl crate::Writable for PC_OVTDIS {}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pc_ovtdis;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_ctrl](pd_ctrl) module"]
pub type PD_CTRL = crate::Reg<u32, _PD_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_CTRL;
#[doc = "`read()` method returns [pd_ctrl::R](pd_ctrl::R) reader structure"]
impl crate::Readable for PD_CTRL {}
#[doc = "`write(|w| ..)` method takes [pd_ctrl::W](pd_ctrl::W) writer structure"]
impl crate::Writable for PD_CTRL {}
#[doc = "Port Control Register"]
pub mod pd_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_model](pd_model) module"]
pub type PD_MODEL = crate::Reg<u32, _PD_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_MODEL;
#[doc = "`read()` method returns [pd_model::R](pd_model::R) reader structure"]
impl crate::Readable for PD_MODEL {}
#[doc = "`write(|w| ..)` method takes [pd_model::W](pd_model::W) writer structure"]
impl crate::Writable for PD_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pd_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_modeh](pd_modeh) module"]
pub type PD_MODEH = crate::Reg<u32, _PD_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_MODEH;
#[doc = "`read()` method returns [pd_modeh::R](pd_modeh::R) reader structure"]
impl crate::Readable for PD_MODEH {}
#[doc = "`write(|w| ..)` method takes [pd_modeh::W](pd_modeh::W) writer structure"]
impl crate::Writable for PD_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pd_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_dout](pd_dout) module"]
pub type PD_DOUT = crate::Reg<u32, _PD_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_DOUT;
#[doc = "`read()` method returns [pd_dout::R](pd_dout::R) reader structure"]
impl crate::Readable for PD_DOUT {}
#[doc = "`write(|w| ..)` method takes [pd_dout::W](pd_dout::W) writer structure"]
impl crate::Writable for PD_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pd_dout;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_douttgl](pd_douttgl) module"]
pub type PD_DOUTTGL = crate::Reg<u32, _PD_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pd_douttgl::W](pd_douttgl::W) writer structure"]
impl crate::Writable for PD_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pd_douttgl;
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_din](pd_din) module"]
pub type PD_DIN = crate::Reg<u32, _PD_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_DIN;
#[doc = "`read()` method returns [pd_din::R](pd_din::R) reader structure"]
impl crate::Readable for PD_DIN {}
#[doc = "Port Data in Register"]
pub mod pd_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_pinlockn](pd_pinlockn) module"]
pub type PD_PINLOCKN = crate::Reg<u32, _PD_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_PINLOCKN;
#[doc = "`read()` method returns [pd_pinlockn::R](pd_pinlockn::R) reader structure"]
impl crate::Readable for PD_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pd_pinlockn::W](pd_pinlockn::W) writer structure"]
impl crate::Writable for PD_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pd_pinlockn;
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_ovtdis](pd_ovtdis) module"]
pub type PD_OVTDIS = crate::Reg<u32, _PD_OVTDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PD_OVTDIS;
#[doc = "`read()` method returns [pd_ovtdis::R](pd_ovtdis::R) reader structure"]
impl crate::Readable for PD_OVTDIS {}
#[doc = "`write(|w| ..)` method takes [pd_ovtdis::W](pd_ovtdis::W) writer structure"]
impl crate::Writable for PD_OVTDIS {}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pd_ovtdis;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_ctrl](pe_ctrl) module"]
pub type PE_CTRL = crate::Reg<u32, _PE_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_CTRL;
#[doc = "`read()` method returns [pe_ctrl::R](pe_ctrl::R) reader structure"]
impl crate::Readable for PE_CTRL {}
#[doc = "`write(|w| ..)` method takes [pe_ctrl::W](pe_ctrl::W) writer structure"]
impl crate::Writable for PE_CTRL {}
#[doc = "Port Control Register"]
pub mod pe_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_model](pe_model) module"]
pub type PE_MODEL = crate::Reg<u32, _PE_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_MODEL;
#[doc = "`read()` method returns [pe_model::R](pe_model::R) reader structure"]
impl crate::Readable for PE_MODEL {}
#[doc = "`write(|w| ..)` method takes [pe_model::W](pe_model::W) writer structure"]
impl crate::Writable for PE_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pe_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_modeh](pe_modeh) module"]
pub type PE_MODEH = crate::Reg<u32, _PE_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_MODEH;
#[doc = "`read()` method returns [pe_modeh::R](pe_modeh::R) reader structure"]
impl crate::Readable for PE_MODEH {}
#[doc = "`write(|w| ..)` method takes [pe_modeh::W](pe_modeh::W) writer structure"]
impl crate::Writable for PE_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pe_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_dout](pe_dout) module"]
pub type PE_DOUT = crate::Reg<u32, _PE_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_DOUT;
#[doc = "`read()` method returns [pe_dout::R](pe_dout::R) reader structure"]
impl crate::Readable for PE_DOUT {}
#[doc = "`write(|w| ..)` method takes [pe_dout::W](pe_dout::W) writer structure"]
impl crate::Writable for PE_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pe_dout;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_douttgl](pe_douttgl) module"]
pub type PE_DOUTTGL = crate::Reg<u32, _PE_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pe_douttgl::W](pe_douttgl::W) writer structure"]
impl crate::Writable for PE_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pe_douttgl;
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_din](pe_din) module"]
pub type PE_DIN = crate::Reg<u32, _PE_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_DIN;
#[doc = "`read()` method returns [pe_din::R](pe_din::R) reader structure"]
impl crate::Readable for PE_DIN {}
#[doc = "Port Data in Register"]
pub mod pe_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_pinlockn](pe_pinlockn) module"]
pub type PE_PINLOCKN = crate::Reg<u32, _PE_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_PINLOCKN;
#[doc = "`read()` method returns [pe_pinlockn::R](pe_pinlockn::R) reader structure"]
impl crate::Readable for PE_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pe_pinlockn::W](pe_pinlockn::W) writer structure"]
impl crate::Writable for PE_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pe_pinlockn;
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_ovtdis](pe_ovtdis) module"]
pub type PE_OVTDIS = crate::Reg<u32, _PE_OVTDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PE_OVTDIS;
#[doc = "`read()` method returns [pe_ovtdis::R](pe_ovtdis::R) reader structure"]
impl crate::Readable for PE_OVTDIS {}
#[doc = "`write(|w| ..)` method takes [pe_ovtdis::W](pe_ovtdis::W) writer structure"]
impl crate::Writable for PE_OVTDIS {}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pe_ovtdis;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_ctrl](pf_ctrl) module"]
pub type PF_CTRL = crate::Reg<u32, _PF_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_CTRL;
#[doc = "`read()` method returns [pf_ctrl::R](pf_ctrl::R) reader structure"]
impl crate::Readable for PF_CTRL {}
#[doc = "`write(|w| ..)` method takes [pf_ctrl::W](pf_ctrl::W) writer structure"]
impl crate::Writable for PF_CTRL {}
#[doc = "Port Control Register"]
pub mod pf_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_model](pf_model) module"]
pub type PF_MODEL = crate::Reg<u32, _PF_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_MODEL;
#[doc = "`read()` method returns [pf_model::R](pf_model::R) reader structure"]
impl crate::Readable for PF_MODEL {}
#[doc = "`write(|w| ..)` method takes [pf_model::W](pf_model::W) writer structure"]
impl crate::Writable for PF_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pf_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_modeh](pf_modeh) module"]
pub type PF_MODEH = crate::Reg<u32, _PF_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_MODEH;
#[doc = "`read()` method returns [pf_modeh::R](pf_modeh::R) reader structure"]
impl crate::Readable for PF_MODEH {}
#[doc = "`write(|w| ..)` method takes [pf_modeh::W](pf_modeh::W) writer structure"]
impl crate::Writable for PF_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pf_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_dout](pf_dout) module"]
pub type PF_DOUT = crate::Reg<u32, _PF_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_DOUT;
#[doc = "`read()` method returns [pf_dout::R](pf_dout::R) reader structure"]
impl crate::Readable for PF_DOUT {}
#[doc = "`write(|w| ..)` method takes [pf_dout::W](pf_dout::W) writer structure"]
impl crate::Writable for PF_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pf_dout;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_douttgl](pf_douttgl) module"]
pub type PF_DOUTTGL = crate::Reg<u32, _PF_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pf_douttgl::W](pf_douttgl::W) writer structure"]
impl crate::Writable for PF_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pf_douttgl;
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_din](pf_din) module"]
pub type PF_DIN = crate::Reg<u32, _PF_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_DIN;
#[doc = "`read()` method returns [pf_din::R](pf_din::R) reader structure"]
impl crate::Readable for PF_DIN {}
#[doc = "Port Data in Register"]
pub mod pf_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_pinlockn](pf_pinlockn) module"]
pub type PF_PINLOCKN = crate::Reg<u32, _PF_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_PINLOCKN;
#[doc = "`read()` method returns [pf_pinlockn::R](pf_pinlockn::R) reader structure"]
impl crate::Readable for PF_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pf_pinlockn::W](pf_pinlockn::W) writer structure"]
impl crate::Writable for PF_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pf_pinlockn;
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_ovtdis](pf_ovtdis) module"]
pub type PF_OVTDIS = crate::Reg<u32, _PF_OVTDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PF_OVTDIS;
#[doc = "`read()` method returns [pf_ovtdis::R](pf_ovtdis::R) reader structure"]
impl crate::Readable for PF_OVTDIS {}
#[doc = "`write(|w| ..)` method takes [pf_ovtdis::W](pf_ovtdis::W) writer structure"]
impl crate::Writable for PF_OVTDIS {}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pf_ovtdis;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_ctrl](pg_ctrl) module"]
pub type PG_CTRL = crate::Reg<u32, _PG_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PG_CTRL;
#[doc = "`read()` method returns [pg_ctrl::R](pg_ctrl::R) reader structure"]
impl crate::Readable for PG_CTRL {}
#[doc = "`write(|w| ..)` method takes [pg_ctrl::W](pg_ctrl::W) writer structure"]
impl crate::Writable for PG_CTRL {}
#[doc = "Port Control Register"]
pub mod pg_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_model](pg_model) module"]
pub type PG_MODEL = crate::Reg<u32, _PG_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PG_MODEL;
#[doc = "`read()` method returns [pg_model::R](pg_model::R) reader structure"]
impl crate::Readable for PG_MODEL {}
#[doc = "`write(|w| ..)` method takes [pg_model::W](pg_model::W) writer structure"]
impl crate::Writable for PG_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pg_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_modeh](pg_modeh) module"]
pub type PG_MODEH = crate::Reg<u32, _PG_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PG_MODEH;
#[doc = "`read()` method returns [pg_modeh::R](pg_modeh::R) reader structure"]
impl crate::Readable for PG_MODEH {}
#[doc = "`write(|w| ..)` method takes [pg_modeh::W](pg_modeh::W) writer structure"]
impl crate::Writable for PG_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pg_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_dout](pg_dout) module"]
pub type PG_DOUT = crate::Reg<u32, _PG_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PG_DOUT;
#[doc = "`read()` method returns [pg_dout::R](pg_dout::R) reader structure"]
impl crate::Readable for PG_DOUT {}
#[doc = "`write(|w| ..)` method takes [pg_dout::W](pg_dout::W) writer structure"]
impl crate::Writable for PG_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pg_dout;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_douttgl](pg_douttgl) module"]
pub type PG_DOUTTGL = crate::Reg<u32, _PG_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PG_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pg_douttgl::W](pg_douttgl::W) writer structure"]
impl crate::Writable for PG_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pg_douttgl;
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_din](pg_din) module"]
pub type PG_DIN = crate::Reg<u32, _PG_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PG_DIN;
#[doc = "`read()` method returns [pg_din::R](pg_din::R) reader structure"]
impl crate::Readable for PG_DIN {}
#[doc = "Port Data in Register"]
pub mod pg_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_pinlockn](pg_pinlockn) module"]
pub type PG_PINLOCKN = crate::Reg<u32, _PG_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PG_PINLOCKN;
#[doc = "`read()` method returns [pg_pinlockn::R](pg_pinlockn::R) reader structure"]
impl crate::Readable for PG_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pg_pinlockn::W](pg_pinlockn::W) writer structure"]
impl crate::Writable for PG_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pg_pinlockn;
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pg_ovtdis](pg_ovtdis) module"]
pub type PG_OVTDIS = crate::Reg<u32, _PG_OVTDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PG_OVTDIS;
#[doc = "`read()` method returns [pg_ovtdis::R](pg_ovtdis::R) reader structure"]
impl crate::Readable for PG_OVTDIS {}
#[doc = "`write(|w| ..)` method takes [pg_ovtdis::W](pg_ovtdis::W) writer structure"]
impl crate::Writable for PG_OVTDIS {}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pg_ovtdis;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ph_ctrl](ph_ctrl) module"]
pub type PH_CTRL = crate::Reg<u32, _PH_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PH_CTRL;
#[doc = "`read()` method returns [ph_ctrl::R](ph_ctrl::R) reader structure"]
impl crate::Readable for PH_CTRL {}
#[doc = "`write(|w| ..)` method takes [ph_ctrl::W](ph_ctrl::W) writer structure"]
impl crate::Writable for PH_CTRL {}
#[doc = "Port Control Register"]
pub mod ph_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ph_model](ph_model) module"]
pub type PH_MODEL = crate::Reg<u32, _PH_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PH_MODEL;
#[doc = "`read()` method returns [ph_model::R](ph_model::R) reader structure"]
impl crate::Readable for PH_MODEL {}
#[doc = "`write(|w| ..)` method takes [ph_model::W](ph_model::W) writer structure"]
impl crate::Writable for PH_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod ph_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ph_modeh](ph_modeh) module"]
pub type PH_MODEH = crate::Reg<u32, _PH_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PH_MODEH;
#[doc = "`read()` method returns [ph_modeh::R](ph_modeh::R) reader structure"]
impl crate::Readable for PH_MODEH {}
#[doc = "`write(|w| ..)` method takes [ph_modeh::W](ph_modeh::W) writer structure"]
impl crate::Writable for PH_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod ph_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ph_dout](ph_dout) module"]
pub type PH_DOUT = crate::Reg<u32, _PH_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PH_DOUT;
#[doc = "`read()` method returns [ph_dout::R](ph_dout::R) reader structure"]
impl crate::Readable for PH_DOUT {}
#[doc = "`write(|w| ..)` method takes [ph_dout::W](ph_dout::W) writer structure"]
impl crate::Writable for PH_DOUT {}
#[doc = "Port Data Out Register"]
pub mod ph_dout;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ph_douttgl](ph_douttgl) module"]
pub type PH_DOUTTGL = crate::Reg<u32, _PH_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PH_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [ph_douttgl::W](ph_douttgl::W) writer structure"]
impl crate::Writable for PH_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod ph_douttgl;
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ph_din](ph_din) module"]
pub type PH_DIN = crate::Reg<u32, _PH_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PH_DIN;
#[doc = "`read()` method returns [ph_din::R](ph_din::R) reader structure"]
impl crate::Readable for PH_DIN {}
#[doc = "Port Data in Register"]
pub mod ph_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ph_pinlockn](ph_pinlockn) module"]
pub type PH_PINLOCKN = crate::Reg<u32, _PH_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PH_PINLOCKN;
#[doc = "`read()` method returns [ph_pinlockn::R](ph_pinlockn::R) reader structure"]
impl crate::Readable for PH_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [ph_pinlockn::W](ph_pinlockn::W) writer structure"]
impl crate::Writable for PH_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod ph_pinlockn;
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ph_ovtdis](ph_ovtdis) module"]
pub type PH_OVTDIS = crate::Reg<u32, _PH_OVTDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PH_OVTDIS;
#[doc = "`read()` method returns [ph_ovtdis::R](ph_ovtdis::R) reader structure"]
impl crate::Readable for PH_OVTDIS {}
#[doc = "`write(|w| ..)` method takes [ph_ovtdis::W](ph_ovtdis::W) writer structure"]
impl crate::Writable for PH_OVTDIS {}
#[doc = "Over Voltage Disable for All Modes"]
pub mod ph_ovtdis;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pi_ctrl](pi_ctrl) module"]
pub type PI_CTRL = crate::Reg<u32, _PI_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PI_CTRL;
#[doc = "`read()` method returns [pi_ctrl::R](pi_ctrl::R) reader structure"]
impl crate::Readable for PI_CTRL {}
#[doc = "`write(|w| ..)` method takes [pi_ctrl::W](pi_ctrl::W) writer structure"]
impl crate::Writable for PI_CTRL {}
#[doc = "Port Control Register"]
pub mod pi_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pi_model](pi_model) module"]
pub type PI_MODEL = crate::Reg<u32, _PI_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PI_MODEL;
#[doc = "`read()` method returns [pi_model::R](pi_model::R) reader structure"]
impl crate::Readable for PI_MODEL {}
#[doc = "`write(|w| ..)` method takes [pi_model::W](pi_model::W) writer structure"]
impl crate::Writable for PI_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pi_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pi_modeh](pi_modeh) module"]
pub type PI_MODEH = crate::Reg<u32, _PI_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PI_MODEH;
#[doc = "`read()` method returns [pi_modeh::R](pi_modeh::R) reader structure"]
impl crate::Readable for PI_MODEH {}
#[doc = "`write(|w| ..)` method takes [pi_modeh::W](pi_modeh::W) writer structure"]
impl crate::Writable for PI_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pi_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pi_dout](pi_dout) module"]
pub type PI_DOUT = crate::Reg<u32, _PI_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PI_DOUT;
#[doc = "`read()` method returns [pi_dout::R](pi_dout::R) reader structure"]
impl crate::Readable for PI_DOUT {}
#[doc = "`write(|w| ..)` method takes [pi_dout::W](pi_dout::W) writer structure"]
impl crate::Writable for PI_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pi_dout;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pi_douttgl](pi_douttgl) module"]
pub type PI_DOUTTGL = crate::Reg<u32, _PI_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PI_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pi_douttgl::W](pi_douttgl::W) writer structure"]
impl crate::Writable for PI_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pi_douttgl;
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pi_din](pi_din) module"]
pub type PI_DIN = crate::Reg<u32, _PI_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PI_DIN;
#[doc = "`read()` method returns [pi_din::R](pi_din::R) reader structure"]
impl crate::Readable for PI_DIN {}
#[doc = "Port Data in Register"]
pub mod pi_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pi_pinlockn](pi_pinlockn) module"]
pub type PI_PINLOCKN = crate::Reg<u32, _PI_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PI_PINLOCKN;
#[doc = "`read()` method returns [pi_pinlockn::R](pi_pinlockn::R) reader structure"]
impl crate::Readable for PI_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pi_pinlockn::W](pi_pinlockn::W) writer structure"]
impl crate::Writable for PI_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pi_pinlockn;
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pi_ovtdis](pi_ovtdis) module"]
pub type PI_OVTDIS = crate::Reg<u32, _PI_OVTDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PI_OVTDIS;
#[doc = "`read()` method returns [pi_ovtdis::R](pi_ovtdis::R) reader structure"]
impl crate::Readable for PI_OVTDIS {}
#[doc = "`write(|w| ..)` method takes [pi_ovtdis::W](pi_ovtdis::W) writer structure"]
impl crate::Writable for PI_OVTDIS {}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pi_ovtdis;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pj_ctrl](pj_ctrl) module"]
pub type PJ_CTRL = crate::Reg<u32, _PJ_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJ_CTRL;
#[doc = "`read()` method returns [pj_ctrl::R](pj_ctrl::R) reader structure"]
impl crate::Readable for PJ_CTRL {}
#[doc = "`write(|w| ..)` method takes [pj_ctrl::W](pj_ctrl::W) writer structure"]
impl crate::Writable for PJ_CTRL {}
#[doc = "Port Control Register"]
pub mod pj_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pj_model](pj_model) module"]
pub type PJ_MODEL = crate::Reg<u32, _PJ_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJ_MODEL;
#[doc = "`read()` method returns [pj_model::R](pj_model::R) reader structure"]
impl crate::Readable for PJ_MODEL {}
#[doc = "`write(|w| ..)` method takes [pj_model::W](pj_model::W) writer structure"]
impl crate::Writable for PJ_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pj_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pj_modeh](pj_modeh) module"]
pub type PJ_MODEH = crate::Reg<u32, _PJ_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJ_MODEH;
#[doc = "`read()` method returns [pj_modeh::R](pj_modeh::R) reader structure"]
impl crate::Readable for PJ_MODEH {}
#[doc = "`write(|w| ..)` method takes [pj_modeh::W](pj_modeh::W) writer structure"]
impl crate::Writable for PJ_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pj_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pj_dout](pj_dout) module"]
pub type PJ_DOUT = crate::Reg<u32, _PJ_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJ_DOUT;
#[doc = "`read()` method returns [pj_dout::R](pj_dout::R) reader structure"]
impl crate::Readable for PJ_DOUT {}
#[doc = "`write(|w| ..)` method takes [pj_dout::W](pj_dout::W) writer structure"]
impl crate::Writable for PJ_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pj_dout;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pj_douttgl](pj_douttgl) module"]
pub type PJ_DOUTTGL = crate::Reg<u32, _PJ_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJ_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pj_douttgl::W](pj_douttgl::W) writer structure"]
impl crate::Writable for PJ_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pj_douttgl;
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pj_din](pj_din) module"]
pub type PJ_DIN = crate::Reg<u32, _PJ_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJ_DIN;
#[doc = "`read()` method returns [pj_din::R](pj_din::R) reader structure"]
impl crate::Readable for PJ_DIN {}
#[doc = "Port Data in Register"]
pub mod pj_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pj_pinlockn](pj_pinlockn) module"]
pub type PJ_PINLOCKN = crate::Reg<u32, _PJ_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJ_PINLOCKN;
#[doc = "`read()` method returns [pj_pinlockn::R](pj_pinlockn::R) reader structure"]
impl crate::Readable for PJ_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pj_pinlockn::W](pj_pinlockn::W) writer structure"]
impl crate::Writable for PJ_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pj_pinlockn;
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pj_ovtdis](pj_ovtdis) module"]
pub type PJ_OVTDIS = crate::Reg<u32, _PJ_OVTDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PJ_OVTDIS;
#[doc = "`read()` method returns [pj_ovtdis::R](pj_ovtdis::R) reader structure"]
impl crate::Readable for PJ_OVTDIS {}
#[doc = "`write(|w| ..)` method takes [pj_ovtdis::W](pj_ovtdis::W) writer structure"]
impl crate::Writable for PJ_OVTDIS {}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pj_ovtdis;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pk_ctrl](pk_ctrl) module"]
pub type PK_CTRL = crate::Reg<u32, _PK_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PK_CTRL;
#[doc = "`read()` method returns [pk_ctrl::R](pk_ctrl::R) reader structure"]
impl crate::Readable for PK_CTRL {}
#[doc = "`write(|w| ..)` method takes [pk_ctrl::W](pk_ctrl::W) writer structure"]
impl crate::Writable for PK_CTRL {}
#[doc = "Port Control Register"]
pub mod pk_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pk_model](pk_model) module"]
pub type PK_MODEL = crate::Reg<u32, _PK_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PK_MODEL;
#[doc = "`read()` method returns [pk_model::R](pk_model::R) reader structure"]
impl crate::Readable for PK_MODEL {}
#[doc = "`write(|w| ..)` method takes [pk_model::W](pk_model::W) writer structure"]
impl crate::Writable for PK_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pk_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pk_modeh](pk_modeh) module"]
pub type PK_MODEH = crate::Reg<u32, _PK_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PK_MODEH;
#[doc = "`read()` method returns [pk_modeh::R](pk_modeh::R) reader structure"]
impl crate::Readable for PK_MODEH {}
#[doc = "`write(|w| ..)` method takes [pk_modeh::W](pk_modeh::W) writer structure"]
impl crate::Writable for PK_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pk_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pk_dout](pk_dout) module"]
pub type PK_DOUT = crate::Reg<u32, _PK_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PK_DOUT;
#[doc = "`read()` method returns [pk_dout::R](pk_dout::R) reader structure"]
impl crate::Readable for PK_DOUT {}
#[doc = "`write(|w| ..)` method takes [pk_dout::W](pk_dout::W) writer structure"]
impl crate::Writable for PK_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pk_dout;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pk_douttgl](pk_douttgl) module"]
pub type PK_DOUTTGL = crate::Reg<u32, _PK_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PK_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pk_douttgl::W](pk_douttgl::W) writer structure"]
impl crate::Writable for PK_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pk_douttgl;
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pk_din](pk_din) module"]
pub type PK_DIN = crate::Reg<u32, _PK_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PK_DIN;
#[doc = "`read()` method returns [pk_din::R](pk_din::R) reader structure"]
impl crate::Readable for PK_DIN {}
#[doc = "Port Data in Register"]
pub mod pk_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pk_pinlockn](pk_pinlockn) module"]
pub type PK_PINLOCKN = crate::Reg<u32, _PK_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PK_PINLOCKN;
#[doc = "`read()` method returns [pk_pinlockn::R](pk_pinlockn::R) reader structure"]
impl crate::Readable for PK_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pk_pinlockn::W](pk_pinlockn::W) writer structure"]
impl crate::Writable for PK_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pk_pinlockn;
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pk_ovtdis](pk_ovtdis) module"]
pub type PK_OVTDIS = crate::Reg<u32, _PK_OVTDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PK_OVTDIS;
#[doc = "`read()` method returns [pk_ovtdis::R](pk_ovtdis::R) reader structure"]
impl crate::Readable for PK_OVTDIS {}
#[doc = "`write(|w| ..)` method takes [pk_ovtdis::W](pk_ovtdis::W) writer structure"]
impl crate::Writable for PK_OVTDIS {}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pk_ovtdis;
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl_ctrl](pl_ctrl) module"]
pub type PL_CTRL = crate::Reg<u32, _PL_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL_CTRL;
#[doc = "`read()` method returns [pl_ctrl::R](pl_ctrl::R) reader structure"]
impl crate::Readable for PL_CTRL {}
#[doc = "`write(|w| ..)` method takes [pl_ctrl::W](pl_ctrl::W) writer structure"]
impl crate::Writable for PL_CTRL {}
#[doc = "Port Control Register"]
pub mod pl_ctrl;
#[doc = "Port Pin Mode Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl_model](pl_model) module"]
pub type PL_MODEL = crate::Reg<u32, _PL_MODEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL_MODEL;
#[doc = "`read()` method returns [pl_model::R](pl_model::R) reader structure"]
impl crate::Readable for PL_MODEL {}
#[doc = "`write(|w| ..)` method takes [pl_model::W](pl_model::W) writer structure"]
impl crate::Writable for PL_MODEL {}
#[doc = "Port Pin Mode Low Register"]
pub mod pl_model;
#[doc = "Port Pin Mode High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl_modeh](pl_modeh) module"]
pub type PL_MODEH = crate::Reg<u32, _PL_MODEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL_MODEH;
#[doc = "`read()` method returns [pl_modeh::R](pl_modeh::R) reader structure"]
impl crate::Readable for PL_MODEH {}
#[doc = "`write(|w| ..)` method takes [pl_modeh::W](pl_modeh::W) writer structure"]
impl crate::Writable for PL_MODEH {}
#[doc = "Port Pin Mode High Register"]
pub mod pl_modeh;
#[doc = "Port Data Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl_dout](pl_dout) module"]
pub type PL_DOUT = crate::Reg<u32, _PL_DOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL_DOUT;
#[doc = "`read()` method returns [pl_dout::R](pl_dout::R) reader structure"]
impl crate::Readable for PL_DOUT {}
#[doc = "`write(|w| ..)` method takes [pl_dout::W](pl_dout::W) writer structure"]
impl crate::Writable for PL_DOUT {}
#[doc = "Port Data Out Register"]
pub mod pl_dout;
#[doc = "Port Data Out Toggle Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl_douttgl](pl_douttgl) module"]
pub type PL_DOUTTGL = crate::Reg<u32, _PL_DOUTTGL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL_DOUTTGL;
#[doc = "`write(|w| ..)` method takes [pl_douttgl::W](pl_douttgl::W) writer structure"]
impl crate::Writable for PL_DOUTTGL {}
#[doc = "Port Data Out Toggle Register"]
pub mod pl_douttgl;
#[doc = "Port Data in Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl_din](pl_din) module"]
pub type PL_DIN = crate::Reg<u32, _PL_DIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL_DIN;
#[doc = "`read()` method returns [pl_din::R](pl_din::R) reader structure"]
impl crate::Readable for PL_DIN {}
#[doc = "Port Data in Register"]
pub mod pl_din;
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl_pinlockn](pl_pinlockn) module"]
pub type PL_PINLOCKN = crate::Reg<u32, _PL_PINLOCKN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL_PINLOCKN;
#[doc = "`read()` method returns [pl_pinlockn::R](pl_pinlockn::R) reader structure"]
impl crate::Readable for PL_PINLOCKN {}
#[doc = "`write(|w| ..)` method takes [pl_pinlockn::W](pl_pinlockn::W) writer structure"]
impl crate::Writable for PL_PINLOCKN {}
#[doc = "Port Unlocked Pins Register"]
pub mod pl_pinlockn;
#[doc = "Over Voltage Disable for All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pl_ovtdis](pl_ovtdis) module"]
pub type PL_OVTDIS = crate::Reg<u32, _PL_OVTDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PL_OVTDIS;
#[doc = "`read()` method returns [pl_ovtdis::R](pl_ovtdis::R) reader structure"]
impl crate::Readable for PL_OVTDIS {}
#[doc = "`write(|w| ..)` method takes [pl_ovtdis::W](pl_ovtdis::W) writer structure"]
impl crate::Writable for PL_OVTDIS {}
#[doc = "Over Voltage Disable for All Modes"]
pub mod pl_ovtdis;
#[doc = "External Interrupt Port Select Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extipsell](extipsell) module"]
pub type EXTIPSELL = crate::Reg<u32, _EXTIPSELL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTIPSELL;
#[doc = "`read()` method returns [extipsell::R](extipsell::R) reader structure"]
impl crate::Readable for EXTIPSELL {}
#[doc = "`write(|w| ..)` method takes [extipsell::W](extipsell::W) writer structure"]
impl crate::Writable for EXTIPSELL {}
#[doc = "External Interrupt Port Select Low Register"]
pub mod extipsell;
#[doc = "External Interrupt Port Select High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extipselh](extipselh) module"]
pub type EXTIPSELH = crate::Reg<u32, _EXTIPSELH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTIPSELH;
#[doc = "`read()` method returns [extipselh::R](extipselh::R) reader structure"]
impl crate::Readable for EXTIPSELH {}
#[doc = "`write(|w| ..)` method takes [extipselh::W](extipselh::W) writer structure"]
impl crate::Writable for EXTIPSELH {}
#[doc = "External Interrupt Port Select High Register"]
pub mod extipselh;
#[doc = "External Interrupt Pin Select Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extipinsell](extipinsell) module"]
pub type EXTIPINSELL = crate::Reg<u32, _EXTIPINSELL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTIPINSELL;
#[doc = "`read()` method returns [extipinsell::R](extipinsell::R) reader structure"]
impl crate::Readable for EXTIPINSELL {}
#[doc = "`write(|w| ..)` method takes [extipinsell::W](extipinsell::W) writer structure"]
impl crate::Writable for EXTIPINSELL {}
#[doc = "External Interrupt Pin Select Low Register"]
pub mod extipinsell;
#[doc = "External Interrupt Pin Select High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extipinselh](extipinselh) module"]
pub type EXTIPINSELH = crate::Reg<u32, _EXTIPINSELH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTIPINSELH;
#[doc = "`read()` method returns [extipinselh::R](extipinselh::R) reader structure"]
impl crate::Readable for EXTIPINSELH {}
#[doc = "`write(|w| ..)` method takes [extipinselh::W](extipinselh::W) writer structure"]
impl crate::Writable for EXTIPINSELH {}
#[doc = "External Interrupt Pin Select High Register"]
pub mod extipinselh;
#[doc = "External Interrupt Rising Edge Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extirise](extirise) module"]
pub type EXTIRISE = crate::Reg<u32, _EXTIRISE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTIRISE;
#[doc = "`read()` method returns [extirise::R](extirise::R) reader structure"]
impl crate::Readable for EXTIRISE {}
#[doc = "`write(|w| ..)` method takes [extirise::W](extirise::W) writer structure"]
impl crate::Writable for EXTIRISE {}
#[doc = "External Interrupt Rising Edge Trigger Register"]
pub mod extirise;
#[doc = "External Interrupt Falling Edge Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extifall](extifall) module"]
pub type EXTIFALL = crate::Reg<u32, _EXTIFALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTIFALL;
#[doc = "`read()` method returns [extifall::R](extifall::R) reader structure"]
impl crate::Readable for EXTIFALL {}
#[doc = "`write(|w| ..)` method takes [extifall::W](extifall::W) writer structure"]
impl crate::Writable for EXTIFALL {}
#[doc = "External Interrupt Falling Edge Trigger Register"]
pub mod extifall;
#[doc = "External Interrupt Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extilevel](extilevel) module"]
pub type EXTILEVEL = crate::Reg<u32, _EXTILEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTILEVEL;
#[doc = "`read()` method returns [extilevel::R](extilevel::R) reader structure"]
impl crate::Readable for EXTILEVEL {}
#[doc = "`write(|w| ..)` method takes [extilevel::W](extilevel::W) writer structure"]
impl crate::Writable for EXTILEVEL {}
#[doc = "External Interrupt Level Register"]
pub mod extilevel;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](ifs) module"]
pub type IFS = crate::Reg<u32, _IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS;
#[doc = "`write(|w| ..)` method takes [ifs::W](ifs::W) writer structure"]
impl crate::Writable for IFS {}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](ifc) module"]
pub type IFC = crate::Reg<u32, _IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFC;
#[doc = "`write(|w| ..)` method takes [ifc::W](ifc::W) writer structure"]
impl crate::Writable for IFC {}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](ien) module"]
pub type IEN = crate::Reg<u32, _IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEN;
#[doc = "`read()` method returns [ien::R](ien::R) reader structure"]
impl crate::Readable for IEN {}
#[doc = "`write(|w| ..)` method takes [ien::W](ien::W) writer structure"]
impl crate::Writable for IEN {}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "EM4 Wake Up Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em4wuen](em4wuen) module"]
pub type EM4WUEN = crate::Reg<u32, _EM4WUEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EM4WUEN;
#[doc = "`read()` method returns [em4wuen::R](em4wuen::R) reader structure"]
impl crate::Readable for EM4WUEN {}
#[doc = "`write(|w| ..)` method takes [em4wuen::W](em4wuen::W) writer structure"]
impl crate::Writable for EM4WUEN {}
#[doc = "EM4 Wake Up Enable Register"]
pub mod em4wuen;
#[doc = "I/O Routing Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](routepen) module"]
pub type ROUTEPEN = crate::Reg<u32, _ROUTEPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTEPEN;
#[doc = "`read()` method returns [routepen::R](routepen::R) reader structure"]
impl crate::Readable for ROUTEPEN {}
#[doc = "`write(|w| ..)` method takes [routepen::W](routepen::W) writer structure"]
impl crate::Writable for ROUTEPEN {}
#[doc = "I/O Routing Pin Enable Register"]
pub mod routepen;
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc0](routeloc0) module"]
pub type ROUTELOC0 = crate::Reg<u32, _ROUTELOC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC0;
#[doc = "`read()` method returns [routeloc0::R](routeloc0::R) reader structure"]
impl crate::Readable for ROUTELOC0 {}
#[doc = "`write(|w| ..)` method takes [routeloc0::W](routeloc0::W) writer structure"]
impl crate::Writable for ROUTELOC0 {}
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "Input Sense Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [insense](insense) module"]
pub type INSENSE = crate::Reg<u32, _INSENSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INSENSE;
#[doc = "`read()` method returns [insense::R](insense::R) reader structure"]
impl crate::Readable for INSENSE {}
#[doc = "`write(|w| ..)` method takes [insense::W](insense::W) writer structure"]
impl crate::Writable for INSENSE {}
#[doc = "Input Sense Register"]
pub mod insense;
#[doc = "Configuration Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Configuration Lock Register"]
pub mod lock;
