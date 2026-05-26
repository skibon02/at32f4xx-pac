#[doc = "Register `CCAL` reader"]
pub type R = crate::R<CCAL_SPEC>;
#[doc = "Register `CCAL` writer"]
pub type W = crate::W<CCAL_SPEC>;
#[doc = "Field `CALVAL` reader - Calibration value"]
pub type CALVAL_R = crate::FieldReader;
#[doc = "Field `CALVAL` writer - Calibration value"]
pub type CALVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Calibration direction\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALDIR_A {
    #[doc = "0: Positive calibration (+0, +4, +8...)"]
    Pos = 0,
    #[doc = "1: Negative calibration (-0, -2, -4...)"]
    Neg = 1,
}
impl From<CALDIR_A> for bool {
    #[inline(always)]
    fn from(variant: CALDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALDIR` reader - Calibration direction"]
pub type CALDIR_R = crate::BitReader<CALDIR_A>;
impl CALDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALDIR_A {
        match self.bits {
            false => CALDIR_A::Pos,
            true => CALDIR_A::Neg,
        }
    }
    #[doc = "Positive calibration (+0, +4, +8...)"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == CALDIR_A::Pos
    }
    #[doc = "Negative calibration (-0, -2, -4...)"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == CALDIR_A::Neg
    }
}
#[doc = "Field `CALDIR` writer - Calibration direction"]
pub type CALDIR_W<'a, REG> = crate::BitWriter<'a, REG, CALDIR_A>;
impl<'a, REG> CALDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Positive calibration (+0, +4, +8...)"]
    #[inline(always)]
    pub fn pos(self) -> &'a mut crate::W<REG> {
        self.variant(CALDIR_A::Pos)
    }
    #[doc = "Negative calibration (-0, -2, -4...)"]
    #[inline(always)]
    pub fn neg(self) -> &'a mut crate::W<REG> {
        self.variant(CALDIR_A::Neg)
    }
}
impl R {
    #[doc = "Bits 0:4 - Calibration value"]
    #[inline(always)]
    pub fn calval(&self) -> CALVAL_R {
        CALVAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&self) -> CALDIR_R {
        CALDIR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCAL")
            .field("caldir", &self.caldir())
            .field("calval", &self.calval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Calibration value"]
    #[inline(always)]
    pub fn calval(&mut self) -> CALVAL_W<'_, CCAL_SPEC> {
        CALVAL_W::new(self, 0)
    }
    #[doc = "Bit 7 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&mut self) -> CALDIR_W<'_, CCAL_SPEC> {
        CALDIR_W::new(self, 7)
    }
}
#[doc = "Calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCAL_SPEC;
impl crate::RegisterSpec for CCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccal::R`](R) reader structure"]
impl crate::Readable for CCAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccal::W`](W) writer structure"]
impl crate::Writable for CCAL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCAL to value 0"]
impl crate::Resettable for CCAL_SPEC {}
