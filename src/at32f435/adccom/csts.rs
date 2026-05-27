#[doc = "Register `CSTS` reader"]
pub type R = crate::R<CSTS_SPEC>;
#[doc = "Field `VMOR(1-3)` reader - ADC%s voltage monitoring out of range flag"]
pub type VMOR_R = crate::BitReader;
#[doc = "Field `OCCE(1-3)` reader - ADC%s ordinary channel conversion end flag"]
pub type OCCE_R = crate::BitReader;
#[doc = "Field `PCCE(1-3)` reader - ADC%s preempted channel conversion end flag"]
pub type PCCE_R = crate::BitReader;
#[doc = "Field `PCCS(1-3)` reader - ADC%s preempted channel conversion start flag"]
pub type PCCS_R = crate::BitReader;
#[doc = "Field `OCCS(1-3)` reader - ADC%s ordinary channel conversion start flag"]
pub type OCCS_R = crate::BitReader;
#[doc = "Field `OCCO(1-3)` reader - ADC%s overrun flag"]
pub type OCCO_R = crate::BitReader;
#[doc = "Field `RDY(1-3)` reader - ADC%s conversion ready flag"]
pub type RDY_R = crate::BitReader;
impl R {
    #[doc = "ADC(1-3) voltage monitoring out of range flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `VMOR1` field.</div>"]
    #[inline(always)]
    pub fn vmor(&self, n: u8) -> VMOR_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        VMOR_R::new(((self.bits >> (n * 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "ADC(1-3) voltage monitoring out of range flag"]
    #[inline(always)]
    pub fn vmor_iter(&self) -> impl Iterator<Item = VMOR_R> + '_ {
        (0..3).map(move |n| VMOR_R::new(((self.bits >> (n * 8)) & 1) != 0))
    }
    #[doc = "Bit 0 - ADC1 voltage monitoring out of range flag"]
    #[inline(always)]
    pub fn vmor1(&self) -> VMOR_R {
        VMOR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - ADC2 voltage monitoring out of range flag"]
    #[inline(always)]
    pub fn vmor2(&self) -> VMOR_R {
        VMOR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC3 voltage monitoring out of range flag"]
    #[inline(always)]
    pub fn vmor3(&self) -> VMOR_R {
        VMOR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "ADC(1-3) ordinary channel conversion end flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OCCE1` field.</div>"]
    #[inline(always)]
    pub fn occe(&self, n: u8) -> OCCE_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OCCE_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "ADC(1-3) ordinary channel conversion end flag"]
    #[inline(always)]
    pub fn occe_iter(&self) -> impl Iterator<Item = OCCE_R> + '_ {
        (0..3).map(move |n| OCCE_R::new(((self.bits >> (n * 8 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - ADC1 ordinary channel conversion end flag"]
    #[inline(always)]
    pub fn occe1(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC2 ordinary channel conversion end flag"]
    #[inline(always)]
    pub fn occe2(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC3 ordinary channel conversion end flag"]
    #[inline(always)]
    pub fn occe3(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "ADC(1-3) preempted channel conversion end flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PCCE1` field.</div>"]
    #[inline(always)]
    pub fn pcce(&self, n: u8) -> PCCE_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        PCCE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "ADC(1-3) preempted channel conversion end flag"]
    #[inline(always)]
    pub fn pcce_iter(&self) -> impl Iterator<Item = PCCE_R> + '_ {
        (0..3).map(move |n| PCCE_R::new(((self.bits >> (n * 8 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - ADC1 preempted channel conversion end flag"]
    #[inline(always)]
    pub fn pcce1(&self) -> PCCE_R {
        PCCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC2 preempted channel conversion end flag"]
    #[inline(always)]
    pub fn pcce2(&self) -> PCCE_R {
        PCCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC3 preempted channel conversion end flag"]
    #[inline(always)]
    pub fn pcce3(&self) -> PCCE_R {
        PCCE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "ADC(1-3) preempted channel conversion start flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PCCS1` field.</div>"]
    #[inline(always)]
    pub fn pccs(&self, n: u8) -> PCCS_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        PCCS_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "ADC(1-3) preempted channel conversion start flag"]
    #[inline(always)]
    pub fn pccs_iter(&self) -> impl Iterator<Item = PCCS_R> + '_ {
        (0..3).map(move |n| PCCS_R::new(((self.bits >> (n * 8 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - ADC1 preempted channel conversion start flag"]
    #[inline(always)]
    pub fn pccs1(&self) -> PCCS_R {
        PCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC2 preempted channel conversion start flag"]
    #[inline(always)]
    pub fn pccs2(&self) -> PCCS_R {
        PCCS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC3 preempted channel conversion start flag"]
    #[inline(always)]
    pub fn pccs3(&self) -> PCCS_R {
        PCCS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "ADC(1-3) ordinary channel conversion start flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OCCS1` field.</div>"]
    #[inline(always)]
    pub fn occs(&self, n: u8) -> OCCS_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OCCS_R::new(((self.bits >> (n * 8 + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "ADC(1-3) ordinary channel conversion start flag"]
    #[inline(always)]
    pub fn occs_iter(&self) -> impl Iterator<Item = OCCS_R> + '_ {
        (0..3).map(move |n| OCCS_R::new(((self.bits >> (n * 8 + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - ADC1 ordinary channel conversion start flag"]
    #[inline(always)]
    pub fn occs1(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - ADC2 ordinary channel conversion start flag"]
    #[inline(always)]
    pub fn occs2(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC3 ordinary channel conversion start flag"]
    #[inline(always)]
    pub fn occs3(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "ADC(1-3) overrun flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OCCO1` field.</div>"]
    #[inline(always)]
    pub fn occo(&self, n: u8) -> OCCO_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OCCO_R::new(((self.bits >> (n * 8 + 5)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "ADC(1-3) overrun flag"]
    #[inline(always)]
    pub fn occo_iter(&self) -> impl Iterator<Item = OCCO_R> + '_ {
        (0..3).map(move |n| OCCO_R::new(((self.bits >> (n * 8 + 5)) & 1) != 0))
    }
    #[doc = "Bit 5 - ADC1 overrun flag"]
    #[inline(always)]
    pub fn occo1(&self) -> OCCO_R {
        OCCO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC2 overrun flag"]
    #[inline(always)]
    pub fn occo2(&self) -> OCCO_R {
        OCCO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 21 - ADC3 overrun flag"]
    #[inline(always)]
    pub fn occo3(&self) -> OCCO_R {
        OCCO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "ADC(1-3) conversion ready flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `RDY1` field.</div>"]
    #[inline(always)]
    pub fn rdy(&self, n: u8) -> RDY_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        RDY_R::new(((self.bits >> (n * 8 + 6)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "ADC(1-3) conversion ready flag"]
    #[inline(always)]
    pub fn rdy_iter(&self) -> impl Iterator<Item = RDY_R> + '_ {
        (0..3).map(move |n| RDY_R::new(((self.bits >> (n * 8 + 6)) & 1) != 0))
    }
    #[doc = "Bit 6 - ADC1 conversion ready flag"]
    #[inline(always)]
    pub fn rdy1(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC2 conversion ready flag"]
    #[inline(always)]
    pub fn rdy2(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC3 conversion ready flag"]
    #[inline(always)]
    pub fn rdy3(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSTS")
            .field("rdy1", &self.rdy1())
            .field("rdy2", &self.rdy2())
            .field("rdy3", &self.rdy3())
            .field("occo1", &self.occo1())
            .field("occo2", &self.occo2())
            .field("occo3", &self.occo3())
            .field("occs1", &self.occs1())
            .field("occs2", &self.occs2())
            .field("occs3", &self.occs3())
            .field("pccs1", &self.pccs1())
            .field("pccs2", &self.pccs2())
            .field("pccs3", &self.pccs3())
            .field("pcce1", &self.pcce1())
            .field("pcce2", &self.pcce2())
            .field("pcce3", &self.pcce3())
            .field("occe1", &self.occe1())
            .field("occe2", &self.occe2())
            .field("occe3", &self.occe3())
            .field("vmor1", &self.vmor1())
            .field("vmor2", &self.vmor2())
            .field("vmor3", &self.vmor3())
            .finish()
    }
}
#[doc = "Common status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSTS_SPEC;
impl crate::RegisterSpec for CSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csts::R`](R) reader structure"]
impl crate::Readable for CSTS_SPEC {}
#[doc = "`reset()` method sets CSTS to value 0"]
impl crate::Resettable for CSTS_SPEC {}
