#[doc = "Register `PSR` reader"]
pub type R = crate::R<PSR_SPEC>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PSR_SPEC>;
#[doc = "Flash non-zero wait area boost. Flash read access is faster for NZW flash range, frequency is limited: 108/160/192MHZ at 1.1/1.2/1.3 LDO voltage\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NZW_BST_A {
    #[doc = "0: NZW_BST disabled"]
    Disabled = 0,
    #[doc = "1: NZW_BST enabled."]
    Enabled = 1,
}
impl From<NZW_BST_A> for bool {
    #[inline(always)]
    fn from(variant: NZW_BST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NZW_BST` reader - Flash non-zero wait area boost. Flash read access is faster for NZW flash range, frequency is limited: 108/160/192MHZ at 1.1/1.2/1.3 LDO voltage"]
pub type NZW_BST_R = crate::BitReader<NZW_BST_A>;
impl NZW_BST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NZW_BST_A {
        match self.bits {
            false => NZW_BST_A::Disabled,
            true => NZW_BST_A::Enabled,
        }
    }
    #[doc = "NZW_BST disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NZW_BST_A::Disabled
    }
    #[doc = "NZW_BST enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NZW_BST_A::Enabled
    }
}
#[doc = "Field `NZW_BST` writer - Flash non-zero wait area boost. Flash read access is faster for NZW flash range, frequency is limited: 108/160/192MHZ at 1.1/1.2/1.3 LDO voltage"]
pub type NZW_BST_W<'a, REG> = crate::BitWriter<'a, REG, NZW_BST_A>;
impl<'a, REG> NZW_BST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NZW_BST disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NZW_BST_A::Disabled)
    }
    #[doc = "NZW_BST enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NZW_BST_A::Enabled)
    }
}
#[doc = "Flash non-zero wait area boost status\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NZW_BST_STS_A {
    #[doc = "0: NZW_BST is disabled"]
    Disabled = 0,
    #[doc = "1: NZW_BST is enabled."]
    Enabled = 1,
}
impl From<NZW_BST_STS_A> for bool {
    #[inline(always)]
    fn from(variant: NZW_BST_STS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NZW_BST_STS` reader - Flash non-zero wait area boost status"]
pub type NZW_BST_STS_R = crate::BitReader<NZW_BST_STS_A>;
impl NZW_BST_STS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NZW_BST_STS_A {
        match self.bits {
            false => NZW_BST_STS_A::Disabled,
            true => NZW_BST_STS_A::Enabled,
        }
    }
    #[doc = "NZW_BST is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NZW_BST_STS_A::Disabled
    }
    #[doc = "NZW_BST is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NZW_BST_STS_A::Enabled
    }
}
impl R {
    #[doc = "Bit 12 - Flash non-zero wait area boost. Flash read access is faster for NZW flash range, frequency is limited: 108/160/192MHZ at 1.1/1.2/1.3 LDO voltage"]
    #[inline(always)]
    pub fn nzw_bst(&self) -> NZW_BST_R {
        NZW_BST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flash non-zero wait area boost status"]
    #[inline(always)]
    pub fn nzw_bst_sts(&self) -> NZW_BST_STS_R {
        NZW_BST_STS_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("nzw_bst_sts", &self.nzw_bst_sts())
            .field("nzw_bst", &self.nzw_bst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - Flash non-zero wait area boost. Flash read access is faster for NZW flash range, frequency is limited: 108/160/192MHZ at 1.1/1.2/1.3 LDO voltage"]
    #[inline(always)]
    pub fn nzw_bst(&mut self) -> NZW_BST_W<'_, PSR_SPEC> {
        NZW_BST_W::new(self, 12)
    }
}
#[doc = "Performance selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PSR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSR to value 0x0330"]
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: u32 = 0x0330;
}
