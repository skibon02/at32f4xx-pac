#[doc = "Register `AHBRST3` reader"]
pub type R = crate::R<AHBRST3_SPEC>;
#[doc = "Register `AHBRST3` writer"]
pub type W = crate::W<AHBRST3_SPEC>;
#[doc = "QSPI1 reset\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QSPI1W_A {
    #[doc = "1: Reset peripheral"]
    Reset = 1,
}
impl From<QSPI1W_A> for bool {
    #[inline(always)]
    fn from(variant: QSPI1W_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QSPI1` reader - QSPI1 reset"]
pub type QSPI1_R = crate::BitReader<QSPI1W_A>;
impl QSPI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<QSPI1W_A> {
        match self.bits {
            true => Some(QSPI1W_A::Reset),
            _ => None,
        }
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == QSPI1W_A::Reset
    }
}
#[doc = "Field `QSPI1` writer - QSPI1 reset"]
pub type QSPI1_W<'a, REG> = crate::BitWriter1S<'a, REG, QSPI1W_A>;
impl<'a, REG> QSPI1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(QSPI1W_A::Reset)
    }
}
impl R {
    #[doc = "Bit 1 - QSPI1 reset"]
    #[inline(always)]
    pub fn qspi1(&self) -> QSPI1_R {
        QSPI1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST3")
            .field("qspi1", &self.qspi1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - QSPI1 reset"]
    #[inline(always)]
    pub fn qspi1(&mut self) -> QSPI1_W<'_, AHBRST3_SPEC> {
        QSPI1_W::new(self, 1)
    }
}
#[doc = "AHB peripheral reset register 3 (CRM_AHBRST3)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrst3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrst3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST3_SPEC;
impl crate::RegisterSpec for AHBRST3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst3::R`](R) reader structure"]
impl crate::Readable for AHBRST3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst3::W`](W) writer structure"]
impl crate::Writable for AHBRST3_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x02;
}
#[doc = "`reset()` method sets AHBRST3 to value 0"]
impl crate::Resettable for AHBRST3_SPEC {}
