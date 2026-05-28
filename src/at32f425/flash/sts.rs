#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Operate busy flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OBF_A {
    #[doc = "0: Flash is idle"]
    Idle = 0,
    #[doc = "1: Flash operation is in progress"]
    Busy = 1,
}
impl From<OBF_A> for bool {
    #[inline(always)]
    fn from(variant: OBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OBF` reader - Operate busy flag"]
pub type OBF_R = crate::BitReader<OBF_A>;
impl OBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OBF_A {
        match self.bits {
            false => OBF_A::Idle,
            true => OBF_A::Busy,
        }
    }
    #[doc = "Flash is idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == OBF_A::Idle
    }
    #[doc = "Flash operation is in progress"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == OBF_A::Busy
    }
}
#[doc = "program error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGMERR_A {
    #[doc = "0: No program error"]
    NoError = 0,
    #[doc = "1: Program error occurred"]
    Error = 1,
}
impl From<PRGMERR_A> for bool {
    #[inline(always)]
    fn from(variant: PRGMERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGMERR` reader - program error"]
pub type PRGMERR_R = crate::BitReader<PRGMERR_A>;
impl PRGMERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRGMERR_A {
        match self.bits {
            false => PRGMERR_A::NoError,
            true => PRGMERR_A::Error,
        }
    }
    #[doc = "No program error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PRGMERR_A::NoError
    }
    #[doc = "Program error occurred"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PRGMERR_A::Error
    }
}
#[doc = "Field `PRGMERR` writer - program error"]
pub type PRGMERR_W<'a, REG> = crate::BitWriter1C<'a, REG, PRGMERR_A>;
impl<'a, REG> PRGMERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No program error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(PRGMERR_A::NoError)
    }
    #[doc = "Program error occurred"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(PRGMERR_A::Error)
    }
}
#[doc = "Erase/program protection error\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPPERR_A {
    #[doc = "0: No erase/program error"]
    NoError = 0,
    #[doc = "1: Erase/program error occurred"]
    Error = 1,
}
impl From<EPPERR_A> for bool {
    #[inline(always)]
    fn from(variant: EPPERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPPERR` reader - Erase/program protection error"]
pub type EPPERR_R = crate::BitReader<EPPERR_A>;
impl EPPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPPERR_A {
        match self.bits {
            false => EPPERR_A::NoError,
            true => EPPERR_A::Error,
        }
    }
    #[doc = "No erase/program error"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == EPPERR_A::NoError
    }
    #[doc = "Erase/program error occurred"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == EPPERR_A::Error
    }
}
#[doc = "Field `EPPERR` writer - Erase/program protection error"]
pub type EPPERR_W<'a, REG> = crate::BitWriter1C<'a, REG, EPPERR_A>;
impl<'a, REG> EPPERR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No erase/program error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(EPPERR_A::NoError)
    }
    #[doc = "Erase/program error occurred"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(EPPERR_A::Error)
    }
}
#[doc = "Operate done flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODF_A {
    #[doc = "0: Flash operation done flag not set"]
    NotSet = 0,
    #[doc = "1: Flash/erase operation done"]
    Done = 1,
}
impl From<ODF_A> for bool {
    #[inline(always)]
    fn from(variant: ODF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODF` reader - Operate done flag"]
pub type ODF_R = crate::BitReader<ODF_A>;
impl ODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ODF_A {
        match self.bits {
            false => ODF_A::NotSet,
            true => ODF_A::Done,
        }
    }
    #[doc = "Flash operation done flag not set"]
    #[inline(always)]
    pub fn is_not_set(&self) -> bool {
        *self == ODF_A::NotSet
    }
    #[doc = "Flash/erase operation done"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == ODF_A::Done
    }
}
#[doc = "Field `ODF` writer - Operate done flag"]
pub type ODF_W<'a, REG> = crate::BitWriter1C<'a, REG, ODF_A>;
impl<'a, REG> ODF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash operation done flag not set"]
    #[inline(always)]
    pub fn not_set(self) -> &'a mut crate::W<REG> {
        self.variant(ODF_A::NotSet)
    }
    #[doc = "Flash/erase operation done"]
    #[inline(always)]
    pub fn done(self) -> &'a mut crate::W<REG> {
        self.variant(ODF_A::Done)
    }
}
impl R {
    #[doc = "Bit 0 - Operate busy flag"]
    #[inline(always)]
    pub fn obf(&self) -> OBF_R {
        OBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - program error"]
    #[inline(always)]
    pub fn prgmerr(&self) -> PRGMERR_R {
        PRGMERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase/program protection error"]
    #[inline(always)]
    pub fn epperr(&self) -> EPPERR_R {
        EPPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Operate done flag"]
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("odf", &self.odf())
            .field("epperr", &self.epperr())
            .field("prgmerr", &self.prgmerr())
            .field("obf", &self.obf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - program error"]
    #[inline(always)]
    pub fn prgmerr(&mut self) -> PRGMERR_W<'_, STS_SPEC> {
        PRGMERR_W::new(self, 2)
    }
    #[doc = "Bit 4 - Erase/program protection error"]
    #[inline(always)]
    pub fn epperr(&mut self) -> EPPERR_W<'_, STS_SPEC> {
        EPPERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Operate done flag"]
    #[inline(always)]
    pub fn odf(&mut self) -> ODF_W<'_, STS_SPEC> {
        ODF_W::new(self, 5)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x34;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {}
