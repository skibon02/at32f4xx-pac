#[doc = "Register `RMP` reader"]
pub type R = crate::R<RMP_SPEC>;
#[doc = "Register `RMP` writer"]
pub type W = crate::W<RMP_SPEC>;
#[doc = "TMR2 IS1 input remap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR2_IS1_IRMP_A {
    #[doc = "0: TMR8_TRGO output"]
    Tmr8trgOut = 0,
    #[doc = "1: Ethernet PTP event"]
    EthPtp = 1,
    #[doc = "2: OTG1_FS SOF signal"]
    Otg1fsSof = 2,
    #[doc = "3: OTG2_FS SOF signal"]
    Otg2fsSof = 3,
}
impl From<TMR2_IS1_IRMP_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR2_IS1_IRMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR2_IS1_IRMP_A {
    type Ux = u8;
}
impl crate::IsEnum for TMR2_IS1_IRMP_A {}
#[doc = "Field `TMR2_IS1_IRMP` reader - TMR2 IS1 input remap"]
pub type TMR2_IS1_IRMP_R = crate::FieldReader<TMR2_IS1_IRMP_A>;
impl TMR2_IS1_IRMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMR2_IS1_IRMP_A {
        match self.bits {
            0 => TMR2_IS1_IRMP_A::Tmr8trgOut,
            1 => TMR2_IS1_IRMP_A::EthPtp,
            2 => TMR2_IS1_IRMP_A::Otg1fsSof,
            3 => TMR2_IS1_IRMP_A::Otg2fsSof,
            _ => unreachable!(),
        }
    }
    #[doc = "TMR8_TRGO output"]
    #[inline(always)]
    pub fn is_tmr8trg_out(&self) -> bool {
        *self == TMR2_IS1_IRMP_A::Tmr8trgOut
    }
    #[doc = "Ethernet PTP event"]
    #[inline(always)]
    pub fn is_eth_ptp(&self) -> bool {
        *self == TMR2_IS1_IRMP_A::EthPtp
    }
    #[doc = "OTG1_FS SOF signal"]
    #[inline(always)]
    pub fn is_otg1fs_sof(&self) -> bool {
        *self == TMR2_IS1_IRMP_A::Otg1fsSof
    }
    #[doc = "OTG2_FS SOF signal"]
    #[inline(always)]
    pub fn is_otg2fs_sof(&self) -> bool {
        *self == TMR2_IS1_IRMP_A::Otg2fsSof
    }
}
#[doc = "Field `TMR2_IS1_IRMP` writer - TMR2 IS1 input remap"]
pub type TMR2_IS1_IRMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TMR2_IS1_IRMP_A, crate::Safe>;
impl<'a, REG> TMR2_IS1_IRMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TMR8_TRGO output"]
    #[inline(always)]
    pub fn tmr8trg_out(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_IS1_IRMP_A::Tmr8trgOut)
    }
    #[doc = "Ethernet PTP event"]
    #[inline(always)]
    pub fn eth_ptp(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_IS1_IRMP_A::EthPtp)
    }
    #[doc = "OTG1_FS SOF signal"]
    #[inline(always)]
    pub fn otg1fs_sof(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_IS1_IRMP_A::Otg1fsSof)
    }
    #[doc = "OTG2_FS SOF signal"]
    #[inline(always)]
    pub fn otg2fs_sof(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_IS1_IRMP_A::Otg2fsSof)
    }
}
impl R {
    #[doc = "Bits 10:11 - TMR2 IS1 input remap"]
    #[inline(always)]
    pub fn tmr2_is1_irmp(&self) -> TMR2_IS1_IRMP_R {
        TMR2_IS1_IRMP_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMP")
            .field("tmr2_is1_irmp", &self.tmr2_is1_irmp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 10:11 - TMR2 IS1 input remap"]
    #[inline(always)]
    pub fn tmr2_is1_irmp(&mut self) -> TMR2_IS1_IRMP_W<'_, RMP_SPEC> {
        TMR2_IS1_IRMP_W::new(self, 10)
    }
}
#[doc = "TMR2 remap\n\nYou can [`read`](crate::Reg::read) this register and get [`rmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMP_SPEC;
impl crate::RegisterSpec for RMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmp::R`](R) reader structure"]
impl crate::Readable for RMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmp::W`](W) writer structure"]
impl crate::Writable for RMP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RMP to value 0"]
impl crate::Resettable for RMP_SPEC {}
