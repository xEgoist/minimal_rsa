use ibig::ubig;
use rand::{thread_rng, Rng};
use lazy_static::lazy_static;
#[derive(PartialEq, Eq, Debug)]
pub enum IsPrime {
    Probably,
    NotPrime,
}
lazy_static!{
static ref LOWER_PRIMES: [ibig::UBig;668] = [
ibig::UBig::from(3_usize), ibig::UBig::from(5_usize), ibig::UBig::from(7_usize), ibig::UBig::from(11_usize), ibig::UBig::from(13_usize), ibig::UBig::from(17_usize), ibig::UBig::from(19_usize), ibig::UBig::from(23_usize), ibig::UBig::from(29_usize), ibig::UBig::from(31_usize), ibig::UBig::from(37_usize), ibig::UBig::from(41_usize), ibig::UBig::from(43_usize), ibig::UBig::from(47_usize), ibig::UBig::from(53_usize), ibig::UBig::from(59_usize), ibig::UBig::from(61_usize), ibig::UBig::from(67_usize), ibig::UBig::from(71_usize), ibig::UBig::from(73_usize), ibig::UBig::from(79_usize), ibig::UBig::from(83_usize), ibig::UBig::from(89_usize), ibig::UBig::from(97_usize), ibig::UBig::from(101_usize), ibig::UBig::from(103_usize), ibig::UBig::from(107_usize), ibig::UBig::from(109_usize), ibig::UBig::from(113_usize), ibig::UBig::from(127_usize), ibig::UBig::from(131_usize), ibig::UBig::from(137_usize), ibig::UBig::from(139_usize), ibig::UBig::from(149_usize), ibig::UBig::from(151_usize), ibig::UBig::from(157_usize), ibig::UBig::from(163_usize), ibig::UBig::from(167_usize), ibig::UBig::from(173_usize), ibig::UBig::from(179_usize), ibig::UBig::from(181_usize), ibig::UBig::from(191_usize), ibig::UBig::from(193_usize), ibig::UBig::from(197_usize), ibig::UBig::from(199_usize), ibig::UBig::from(211_usize), ibig::UBig::from(223_usize), ibig::UBig::from(227_usize), ibig::UBig::from(229_usize), ibig::UBig::from(233_usize), ibig::UBig::from(239_usize), ibig::UBig::from(241_usize), ibig::UBig::from(251_usize), ibig::UBig::from(257_usize), ibig::UBig::from(263_usize), ibig::UBig::from(269_usize), ibig::UBig::from(271_usize), ibig::UBig::from(277_usize), ibig::UBig::from(281_usize), ibig::UBig::from(283_usize), ibig::UBig::from(293_usize), ibig::UBig::from(307_usize), ibig::UBig::from(311_usize), ibig::UBig::from(313_usize), ibig::UBig::from(317_usize), ibig::UBig::from(331_usize), ibig::UBig::from(337_usize), ibig::UBig::from(347_usize), ibig::UBig::from(349_usize), ibig::UBig::from(353_usize), ibig::UBig::from(359_usize), ibig::UBig::from(367_usize), ibig::UBig::from(373_usize), ibig::UBig::from(379_usize), ibig::UBig::from(383_usize), ibig::UBig::from(389_usize), ibig::UBig::from(397_usize), ibig::UBig::from(401_usize), ibig::UBig::from(409_usize), ibig::UBig::from(419_usize), ibig::UBig::from(421_usize), ibig::UBig::from(431_usize), ibig::UBig::from(433_usize), ibig::UBig::from(439_usize), ibig::UBig::from(443_usize), ibig::UBig::from(449_usize), ibig::UBig::from(457_usize), ibig::UBig::from(461_usize), ibig::UBig::from(463_usize), ibig::UBig::from(467_usize), ibig::UBig::from(479_usize), ibig::UBig::from(487_usize), ibig::UBig::from(491_usize), ibig::UBig::from(499_usize), ibig::UBig::from(503_usize), ibig::UBig::from(509_usize), ibig::UBig::from(521_usize), ibig::UBig::from(523_usize), ibig::UBig::from(541_usize), ibig::UBig::from(547_usize), ibig::UBig::from(557_usize), ibig::UBig::from(563_usize), ibig::UBig::from(569_usize), ibig::UBig::from(571_usize), ibig::UBig::from(577_usize), ibig::UBig::from(587_usize), ibig::UBig::from(593_usize), ibig::UBig::from(599_usize), ibig::UBig::from(601_usize), ibig::UBig::from(607_usize), ibig::UBig::from(613_usize), ibig::UBig::from(617_usize), ibig::UBig::from(619_usize), ibig::UBig::from(631_usize), ibig::UBig::from(641_usize), ibig::UBig::from(643_usize), ibig::UBig::from(647_usize), ibig::UBig::from(653_usize), ibig::UBig::from(659_usize), ibig::UBig::from(661_usize), ibig::UBig::from(673_usize), ibig::UBig::from(677_usize), ibig::UBig::from(683_usize), ibig::UBig::from(691_usize), ibig::UBig::from(701_usize), ibig::UBig::from(709_usize), ibig::UBig::from(719_usize), ibig::UBig::from(727_usize), ibig::UBig::from(733_usize), ibig::UBig::from(739_usize), ibig::UBig::from(743_usize), ibig::UBig::from(751_usize), ibig::UBig::from(757_usize), ibig::UBig::from(761_usize), ibig::UBig::from(769_usize), ibig::UBig::from(773_usize), ibig::UBig::from(787_usize), ibig::UBig::from(797_usize), ibig::UBig::from(809_usize), ibig::UBig::from(811_usize), ibig::UBig::from(821_usize), ibig::UBig::from(823_usize), ibig::UBig::from(827_usize), ibig::UBig::from(829_usize), ibig::UBig::from(839_usize), ibig::UBig::from(853_usize), ibig::UBig::from(857_usize), ibig::UBig::from(859_usize), ibig::UBig::from(863_usize), ibig::UBig::from(877_usize), ibig::UBig::from(881_usize), ibig::UBig::from(883_usize), ibig::UBig::from(887_usize), ibig::UBig::from(907_usize), ibig::UBig::from(911_usize), ibig::UBig::from(919_usize), ibig::UBig::from(929_usize), ibig::UBig::from(937_usize), ibig::UBig::from(941_usize), ibig::UBig::from(947_usize), ibig::UBig::from(953_usize), ibig::UBig::from(967_usize), ibig::UBig::from(971_usize), ibig::UBig::from(977_usize), ibig::UBig::from(983_usize), ibig::UBig::from(991_usize), ibig::UBig::from(997_usize), ibig::UBig::from(1009_usize), ibig::UBig::from(1013_usize), ibig::UBig::from(1019_usize), ibig::UBig::from(1021_usize), ibig::UBig::from(1031_usize), ibig::UBig::from(1033_usize), ibig::UBig::from(1039_usize), ibig::UBig::from(1049_usize), ibig::UBig::from(1051_usize), ibig::UBig::from(1061_usize), ibig::UBig::from(1063_usize), ibig::UBig::from(1069_usize), ibig::UBig::from(1087_usize), ibig::UBig::from(1091_usize), ibig::UBig::from(1093_usize), ibig::UBig::from(1097_usize), ibig::UBig::from(1103_usize), ibig::UBig::from(1109_usize), ibig::UBig::from(1117_usize), ibig::UBig::from(1123_usize), ibig::UBig::from(1129_usize), ibig::UBig::from(1151_usize), ibig::UBig::from(1153_usize), ibig::UBig::from(1163_usize), ibig::UBig::from(1171_usize), ibig::UBig::from(1181_usize), ibig::UBig::from(1187_usize), ibig::UBig::from(1193_usize), ibig::UBig::from(1201_usize), ibig::UBig::from(1213_usize), ibig::UBig::from(1217_usize), ibig::UBig::from(1223_usize), ibig::UBig::from(1229_usize), ibig::UBig::from(1231_usize), ibig::UBig::from(1237_usize), ibig::UBig::from(1249_usize), ibig::UBig::from(1259_usize), ibig::UBig::from(1277_usize), ibig::UBig::from(1279_usize), ibig::UBig::from(1283_usize), ibig::UBig::from(1289_usize), ibig::UBig::from(1291_usize), ibig::UBig::from(1297_usize), ibig::UBig::from(1301_usize), ibig::UBig::from(1303_usize), ibig::UBig::from(1307_usize), ibig::UBig::from(1319_usize), ibig::UBig::from(1321_usize), ibig::UBig::from(1327_usize), ibig::UBig::from(1361_usize), ibig::UBig::from(1367_usize), ibig::UBig::from(1373_usize), ibig::UBig::from(1381_usize), ibig::UBig::from(1399_usize), ibig::UBig::from(1409_usize), ibig::UBig::from(1423_usize), ibig::UBig::from(1427_usize), ibig::UBig::from(1429_usize), ibig::UBig::from(1433_usize), ibig::UBig::from(1439_usize), ibig::UBig::from(1447_usize), ibig::UBig::from(1451_usize), ibig::UBig::from(1453_usize), ibig::UBig::from(1459_usize), ibig::UBig::from(1471_usize), ibig::UBig::from(1481_usize), ibig::UBig::from(1483_usize), ibig::UBig::from(1487_usize), ibig::UBig::from(1489_usize), ibig::UBig::from(1493_usize), ibig::UBig::from(1499_usize), ibig::UBig::from(1511_usize), ibig::UBig::from(1523_usize), ibig::UBig::from(1531_usize), ibig::UBig::from(1543_usize), ibig::UBig::from(1549_usize), ibig::UBig::from(1553_usize), ibig::UBig::from(1559_usize), ibig::UBig::from(1567_usize), ibig::UBig::from(1571_usize), ibig::UBig::from(1579_usize), ibig::UBig::from(1583_usize), ibig::UBig::from(1597_usize), ibig::UBig::from(1601_usize), ibig::UBig::from(1607_usize), ibig::UBig::from(1609_usize), ibig::UBig::from(1613_usize), ibig::UBig::from(1619_usize), ibig::UBig::from(1621_usize), ibig::UBig::from(1627_usize), ibig::UBig::from(1637_usize), ibig::UBig::from(1657_usize), ibig::UBig::from(1663_usize), ibig::UBig::from(1667_usize), ibig::UBig::from(1669_usize), ibig::UBig::from(1693_usize), ibig::UBig::from(1697_usize), ibig::UBig::from(1699_usize), ibig::UBig::from(1709_usize), ibig::UBig::from(1721_usize), ibig::UBig::from(1723_usize), ibig::UBig::from(1733_usize), ibig::UBig::from(1741_usize), ibig::UBig::from(1747_usize), ibig::UBig::from(1753_usize), ibig::UBig::from(1759_usize), ibig::UBig::from(1777_usize), ibig::UBig::from(1783_usize), ibig::UBig::from(1787_usize), ibig::UBig::from(1789_usize), ibig::UBig::from(1801_usize), ibig::UBig::from(1811_usize), ibig::UBig::from(1823_usize), ibig::UBig::from(1831_usize), ibig::UBig::from(1847_usize), ibig::UBig::from(1861_usize), ibig::UBig::from(1867_usize), ibig::UBig::from(1871_usize), ibig::UBig::from(1873_usize), ibig::UBig::from(1877_usize), ibig::UBig::from(1879_usize), ibig::UBig::from(1889_usize), ibig::UBig::from(1901_usize), ibig::UBig::from(1907_usize), ibig::UBig::from(1913_usize), ibig::UBig::from(1931_usize), ibig::UBig::from(1933_usize), ibig::UBig::from(1949_usize), ibig::UBig::from(1951_usize), ibig::UBig::from(1973_usize), ibig::UBig::from(1979_usize), ibig::UBig::from(1987_usize), ibig::UBig::from(1993_usize), ibig::UBig::from(1997_usize), ibig::UBig::from(1999_usize), ibig::UBig::from(2003_usize), ibig::UBig::from(2011_usize), ibig::UBig::from(2017_usize), ibig::UBig::from(2027_usize), ibig::UBig::from(2029_usize), ibig::UBig::from(2039_usize), ibig::UBig::from(2053_usize), ibig::UBig::from(2063_usize), ibig::UBig::from(2069_usize), ibig::UBig::from(2081_usize), ibig::UBig::from(2083_usize), ibig::UBig::from(2087_usize), ibig::UBig::from(2089_usize), ibig::UBig::from(2099_usize), ibig::UBig::from(2111_usize), ibig::UBig::from(2113_usize), ibig::UBig::from(2129_usize), ibig::UBig::from(2131_usize), ibig::UBig::from(2137_usize), ibig::UBig::from(2141_usize), ibig::UBig::from(2143_usize), ibig::UBig::from(2153_usize), ibig::UBig::from(2161_usize), ibig::UBig::from(2179_usize), ibig::UBig::from(2203_usize), ibig::UBig::from(2207_usize), ibig::UBig::from(2213_usize), ibig::UBig::from(2221_usize), ibig::UBig::from(2237_usize), ibig::UBig::from(2239_usize), ibig::UBig::from(2243_usize), ibig::UBig::from(2251_usize), ibig::UBig::from(2267_usize), ibig::UBig::from(2269_usize), ibig::UBig::from(2273_usize), ibig::UBig::from(2281_usize), ibig::UBig::from(2287_usize), ibig::UBig::from(2293_usize), ibig::UBig::from(2297_usize), ibig::UBig::from(2309_usize), ibig::UBig::from(2311_usize), ibig::UBig::from(2333_usize), ibig::UBig::from(2339_usize), ibig::UBig::from(2341_usize), ibig::UBig::from(2347_usize), ibig::UBig::from(2351_usize), ibig::UBig::from(2357_usize), ibig::UBig::from(2371_usize), ibig::UBig::from(2377_usize), ibig::UBig::from(2381_usize), ibig::UBig::from(2383_usize), ibig::UBig::from(2389_usize), ibig::UBig::from(2393_usize), ibig::UBig::from(2399_usize), ibig::UBig::from(2411_usize), ibig::UBig::from(2417_usize), ibig::UBig::from(2423_usize), ibig::UBig::from(2437_usize), ibig::UBig::from(2441_usize), ibig::UBig::from(2447_usize), ibig::UBig::from(2459_usize), ibig::UBig::from(2467_usize), ibig::UBig::from(2473_usize), ibig::UBig::from(2477_usize), ibig::UBig::from(2503_usize), ibig::UBig::from(2521_usize), ibig::UBig::from(2531_usize), ibig::UBig::from(2539_usize), ibig::UBig::from(2543_usize), ibig::UBig::from(2549_usize), ibig::UBig::from(2551_usize), ibig::UBig::from(2557_usize), ibig::UBig::from(2579_usize), ibig::UBig::from(2591_usize), ibig::UBig::from(2593_usize), ibig::UBig::from(2609_usize), ibig::UBig::from(2617_usize), ibig::UBig::from(2621_usize), ibig::UBig::from(2633_usize), ibig::UBig::from(2647_usize), ibig::UBig::from(2657_usize), ibig::UBig::from(2659_usize), ibig::UBig::from(2663_usize), ibig::UBig::from(2671_usize), ibig::UBig::from(2677_usize), ibig::UBig::from(2683_usize), ibig::UBig::from(2687_usize), ibig::UBig::from(2689_usize), ibig::UBig::from(2693_usize), ibig::UBig::from(2699_usize), ibig::UBig::from(2707_usize), ibig::UBig::from(2711_usize), ibig::UBig::from(2713_usize), ibig::UBig::from(2719_usize), ibig::UBig::from(2729_usize), ibig::UBig::from(2731_usize), ibig::UBig::from(2741_usize), ibig::UBig::from(2749_usize), ibig::UBig::from(2753_usize), ibig::UBig::from(2767_usize), ibig::UBig::from(2777_usize), ibig::UBig::from(2789_usize), ibig::UBig::from(2791_usize), ibig::UBig::from(2797_usize), ibig::UBig::from(2801_usize), ibig::UBig::from(2803_usize), ibig::UBig::from(2819_usize), ibig::UBig::from(2833_usize), ibig::UBig::from(2837_usize), ibig::UBig::from(2843_usize), ibig::UBig::from(2851_usize), ibig::UBig::from(2857_usize), ibig::UBig::from(2861_usize), ibig::UBig::from(2879_usize), ibig::UBig::from(2887_usize), ibig::UBig::from(2897_usize), ibig::UBig::from(2903_usize), ibig::UBig::from(2909_usize), ibig::UBig::from(2917_usize), ibig::UBig::from(2927_usize), ibig::UBig::from(2939_usize), ibig::UBig::from(2953_usize), ibig::UBig::from(2957_usize), ibig::UBig::from(2963_usize), ibig::UBig::from(2969_usize), ibig::UBig::from(2971_usize), ibig::UBig::from(2999_usize), ibig::UBig::from(3001_usize), ibig::UBig::from(3011_usize), ibig::UBig::from(3019_usize), ibig::UBig::from(3023_usize), ibig::UBig::from(3037_usize), ibig::UBig::from(3041_usize), ibig::UBig::from(3049_usize), ibig::UBig::from(3061_usize), ibig::UBig::from(3067_usize), ibig::UBig::from(3079_usize), ibig::UBig::from(3083_usize), ibig::UBig::from(3089_usize), ibig::UBig::from(3109_usize), ibig::UBig::from(3119_usize), ibig::UBig::from(3121_usize), ibig::UBig::from(3137_usize), ibig::UBig::from(3163_usize), ibig::UBig::from(3167_usize), ibig::UBig::from(3169_usize), ibig::UBig::from(3181_usize), ibig::UBig::from(3187_usize), ibig::UBig::from(3191_usize), ibig::UBig::from(3203_usize), ibig::UBig::from(3209_usize), ibig::UBig::from(3217_usize), ibig::UBig::from(3221_usize), ibig::UBig::from(3229_usize), ibig::UBig::from(3251_usize), ibig::UBig::from(3253_usize), ibig::UBig::from(3257_usize), ibig::UBig::from(3259_usize), ibig::UBig::from(3271_usize), ibig::UBig::from(3299_usize), ibig::UBig::from(3301_usize), ibig::UBig::from(3307_usize), ibig::UBig::from(3313_usize), ibig::UBig::from(3319_usize), ibig::UBig::from(3323_usize), ibig::UBig::from(3329_usize), ibig::UBig::from(3331_usize), ibig::UBig::from(3343_usize), ibig::UBig::from(3347_usize), ibig::UBig::from(3359_usize), ibig::UBig::from(3361_usize), ibig::UBig::from(3371_usize), ibig::UBig::from(3373_usize), ibig::UBig::from(3389_usize), ibig::UBig::from(3391_usize), ibig::UBig::from(3407_usize), ibig::UBig::from(3413_usize), ibig::UBig::from(3433_usize), ibig::UBig::from(3449_usize), ibig::UBig::from(3457_usize), ibig::UBig::from(3461_usize), ibig::UBig::from(3463_usize), ibig::UBig::from(3467_usize), ibig::UBig::from(3469_usize), ibig::UBig::from(3491_usize), ibig::UBig::from(3499_usize), ibig::UBig::from(3511_usize), ibig::UBig::from(3517_usize), ibig::UBig::from(3527_usize), ibig::UBig::from(3529_usize), ibig::UBig::from(3533_usize), ibig::UBig::from(3539_usize), ibig::UBig::from(3541_usize), ibig::UBig::from(3547_usize), ibig::UBig::from(3557_usize), ibig::UBig::from(3559_usize), ibig::UBig::from(3571_usize), ibig::UBig::from(3581_usize), ibig::UBig::from(3583_usize), ibig::UBig::from(3593_usize), ibig::UBig::from(3607_usize), ibig::UBig::from(3613_usize), ibig::UBig::from(3617_usize), ibig::UBig::from(3623_usize), ibig::UBig::from(3631_usize), ibig::UBig::from(3637_usize), ibig::UBig::from(3643_usize), ibig::UBig::from(3659_usize), ibig::UBig::from(3671_usize), ibig::UBig::from(3673_usize), ibig::UBig::from(3677_usize), ibig::UBig::from(3691_usize), ibig::UBig::from(3697_usize), ibig::UBig::from(3701_usize), ibig::UBig::from(3709_usize), ibig::UBig::from(3719_usize), ibig::UBig::from(3727_usize), ibig::UBig::from(3733_usize), ibig::UBig::from(3739_usize), ibig::UBig::from(3761_usize), ibig::UBig::from(3767_usize), ibig::UBig::from(3769_usize), ibig::UBig::from(3779_usize), ibig::UBig::from(3793_usize), ibig::UBig::from(3797_usize), ibig::UBig::from(3803_usize), ibig::UBig::from(3821_usize), ibig::UBig::from(3823_usize), ibig::UBig::from(3833_usize), ibig::UBig::from(3847_usize), ibig::UBig::from(3851_usize), ibig::UBig::from(3853_usize), ibig::UBig::from(3863_usize), ibig::UBig::from(3877_usize), ibig::UBig::from(3881_usize), ibig::UBig::from(3889_usize), ibig::UBig::from(3907_usize), ibig::UBig::from(3911_usize), ibig::UBig::from(3917_usize), ibig::UBig::from(3919_usize), ibig::UBig::from(3923_usize), ibig::UBig::from(3929_usize), ibig::UBig::from(3931_usize), ibig::UBig::from(3943_usize), ibig::UBig::from(3947_usize), ibig::UBig::from(3967_usize), ibig::UBig::from(3989_usize), ibig::UBig::from(4001_usize), ibig::UBig::from(4003_usize), ibig::UBig::from(4007_usize), ibig::UBig::from(4013_usize), ibig::UBig::from(4019_usize), ibig::UBig::from(4021_usize), ibig::UBig::from(4027_usize), ibig::UBig::from(4049_usize), ibig::UBig::from(4051_usize), ibig::UBig::from(4057_usize), ibig::UBig::from(4073_usize), ibig::UBig::from(4079_usize), ibig::UBig::from(4091_usize), ibig::UBig::from(4093_usize), ibig::UBig::from(4099_usize), ibig::UBig::from(4111_usize), ibig::UBig::from(4127_usize), ibig::UBig::from(4129_usize), ibig::UBig::from(4133_usize), ibig::UBig::from(4139_usize), ibig::UBig::from(4153_usize), ibig::UBig::from(4157_usize), ibig::UBig::from(4159_usize), ibig::UBig::from(4177_usize), ibig::UBig::from(4201_usize), ibig::UBig::from(4211_usize), ibig::UBig::from(4217_usize), ibig::UBig::from(4219_usize), ibig::UBig::from(4229_usize), ibig::UBig::from(4231_usize), ibig::UBig::from(4241_usize), ibig::UBig::from(4243_usize), ibig::UBig::from(4253_usize), ibig::UBig::from(4259_usize), ibig::UBig::from(4261_usize), ibig::UBig::from(4271_usize), ibig::UBig::from(4273_usize), ibig::UBig::from(4283_usize), ibig::UBig::from(4289_usize), ibig::UBig::from(4297_usize), ibig::UBig::from(4327_usize), ibig::UBig::from(4337_usize), ibig::UBig::from(4339_usize), ibig::UBig::from(4349_usize), ibig::UBig::from(4357_usize), ibig::UBig::from(4363_usize), ibig::UBig::from(4373_usize), ibig::UBig::from(4391_usize), ibig::UBig::from(4397_usize), ibig::UBig::from(4409_usize), ibig::UBig::from(4421_usize), ibig::UBig::from(4423_usize), ibig::UBig::from(4441_usize), ibig::UBig::from(4447_usize), ibig::UBig::from(4451_usize), ibig::UBig::from(4457_usize), ibig::UBig::from(4463_usize), ibig::UBig::from(4481_usize), ibig::UBig::from(4483_usize), ibig::UBig::from(4493_usize), ibig::UBig::from(4507_usize), ibig::UBig::from(4513_usize), ibig::UBig::from(4517_usize), ibig::UBig::from(4519_usize), ibig::UBig::from(4523_usize), ibig::UBig::from(4547_usize), ibig::UBig::from(4549_usize), ibig::UBig::from(4561_usize), ibig::UBig::from(4567_usize), ibig::UBig::from(4583_usize), ibig::UBig::from(4591_usize), ibig::UBig::from(4597_usize), ibig::UBig::from(4603_usize), ibig::UBig::from(4621_usize), ibig::UBig::from(4637_usize), ibig::UBig::from(4639_usize), ibig::UBig::from(4643_usize), ibig::UBig::from(4649_usize), ibig::UBig::from(4651_usize), ibig::UBig::from(4657_usize), ibig::UBig::from(4663_usize), ibig::UBig::from(4673_usize), ibig::UBig::from(4679_usize), ibig::UBig::from(4691_usize), ibig::UBig::from(4703_usize), ibig::UBig::from(4721_usize), ibig::UBig::from(4723_usize), ibig::UBig::from(4729_usize), ibig::UBig::from(4733_usize), ibig::UBig::from(4751_usize), ibig::UBig::from(4759_usize), ibig::UBig::from(4783_usize), ibig::UBig::from(4787_usize), ibig::UBig::from(4789_usize), ibig::UBig::from(4793_usize), ibig::UBig::from(4799_usize), ibig::UBig::from(4801_usize), ibig::UBig::from(4813_usize), ibig::UBig::from(4817_usize), ibig::UBig::from(4831_usize), ibig::UBig::from(4861_usize), ibig::UBig::from(4871_usize), ibig::UBig::from(4877_usize), ibig::UBig::from(4889_usize), ibig::UBig::from(4903_usize), ibig::UBig::from(4909_usize), ibig::UBig::from(4919_usize), ibig::UBig::from(4931_usize), ibig::UBig::from(4933_usize), ibig::UBig::from(4937_usize), ibig::UBig::from(4943_usize), ibig::UBig::from(4951_usize), ibig::UBig::from(4957_usize), ibig::UBig::from(4967_usize), ibig::UBig::from(4969_usize), ibig::UBig::from(4973_usize), ibig::UBig::from(4987_usize), ibig::UBig::from(4993_usize), ibig::UBig::from(4999_usize)];
}
impl Miller for ibig::UBig {
    fn probably_prime(&self, rounds: u32) -> IsPrime {
        if self == &ubig!(0) {
            return IsPrime::NotPrime;
        }
        if self <= &ubig!(5) {
            return IsPrime::Probably;
        }
        for num in LOWER_PRIMES.iter() {
            if self % num == ubig!(0) {
              return IsPrime::NotPrime;
            }
        }

        let (mut r, mut s) = (0, self - ubig!(1));
        while &s & ubig!(1) != ubig!(0) {
            r += 1;
            s /= ubig!(2);
        }
        let mut ret = IsPrime::NotPrime;
        let mut rng = thread_rng();
        for _ in 0..rounds {
            let a = rng.gen_range(ubig!(2)..self - ubig!(1));
            let mut x = pow_mod(a, s.clone(), self);
            if x == ubig!(1) || x == (self - ubig!(1)) || &x % self == ubig!(0) {
                continue;
            }
            for _ in 0..r - 1 {
                x = pow_mod(x, ubig!(2), self);
                if x == (self - ubig!(1)) {
                    ret = IsPrime::Probably;
                    break;
                }
            }
            if ret == IsPrime::NotPrime {
                return IsPrime::NotPrime;
            }
        }

        IsPrime::Probably
    }
}

pub trait Miller {
    fn probably_prime(&self, rounds: u32) -> IsPrime;
}

pub fn pow_mod(b: ibig::UBig, e: ibig::UBig, m: &ibig::UBig) -> ibig::UBig {
    if e == ubig!(0) {
      return ubig!(1);
    }
    let mut ret = b.clone();
    let bits = e.bit_len() - 1;

    for bit in (0..bits).rev() {
     ret = ret.pow(2);
      ret %= m;
      if e.bit(bit) {
        ret *= &b;
        ret %= m;
      }
    }
    ret
}

pub fn recursive_mod_pow(b: &ibig::UBig, e: &ibig::UBig, m: &ibig::UBig) -> ibig::UBig {
    if *e == ubig!(0){
      return ubig!(1);
    }
    let tmp = recursive_mod_pow(b, &(e >>1), m) % m;
    if e % ubig!(2) == ubig!(0) {
      (&tmp * &tmp) % m
    }
    else if *e > ubig!(0) {
       (b * &tmp * &tmp) % m
     }
     else {
    ((&tmp * &tmp) / b) % m
     }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ibig::UBig;

    #[test]
    fn it_works() {
        let t = ubig!(3);
        assert_eq!(IsPrime::Probably, t.probably_prime(40));
    }

    #[test]
    fn test_pow_mod_1() {
        assert_eq!(ubig!(2), recursive_mod_pow(&ubig!(5), &ubig!(3), &ubig!(3)));
    }

    #[test]
    fn test_pow_mod_2() {
        let b = ubig!(5);
        let e =  ubig!(150);
        let m = ubig!(60);
        assert_eq!(ubig!(25), pow_mod(b.clone(), e.clone(), &m));
        assert_eq!(ubig!(25), recursive_mod_pow(&b, &e, &m));
    }

    #[test]
    fn test_pow_mod_e0() {
        assert_eq!(ubig!(1), pow_mod(ubig!(5), ubig!(0), &ubig!(60)));
        assert_eq!(ubig!(1), recursive_mod_pow(&ubig!(5), &ubig!(0), &ubig!(60)));
    }

    #[test]
    fn is_prime_25() {
        let t = ubig!(25);
        assert_eq!(IsPrime::NotPrime, t.probably_prime(40))
    }


    #[test]
    fn very_large_prime() {
        let t = "1936738294519690982211090334402079885308248998113910860490043561062398610429619904537193501740559101".parse::<UBig>().unwrap();
        assert_eq!(IsPrime::Probably, t.probably_prime(40));
    }

    #[test]
    fn is_prime_zero() {
        let t = ubig!(0);
        assert_eq!(IsPrime::NotPrime, t.probably_prime(40));
    }
}
