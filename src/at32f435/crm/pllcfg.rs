#[doc = "Register `PLLCFG` reader"]
pub type R = crate::R<PLLCFG_SPEC>;
#[doc = "Register `PLLCFG` writer"]
pub type W = crate::W<PLLCFG_SPEC>;
#[doc = "Field `MS` reader - PLL pre-division (1-15)"]
pub type MS_R = crate::FieldReader;
#[doc = "Field `MS` writer - PLL pre-division (1-15)"]
pub type MS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NS` reader - PLL multiplication factor (31-500)"]
pub type NS_R = crate::FieldReader<u16>;
#[doc = "Field `NS` writer - PLL multiplication factor (31-500)"]
pub type NS_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "PLL post-division\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POST_DIVISION_A {
    #[doc = "2: Division by 4"]
    Div4 = 2,
    #[doc = "3: Division by 8"]
    Div8 = 3,
    #[doc = "4: Division by 16"]
    Div16 = 4,
    #[doc = "5: Division by 32"]
    Div32 = 5,
}
impl From<POST_DIVISION_A> for u8 {
    #[inline(always)]
    fn from(variant: POST_DIVISION_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POST_DIVISION_A {
    type Ux = u8;
}
impl crate::IsEnum for POST_DIVISION_A {}
#[doc = "Field `FR` reader - PLL post-division"]
pub type FR_R = crate::FieldReader<POST_DIVISION_A>;
impl FR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<POST_DIVISION_A> {
        match self.bits {
            2 => Some(POST_DIVISION_A::Div4),
            3 => Some(POST_DIVISION_A::Div8),
            4 => Some(POST_DIVISION_A::Div16),
            5 => Some(POST_DIVISION_A::Div32),
            _ => None,
        }
    }
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == POST_DIVISION_A::Div4
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == POST_DIVISION_A::Div8
    }
    #[doc = "Division by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == POST_DIVISION_A::Div16
    }
    #[doc = "Division by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == POST_DIVISION_A::Div32
    }
}
#[doc = "Field `FR` writer - PLL post-division"]
pub type FR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, POST_DIVISION_A>;
impl<'a, REG> FR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Division by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(POST_DIVISION_A::Div4)
    }
    #[doc = "Division by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(POST_DIVISION_A::Div8)
    }
    #[doc = "Division by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(POST_DIVISION_A::Div16)
    }
    #[doc = "Division by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(POST_DIVISION_A::Div32)
    }
}
#[doc = "PLL reference clock select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_REFERENCE_CLOCK_SELECT_A {
    #[doc = "0: HICK-divided clock"]
    Hick = 0,
    #[doc = "1: HEXT clock"]
    Hext = 1,
}
impl From<PLL_REFERENCE_CLOCK_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_REFERENCE_CLOCK_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRCS` reader - PLL reference clock select"]
pub type PLLRCS_R = crate::BitReader<PLL_REFERENCE_CLOCK_SELECT_A>;
impl PLLRCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLL_REFERENCE_CLOCK_SELECT_A {
        match self.bits {
            false => PLL_REFERENCE_CLOCK_SELECT_A::Hick,
            true => PLL_REFERENCE_CLOCK_SELECT_A::Hext,
        }
    }
    #[doc = "HICK-divided clock"]
    #[inline(always)]
    pub fn is_hick(&self) -> bool {
        *self == PLL_REFERENCE_CLOCK_SELECT_A::Hick
    }
    #[doc = "HEXT clock"]
    #[inline(always)]
    pub fn is_hext(&self) -> bool {
        *self == PLL_REFERENCE_CLOCK_SELECT_A::Hext
    }
}
#[doc = "Field `PLLRCS` writer - PLL reference clock select"]
pub type PLLRCS_W<'a, REG> = crate::BitWriter<'a, REG, PLL_REFERENCE_CLOCK_SELECT_A>;
impl<'a, REG> PLLRCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HICK-divided clock"]
    #[inline(always)]
    pub fn hick(self) -> &'a mut crate::W<REG> {
        self.variant(PLL_REFERENCE_CLOCK_SELECT_A::Hick)
    }
    #[doc = "HEXT clock"]
    #[inline(always)]
    pub fn hext(self) -> &'a mut crate::W<REG> {
        self.variant(PLL_REFERENCE_CLOCK_SELECT_A::Hext)
    }
}
impl R {
    #[doc = "Bits 0:3 - PLL pre-division (1-15)"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:14 - PLL multiplication factor (31-500)"]
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:18 - PLL post-division"]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 22 - PLL reference clock select"]
    #[inline(always)]
    pub fn pllrcs(&self) -> PLLRCS_R {
        PLLRCS_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCFG")
            .field("ms", &self.ms())
            .field("ns", &self.ns())
            .field("fr", &self.fr())
            .field("pllrcs", &self.pllrcs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - PLL pre-division (1-15)"]
    #[inline(always)]
    pub fn ms(&mut self) -> MS_W<'_, PLLCFG_SPEC> {
        MS_W::new(self, 0)
    }
    #[doc = "Bits 6:14 - PLL multiplication factor (31-500)"]
    #[inline(always)]
    pub fn ns(&mut self) -> NS_W<'_, PLLCFG_SPEC> {
        NS_W::new(self, 6)
    }
    #[doc = "Bits 16:18 - PLL post-division"]
    #[inline(always)]
    pub fn fr(&mut self) -> FR_W<'_, PLLCFG_SPEC> {
        FR_W::new(self, 16)
    }
    #[doc = "Bit 22 - PLL reference clock select"]
    #[inline(always)]
    pub fn pllrcs(&mut self) -> PLLRCS_W<'_, PLLCFG_SPEC> {
        PLLRCS_W::new(self, 22)
    }
}
#[doc = "PLL configuration register (CRM_PLLCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLCFG_SPEC;
impl crate::RegisterSpec for PLLCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfg::R`](R) reader structure"]
impl crate::Readable for PLLCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllcfg::W`](W) writer structure"]
impl crate::Writable for PLLCFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLCFG to value 0x0003_3002"]
impl crate::Resettable for PLLCFG_SPEC {
    const RESET_VALUE: u32 = 0x0003_3002;
}
