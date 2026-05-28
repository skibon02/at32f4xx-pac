#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    psr: PSR,
    _reserved_1_usd: [u8; 0x80],
    _reserved2: [u8; 0x44],
    slib_sts2: SLIB_STS2,
    slib_sts0: SLIB_STS0,
    slib_sts1: SLIB_STS1,
    slib_pwd_clr: SLIB_PWD_CLR,
    slib_misc_sts: SLIB_MISC_STS,
    slib_set_pwd: SLIB_SET_PWD,
    slib_set_range0: SLIB_SET_RANGE0,
    slib_set_range1: SLIB_SET_RANGE1,
    _reserved10: [u8; 0x08],
    slib_unlock: SLIB_UNLOCK,
    crc_ctrl: CRC_CTRL,
    crc_chkr: CRC_CHKR,
}
impl RegisterBlock {
    #[doc = "0x00 - Performance selection register"]
    #[inline(always)]
    pub const fn psr(&self) -> &PSR {
        &self.psr
    }
    #[doc = "0x04..0x84 - Bank %s registers"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of cluster in the array. `n == 0` corresponds to `Bank1` cluster.</div>"]
    #[inline(always)]
    pub const fn bank(&self, n: usize) -> &Bank {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x84 - Bank %s registers"]
    #[inline(always)]
    pub fn bank_iter(&self) -> impl Iterator<Item = &Bank> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(64 * n)
                .cast()
        })
    }
    #[doc = "0x04..0x44 - Bank 1 registers"]
    #[inline(always)]
    pub const fn bank1(&self) -> &Bank {
        self.bank(0)
    }
    #[doc = "0x44..0x84 - Bank 2 registers"]
    #[inline(always)]
    pub const fn bank2(&self) -> &Bank {
        self.bank(1)
    }
    #[doc = "0x08 - USD unlock register"]
    #[inline(always)]
    pub const fn usd_unlock(&self) -> &USD_UNLOCK {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x10 - Control register"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x1c - User system data register"]
    #[inline(always)]
    pub const fn usd(&self) -> &USD {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x20 - Erase/program protection status register 0"]
    #[inline(always)]
    pub const fn epps0(&self) -> &EPPS0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x2c - Erase/program protection status register 1"]
    #[inline(always)]
    pub const fn epps1(&self) -> &EPPS1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x50 - Control 2 register"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x58 - Flash continue read register"]
    #[inline(always)]
    pub const fn contr(&self) -> &CONTR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x60 - Flash divider register"]
    #[inline(always)]
    pub const fn divr(&self) -> &DIVR {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0xc8 - sLib status 2 register"]
    #[inline(always)]
    pub const fn slib_sts2(&self) -> &SLIB_STS2 {
        &self.slib_sts2
    }
    #[doc = "0xcc - sLib status 0 register"]
    #[inline(always)]
    pub const fn slib_sts0(&self) -> &SLIB_STS0 {
        &self.slib_sts0
    }
    #[doc = "0xd0 - sLib status 1 register"]
    #[inline(always)]
    pub const fn slib_sts1(&self) -> &SLIB_STS1 {
        &self.slib_sts1
    }
    #[doc = "0xd4 - SLIB password clear register"]
    #[inline(always)]
    pub const fn slib_pwd_clr(&self) -> &SLIB_PWD_CLR {
        &self.slib_pwd_clr
    }
    #[doc = "0xd8 - sLib misc status register"]
    #[inline(always)]
    pub const fn slib_misc_sts(&self) -> &SLIB_MISC_STS {
        &self.slib_misc_sts
    }
    #[doc = "0xdc - sLib password setting register"]
    #[inline(always)]
    pub const fn slib_set_pwd(&self) -> &SLIB_SET_PWD {
        &self.slib_set_pwd
    }
    #[doc = "0xe0 - Configure sLib range register 0"]
    #[inline(always)]
    pub const fn slib_set_range0(&self) -> &SLIB_SET_RANGE0 {
        &self.slib_set_range0
    }
    #[doc = "0xe4 - Configure sLib range register 1"]
    #[inline(always)]
    pub const fn slib_set_range1(&self) -> &SLIB_SET_RANGE1 {
        &self.slib_set_range1
    }
    #[doc = "0xf0 - sLib unlock register"]
    #[inline(always)]
    pub const fn slib_unlock(&self) -> &SLIB_UNLOCK {
        &self.slib_unlock
    }
    #[doc = "0xf4 - CRC controler register"]
    #[inline(always)]
    pub const fn crc_ctrl(&self) -> &CRC_CTRL {
        &self.crc_ctrl
    }
    #[doc = "0xf8 - CRC check result register"]
    #[inline(always)]
    pub const fn crc_chkr(&self) -> &CRC_CHKR {
        &self.crc_chkr
    }
}
#[doc = "PSR (rw) register accessor: Performance selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`] module"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Performance selection register"]
pub mod psr;
#[doc = "Bank %s registers"]
pub use self::bank::Bank;
#[doc = r"Cluster"]
#[doc = "Bank %s registers"]
pub mod bank;
#[doc = "USD_UNLOCK (w) register accessor: USD unlock register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usd_unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usd_unlock`] module"]
pub type USD_UNLOCK = crate::Reg<usd_unlock::USD_UNLOCK_SPEC>;
#[doc = "USD unlock register"]
pub mod usd_unlock;
#[doc = "CTRL1 (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control register"]
pub mod ctrl1;
#[doc = "USD (r) register accessor: User system data register\n\nYou can [`read`](crate::Reg::read) this register and get [`usd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usd`] module"]
pub type USD = crate::Reg<usd::USD_SPEC>;
#[doc = "User system data register"]
pub mod usd;
#[doc = "EPPS0 (r) register accessor: Erase/program protection status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`epps0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epps0`] module"]
pub type EPPS0 = crate::Reg<epps0::EPPS0_SPEC>;
#[doc = "Erase/program protection status register 0"]
pub mod epps0;
#[doc = "EPPS1 (r) register accessor: Erase/program protection status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`epps1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epps1`] module"]
pub type EPPS1 = crate::Reg<epps1::EPPS1_SPEC>;
#[doc = "Erase/program protection status register 1"]
pub mod epps1;
#[doc = "CTRL2 (rw) register accessor: Control 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control 2 register"]
pub mod ctrl2;
#[doc = "CONTR (rw) register accessor: Flash continue read register\n\nYou can [`read`](crate::Reg::read) this register and get [`contr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`contr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@contr`] module"]
pub type CONTR = crate::Reg<contr::CONTR_SPEC>;
#[doc = "Flash continue read register"]
pub mod contr;
#[doc = "DIVR (rw) register accessor: Flash divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divr`] module"]
pub type DIVR = crate::Reg<divr::DIVR_SPEC>;
#[doc = "Flash divider register"]
pub mod divr;
#[doc = "SLIB_STS2 (rw) register accessor: sLib status 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`slib_sts2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_sts2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_sts2`] module"]
pub type SLIB_STS2 = crate::Reg<slib_sts2::SLIB_STS2_SPEC>;
#[doc = "sLib status 2 register"]
pub mod slib_sts2;
#[doc = "SLIB_STS0 (rw) register accessor: sLib status 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`slib_sts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_sts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_sts0`] module"]
pub type SLIB_STS0 = crate::Reg<slib_sts0::SLIB_STS0_SPEC>;
#[doc = "sLib status 0 register"]
pub mod slib_sts0;
#[doc = "SLIB_STS1 (rw) register accessor: sLib status 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`slib_sts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_sts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_sts1`] module"]
pub type SLIB_STS1 = crate::Reg<slib_sts1::SLIB_STS1_SPEC>;
#[doc = "sLib status 1 register"]
pub mod slib_sts1;
#[doc = "SLIB_PWD_CLR (w) register accessor: SLIB password clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_pwd_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_pwd_clr`] module"]
pub type SLIB_PWD_CLR = crate::Reg<slib_pwd_clr::SLIB_PWD_CLR_SPEC>;
#[doc = "SLIB password clear register"]
pub mod slib_pwd_clr;
#[doc = "SLIB_MISC_STS (rw) register accessor: sLib misc status register\n\nYou can [`read`](crate::Reg::read) this register and get [`slib_misc_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_misc_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_misc_sts`] module"]
pub type SLIB_MISC_STS = crate::Reg<slib_misc_sts::SLIB_MISC_STS_SPEC>;
#[doc = "sLib misc status register"]
pub mod slib_misc_sts;
#[doc = "SLIB_SET_PWD (w) register accessor: sLib password setting register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_set_pwd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_set_pwd`] module"]
pub type SLIB_SET_PWD = crate::Reg<slib_set_pwd::SLIB_SET_PWD_SPEC>;
#[doc = "sLib password setting register"]
pub mod slib_set_pwd;
#[doc = "SLIB_SET_RANGE0 (w) register accessor: Configure sLib range register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_set_range0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_set_range0`] module"]
pub type SLIB_SET_RANGE0 = crate::Reg<slib_set_range0::SLIB_SET_RANGE0_SPEC>;
#[doc = "Configure sLib range register 0"]
pub mod slib_set_range0;
#[doc = "SLIB_SET_RANGE1 (w) register accessor: Configure sLib range register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_set_range1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_set_range1`] module"]
pub type SLIB_SET_RANGE1 = crate::Reg<slib_set_range1::SLIB_SET_RANGE1_SPEC>;
#[doc = "Configure sLib range register 1"]
pub mod slib_set_range1;
#[doc = "SLIB_UNLOCK (w) register accessor: sLib unlock register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_unlock`] module"]
pub type SLIB_UNLOCK = crate::Reg<slib_unlock::SLIB_UNLOCK_SPEC>;
#[doc = "sLib unlock register"]
pub mod slib_unlock;
#[doc = "CRC_CTRL (w) register accessor: CRC controler register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_ctrl`] module"]
pub type CRC_CTRL = crate::Reg<crc_ctrl::CRC_CTRL_SPEC>;
#[doc = "CRC controler register"]
pub mod crc_ctrl;
#[doc = "CRC_CHKR (r) register accessor: CRC check result register\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_chkr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_chkr`] module"]
pub type CRC_CHKR = crate::Reg<crc_chkr::CRC_CHKR_SPEC>;
#[doc = "CRC check result register"]
pub mod crc_chkr;
