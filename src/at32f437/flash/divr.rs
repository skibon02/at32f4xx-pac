#[doc = "Register `DIVR` reader"]
pub type R = crate::R<DIVR_SPEC>;
#[doc = "Register `DIVR` writer"]
pub type W = crate::W<DIVR_SPEC>;
#[doc = "Flash divider\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLASH_DIV_A {
    #[doc = "0: Flash clock frequency = HCLK/2"]
    Div2 = 0,
    #[doc = "1: Flash clock frequency = HCLK/3"]
    Div3 = 1,
    #[doc = "2: Flash clock frequency = HCLK/4"]
    Div4 = 2,
}
impl From<FLASH_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASH_DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FLASH_DIV_A {
    type Ux = u8;
}
impl crate::IsEnum for FLASH_DIV_A {}
#[doc = "Field `FDIV` reader - Flash divider"]
pub type FDIV_R = crate::FieldReader<FLASH_DIV_A>;
impl FDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLASH_DIV_A {
        match self.bits {
            0 => FLASH_DIV_A::Div2,
            1 => FLASH_DIV_A::Div3,
            _ => FLASH_DIV_A::Div4,
        }
    }
    #[doc = "Flash clock frequency = HCLK/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == FLASH_DIV_A::Div2
    }
    #[doc = "Flash clock frequency = HCLK/3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == FLASH_DIV_A::Div3
    }
    #[doc = "Flash clock frequency = HCLK/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        matches!(self.variant(), FLASH_DIV_A::Div4)
    }
}
#[doc = "Field `FDIV` writer - Flash divider"]
pub type FDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, FLASH_DIV_A, crate::Safe>;
impl<'a, REG> FDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash clock frequency = HCLK/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_DIV_A::Div2)
    }
    #[doc = "Flash clock frequency = HCLK/3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_DIV_A::Div3)
    }
    #[doc = "Flash clock frequency = HCLK/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(FLASH_DIV_A::Div4)
    }
}
#[doc = "Field `FDIV_STS` reader - Flash divider status"]
pub use FDIV_R as FDIV_STS_R;
impl R {
    #[doc = "Bits 0:1 - Flash divider"]
    #[inline(always)]
    pub fn fdiv(&self) -> FDIV_R {
        FDIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Flash divider status"]
    #[inline(always)]
    pub fn fdiv_sts(&self) -> FDIV_STS_R {
        FDIV_STS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIVR")
            .field("fdiv", &self.fdiv())
            .field("fdiv_sts", &self.fdiv_sts())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Flash divider"]
    #[inline(always)]
    pub fn fdiv(&mut self) -> FDIV_W<'_, DIVR_SPEC> {
        FDIV_W::new(self, 0)
    }
}
#[doc = "Flash divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIVR_SPEC;
impl crate::RegisterSpec for DIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divr::R`](R) reader structure"]
impl crate::Readable for DIVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`divr::W`](W) writer structure"]
impl crate::Writable for DIVR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIVR to value 0x22"]
impl crate::Resettable for DIVR_SPEC {
    const RESET_VALUE: u32 = 0x22;
}
