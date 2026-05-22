#[doc = "Register `CFG2` reader"]
pub type R = crate::R<CFG2_SPEC>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<CFG2_SPEC>;
#[doc = "MII or RMII selection bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MII_RMII_SEL_A {
    #[doc = "0: MII interface selected"]
    Mii = 0,
    #[doc = "1: RMII interface selected"]
    Rmii = 1,
}
impl From<MII_RMII_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: MII_RMII_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MII_RMII_SEL` reader - MII or RMII selection bits"]
pub type MII_RMII_SEL_R = crate::BitReader<MII_RMII_SEL_A>;
impl MII_RMII_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MII_RMII_SEL_A {
        match self.bits {
            false => MII_RMII_SEL_A::Mii,
            true => MII_RMII_SEL_A::Rmii,
        }
    }
    #[doc = "MII interface selected"]
    #[inline(always)]
    pub fn is_mii(&self) -> bool {
        *self == MII_RMII_SEL_A::Mii
    }
    #[doc = "RMII interface selected"]
    #[inline(always)]
    pub fn is_rmii(&self) -> bool {
        *self == MII_RMII_SEL_A::Rmii
    }
}
#[doc = "Field `MII_RMII_SEL` writer - MII or RMII selection bits"]
pub type MII_RMII_SEL_W<'a, REG> = crate::BitWriter<'a, REG, MII_RMII_SEL_A>;
impl<'a, REG> MII_RMII_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MII interface selected"]
    #[inline(always)]
    pub fn mii(self) -> &'a mut crate::W<REG> {
        self.variant(MII_RMII_SEL_A::Mii)
    }
    #[doc = "RMII interface selected"]
    #[inline(always)]
    pub fn rmii(self) -> &'a mut crate::W<REG> {
        self.variant(MII_RMII_SEL_A::Rmii)
    }
}
impl R {
    #[doc = "Bit 23 - MII or RMII selection bits"]
    #[inline(always)]
    pub fn mii_rmii_sel(&self) -> MII_RMII_SEL_R {
        MII_RMII_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG2")
            .field("mii_rmii_sel", &self.mii_rmii_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 23 - MII or RMII selection bits"]
    #[inline(always)]
    pub fn mii_rmii_sel(&mut self) -> MII_RMII_SEL_W<'_, CFG2_SPEC> {
        MII_RMII_SEL_W::new(self, 23)
    }
}
#[doc = "configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {}
