#![allow(unused)]
use crate::{XMVECTORF32, XMVECTORU32, XMVECTORI32, XM_SELECT_0, XM_SELECT_1, XM_PI, XM_2PI, XM_1DIVPI, XM_1DIV2PI, XM_PIDIV2};

const fn float(u: u32) -> f32 {
    u as f32
}

pub const g_XMSinCoefficients0: XMVECTORF32 = XMVECTORF32 { f: [ -0.16666667,  0.0083333310, -0.00019840874,  2.7525562e-06 ] };
pub const g_XMSinCoefficients1: XMVECTORF32 = XMVECTORF32 { f: [ -2.3889859e-08, -0.16665852 /*Est1*/,  0.0083139502 /*Est2*/, -0.00018524670 /*Est3*/ ] };
pub const g_XMCosCoefficients0: XMVECTORF32 = XMVECTORF32 { f: [ -0.5,  0.041666638, -0.0013888378,  2.4760495e-05 ] };
pub const g_XMCosCoefficients1: XMVECTORF32 = XMVECTORF32 { f: [ -2.6051615e-07, -0.49992746 /*Est1*/,  0.041493919 /*Est2*/, -0.0012712436 /*Est3*/ ] };
pub const g_XMTanCoefficients0: XMVECTORF32 = XMVECTORF32 { f: [ 1.0, 0.333333333, 0.133333333, 5.396825397e-2 ] };
pub const g_XMTanCoefficients1: XMVECTORF32 = XMVECTORF32 { f: [ 2.186948854e-2, 8.863235530e-3, 3.592128167e-3, 1.455834485e-3 ] };
pub const g_XMTanCoefficients2: XMVECTORF32 = XMVECTORF32 { f: [ 5.900274264e-4, 2.391290764e-4, 9.691537707e-5, 3.927832950e-5 ] };
pub const g_XMArcCoefficients0: XMVECTORF32 = XMVECTORF32 { f: [  1.5707963050, -0.2145988016,  0.0889789874, -0.0501743046 ] };
pub const g_XMArcCoefficients1: XMVECTORF32 = XMVECTORF32 { f: [  0.0308918810, -0.0170881256,  0.0066700901, -0.0012624911 ] };
pub const g_XMATanCoefficients0: XMVECTORF32 = XMVECTORF32 { f: [ -0.3333314528,  0.1999355085, -0.1420889944,  0.1065626393 ] };
pub const g_XMATanCoefficients1: XMVECTORF32 = XMVECTORF32 { f: [ -0.0752896400,  0.0429096138, -0.0161657367,  0.0028662257 ] };
pub const g_XMATanEstCoefficients0: XMVECTORF32 = XMVECTORF32 { f: [  0.999866,  0.999866,  0.999866,  0.999866 ] };
pub const g_XMATanEstCoefficients1: XMVECTORF32 = XMVECTORF32 { f: [ -0.3302995,  0.180141, -0.085133,  0.0208351 ] };
pub const g_XMTanEstCoefficients: XMVECTORF32 = XMVECTORF32 { f: [ 2.484, -1.954923183e-1, 2.467401101, XM_1DIVPI ] };
pub const g_XMArcEstCoefficients: XMVECTORF32 = XMVECTORF32 { f: [  1.5707288, -0.2121144,  0.0742610, -0.0187293 ] };
pub const g_XMPiConstants0: XMVECTORF32 = XMVECTORF32 { f: [ XM_PI, XM_2PI, XM_1DIVPI, XM_1DIV2PI ] };
pub const g_XMIdentityR0: XMVECTORF32 = XMVECTORF32 { f: [ 1.0, 0.0, 0.0, 0.0 ] };
pub const g_XMIdentityR1: XMVECTORF32 = XMVECTORF32 { f: [ 0.0, 1.0, 0.0, 0.0 ] };
pub const g_XMIdentityR2: XMVECTORF32 = XMVECTORF32 { f: [ 0.0, 0.0, 1.0, 0.0 ] };
pub const g_XMIdentityR3: XMVECTORF32 = XMVECTORF32 { f: [ 0.0, 0.0, 0.0, 1.0 ] };
pub const g_XMNegIdentityR0: XMVECTORF32 = XMVECTORF32 { f: [ -1.0, 0.0, 0.0, 0.0 ] };
pub const g_XMNegIdentityR1: XMVECTORF32 = XMVECTORF32 { f: [ 0.0, -1.0, 0.0, 0.0 ] };
pub const g_XMNegIdentityR2: XMVECTORF32 = XMVECTORF32 { f: [ 0.0, 0.0, -1.0, 0.0 ] };
pub const g_XMNegIdentityR3: XMVECTORF32 = XMVECTORF32 { f: [ 0.0, 0.0, 0.0, -1.0 ] };
pub const g_XMNegativeZero: XMVECTORU32 = XMVECTORU32 { u: [ 0x80000000, 0x80000000, 0x80000000, 0x80000000 ] };
pub const g_XMNegate3: XMVECTORU32 = XMVECTORU32 { u: [ 0x80000000, 0x80000000, 0x80000000, 0x00000000 ] };
pub const g_XMMaskXY: XMVECTORU32 = XMVECTORU32 { u: [ 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000, 0x00000000 ] };
pub const g_XMMask3: XMVECTORU32 = XMVECTORU32 { u: [ 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0x00000000 ] };
pub const g_XMMaskX: XMVECTORU32 = XMVECTORU32 { u: [ 0xFFFFFFFF, 0x00000000, 0x00000000, 0x00000000 ] };
pub const g_XMMaskY: XMVECTORU32 = XMVECTORU32 { u: [ 0x00000000, 0xFFFFFFFF, 0x00000000, 0x00000000 ] };
pub const g_XMMaskZ: XMVECTORU32 = XMVECTORU32 { u: [ 0x00000000, 0x00000000, 0xFFFFFFFF, 0x00000000 ] };
pub const g_XMMaskW: XMVECTORU32 = XMVECTORU32 { u: [ 0x00000000, 0x00000000, 0x00000000, 0xFFFFFFFF ] };
pub const g_XMOne: XMVECTORF32 = XMVECTORF32 { f: [ 1.0, 1.0, 1.0, 1.0 ] };
pub const g_XMOne3: XMVECTORF32 = XMVECTORF32 { f: [ 1.0, 1.0, 1.0, 0.0 ] };
pub const g_XMZero: XMVECTORF32 = XMVECTORF32 { f: [ 0.0, 0.0, 0.0, 0.0 ] };
pub const g_XMTwo: XMVECTORF32 = XMVECTORF32 { f: [ 2.0, 2.0, 2.0, 2.0 ] };
pub const g_XMFour: XMVECTORF32 = XMVECTORF32 { f: [ 4.0, 4.0, 4.0, 4.0 ] };
pub const g_XMSix: XMVECTORF32 = XMVECTORF32 { f: [ 6.0, 6.0, 6.0, 6.0 ] };
pub const g_XMNegativeOne: XMVECTORF32 = XMVECTORF32 { f: [ -1.0, -1.0, -1.0, -1.0 ] };
pub const g_XMOneHalf: XMVECTORF32 = XMVECTORF32 { f: [ 0.5, 0.5, 0.5, 0.5 ] };
pub const g_XMNegativeOneHalf: XMVECTORF32 = XMVECTORF32 { f: [ -0.5, -0.5, -0.5, -0.5 ] };
pub const g_XMNegativeTwoPi: XMVECTORF32 = XMVECTORF32 { f: [ -XM_2PI, -XM_2PI, -XM_2PI, -XM_2PI ] };
pub const g_XMNegativePi: XMVECTORF32 = XMVECTORF32 { f: [ -XM_PI, -XM_PI, -XM_PI, -XM_PI ] };
pub const g_XMHalfPi: XMVECTORF32 = XMVECTORF32 { f: [ XM_PIDIV2, XM_PIDIV2, XM_PIDIV2, XM_PIDIV2 ] };
pub const g_XMPi: XMVECTORF32 = XMVECTORF32 { f: [ XM_PI, XM_PI, XM_PI, XM_PI ] };
pub const g_XMReciprocalPi: XMVECTORF32 = XMVECTORF32 { f: [ XM_1DIVPI, XM_1DIVPI, XM_1DIVPI, XM_1DIVPI ] };
pub const g_XMTwoPi: XMVECTORF32 = XMVECTORF32 { f: [ XM_2PI, XM_2PI, XM_2PI, XM_2PI ] };
pub const g_XMReciprocalTwoPi: XMVECTORF32 = XMVECTORF32 { f: [ XM_1DIV2PI, XM_1DIV2PI, XM_1DIV2PI, XM_1DIV2PI ] };
pub const g_XMEpsilon: XMVECTORF32 = XMVECTORF32 { f: [ 1.192092896e-7, 1.192092896e-7, 1.192092896e-7, 1.192092896e-7 ] };
pub const g_XMInfinity: XMVECTORI32 = XMVECTORI32 { i: [ 0x7F800000, 0x7F800000, 0x7F800000, 0x7F800000 ] };
pub const g_XMQNaN: XMVECTORI32 = XMVECTORI32 { i: [ 0x7FC00000, 0x7FC00000, 0x7FC00000, 0x7FC00000 ] };
pub const g_XMQNaNTest: XMVECTORI32 = XMVECTORI32 { i: [ 0x007FFFFF, 0x007FFFFF, 0x007FFFFF, 0x007FFFFF ] };
pub const g_XMAbsMask: XMVECTORI32 = XMVECTORI32 { i: [ 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF, 0x7FFFFFFF ] };
pub const g_XMFltMin: XMVECTORI32 = XMVECTORI32 { i: [ 0x00800000, 0x00800000, 0x00800000, 0x00800000 ] };
pub const g_XMFltMax: XMVECTORI32 = XMVECTORI32 { i: [ 0x7F7FFFFF, 0x7F7FFFFF, 0x7F7FFFFF, 0x7F7FFFFF ] };
pub const g_XMNegOneMask: XMVECTORU32 = XMVECTORU32 { u: [ 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF ] };
pub const g_XMMaskA8R8G8B8: XMVECTORU32 = XMVECTORU32 { u: [ 0x00FF0000, 0x0000FF00, 0x000000FF, 0xFF000000 ] };
pub const g_XMFlipA8R8G8B8: XMVECTORU32 = XMVECTORU32 { u: [ 0x00000000, 0x00000000, 0x00000000, 0x80000000 ] };
pub const g_XMFixAA8R8G8B8: XMVECTORF32 = XMVECTORF32 { f: [ 0.0, 0.0, 0.0, float(0x80000000u32) ] };
pub const g_XMNormalizeA8R8G8B8: XMVECTORF32 = XMVECTORF32 { f: [ 1.0 / (255.0 * float(0x10000)), 1.0 / (255.0 * float(0x100)), 1.0 / 255.0, 1.0 / (255.0 * float(0x1000000)) ] };
pub const g_XMMaskA2B10G10R10: XMVECTORU32 = XMVECTORU32 { u: [ 0x000003FF, 0x000FFC00, 0x3FF00000, 0xC0000000 ] };
pub const g_XMFlipA2B10G10R10: XMVECTORU32 = XMVECTORU32 { u: [ 0x00000200, 0x00080000, 0x20000000, 0x80000000 ] };
pub const g_XMFixAA2B10G10R10: XMVECTORF32 = XMVECTORF32 { f: [ -512.0, -512.0 * float(0x400), -512.0 * float(0x100000), float(0x80000000u32) ] };
pub const g_XMNormalizeA2B10G10R10: XMVECTORF32 = XMVECTORF32 { f: [ 1.0 / 511.0, 1.0 / (511.0 * float(0x400)), 1.0 / (511.0 * float(0x100000)), 1.0 / (3.0 * float(0x40000000)) ] };
pub const g_XMMaskX16Y16: XMVECTORU32 = XMVECTORU32 { u: [ 0x0000FFFF, 0xFFFF0000, 0x00000000, 0x00000000 ] };
pub const g_XMFlipX16Y16: XMVECTORI32 = XMVECTORI32 { i: [ 0x00008000, 0x00000000, 0x00000000, 0x00000000 ] };
pub const g_XMFixX16Y16: XMVECTORF32 = XMVECTORF32 { f: [ -32768.0, 0.0, 0.0, 0.0 ] };
pub const g_XMNormalizeX16Y16: XMVECTORF32 = XMVECTORF32 { f: [ 1.0 / 32767.0, 1.0 / (32767.0 * 65536.0), 0.0, 0.0 ] };
pub const g_XMMaskX16Y16Z16W16: XMVECTORU32 = XMVECTORU32 { u: [ 0x0000FFFF, 0x0000FFFF, 0xFFFF0000, 0xFFFF0000 ] };
pub const g_XMFlipX16Y16Z16W16: XMVECTORI32 = XMVECTORI32 { i: [ 0x00008000, 0x00008000, 0x00000000, 0x00000000 ] };
pub const g_XMFixX16Y16Z16W16: XMVECTORF32 = XMVECTORF32 { f: [ -32768.0, -32768.0, 0.0, 0.0 ] };
pub const g_XMNormalizeX16Y16Z16W16: XMVECTORF32 = XMVECTORF32 { f: [ 1.0 / 32767.0, 1.0 / 32767.0, 1.0 / (32767.0 * 65536.0), 1.0 / (32767.0 * 65536.0) ] };
pub const g_XMNoFraction: XMVECTORF32 = XMVECTORF32 { f: [ 8388608.0, 8388608.0, 8388608.0, 8388608.0 ] };
pub const g_XMMaskByte: XMVECTORI32 = XMVECTORI32 { i: [ 0x000000FF, 0x000000FF, 0x000000FF, 0x000000FF ] };
pub const g_XMNegateX: XMVECTORF32 = XMVECTORF32 { f: [ -1.0, 1.0, 1.0, 1.0 ] };
pub const g_XMNegateY: XMVECTORF32 = XMVECTORF32 { f: [ 1.0, -1.0, 1.0, 1.0 ] };
pub const g_XMNegateZ: XMVECTORF32 = XMVECTORF32 { f: [ 1.0, 1.0, -1.0, 1.0 ] };
pub const g_XMNegateW: XMVECTORF32 = XMVECTORF32 { f: [ 1.0, 1.0, 1.0, -1.0 ] };
pub const g_XMSelect0101: XMVECTORU32 = XMVECTORU32 { u: [ XM_SELECT_0, XM_SELECT_1, XM_SELECT_0, XM_SELECT_1 ] };
pub const g_XMSelect1010: XMVECTORU32 = XMVECTORU32 { u: [ XM_SELECT_1, XM_SELECT_0, XM_SELECT_1, XM_SELECT_0 ] };
pub const g_XMOneHalfMinusEpsilon: XMVECTORI32 = XMVECTORI32 { i: [ 0x3EFFFFFD, 0x3EFFFFFD, 0x3EFFFFFD, 0x3EFFFFFD ] };
pub const g_XMSelect1000: XMVECTORU32 = XMVECTORU32 { u: [ XM_SELECT_1, XM_SELECT_0, XM_SELECT_0, XM_SELECT_0 ] };
pub const g_XMSelect1100: XMVECTORU32 = XMVECTORU32 { u: [ XM_SELECT_1, XM_SELECT_1, XM_SELECT_0, XM_SELECT_0 ] };
pub const g_XMSelect1110: XMVECTORU32 = XMVECTORU32 { u: [ XM_SELECT_1, XM_SELECT_1, XM_SELECT_1, XM_SELECT_0 ] };
pub const g_XMSelect1011: XMVECTORU32 = XMVECTORU32 { u: [ XM_SELECT_1, XM_SELECT_0, XM_SELECT_1, XM_SELECT_1 ] };
pub const g_XMFixupY16: XMVECTORF32 = XMVECTORF32 { f: [ 1.0, 1.0 / 65536.0, 0.0, 0.0 ] };
pub const g_XMFixupY16W16: XMVECTORF32 = XMVECTORF32 { f: [ 1.0, 1.0, 1.0 / 65536.0, 1.0 / 65536.0 ] };
pub const g_XMFlipY: XMVECTORU32 = XMVECTORU32 { u: [ 0, 0x80000000, 0, 0 ] };
pub const g_XMFlipZ: XMVECTORU32 = XMVECTORU32 { u: [ 0, 0, 0x80000000, 0 ] };
pub const g_XMFlipW: XMVECTORU32 = XMVECTORU32 { u: [ 0, 0, 0, 0x80000000 ] };
pub const g_XMFlipYZ: XMVECTORU32 = XMVECTORU32 { u: [ 0, 0x80000000, 0x80000000, 0 ] };
pub const g_XMFlipZW: XMVECTORU32 = XMVECTORU32 { u: [ 0, 0, 0x80000000, 0x80000000 ] };
pub const g_XMFlipYW: XMVECTORU32 = XMVECTORU32 { u: [ 0, 0x80000000, 0, 0x80000000 ] };
pub const g_XMMaskDec4: XMVECTORI32 = XMVECTORI32 { i: [ 0x3FF, 0x3FF << 10, 0x3FF << 20, (0xC0000000u32 as i32) ] };
pub const g_XMXorDec4: XMVECTORI32 = XMVECTORI32 { i: [ 0x200, 0x200 << 10, 0x200 << 20, 0 ] };
pub const g_XMAddUDec4: XMVECTORF32 = XMVECTORF32 { f: [ 0.0, 0.0, 0.0, 32768.0 * 65536.0 ] };
pub const g_XMAddDec4: XMVECTORF32 = XMVECTORF32 { f: [ -512.0, -512.0 * 1024.0, -512.0 * 1024.0 * 1024.0, 0.0 ] };
pub const g_XMMulDec4: XMVECTORF32 = XMVECTORF32 { f: [ 1.0, 1.0 / 1024.0, 1.0 / (1024.0 * 1024.0), 1.0 / (1024.0 * 1024.0 * 1024.0) ] };
pub const g_XMMaskByte4: XMVECTORU32 = XMVECTORU32 { u: [ 0xFF, 0xFF00, 0xFF0000, 0xFF000000 ] };
pub const g_XMXorByte4: XMVECTORI32 = XMVECTORI32 { i: [ 0x80, 0x8000, 0x800000, 0x00000000 ] };
pub const g_XMAddByte4: XMVECTORF32 = XMVECTORF32 { f: [ -128.0, -128.0 * 256.0, -128.0 * 65536.0, 0.0 ] };
pub const g_XMFixUnsigned: XMVECTORF32 = XMVECTORF32 { f: [ 32768.0 * 65536.0, 32768.0 * 65536.0, 32768.0 * 65536.0, 32768.0 * 65536.0 ] };
pub const g_XMMaxInt: XMVECTORF32 = XMVECTORF32 { f: [ 65536.0 * 32768.0 - 128.0, 65536.0 * 32768.0 - 128.0, 65536.0 * 32768.0 - 128.0, 65536.0 * 32768.0 - 128.0 ] };
pub const g_XMMaxUInt: XMVECTORF32 = XMVECTORF32 { f: [ 65536.0 * 65536.0 - 256.0, 65536.0 * 65536.0 - 256.0, 65536.0 * 65536.0 - 256.0, 65536.0 * 65536.0 - 256.0 ] };
pub const g_XMUnsignedFix: XMVECTORF32 = XMVECTORF32 { f: [ 32768.0 * 65536.0, 32768.0 * 65536.0, 32768.0 * 65536.0, 32768.0 * 65536.0 ] };
pub const g_XMsrgbScale: XMVECTORF32 = XMVECTORF32 { f: [ 12.92, 12.92, 12.92, 1.0 ] };
pub const g_XMsrgbA: XMVECTORF32 = XMVECTORF32 { f: [ 0.055, 0.055, 0.055, 0.0 ] };
pub const g_XMsrgbA1: XMVECTORF32 = XMVECTORF32 { f: [ 1.055, 1.055, 1.055, 1.0 ] };
pub const g_XMExponentBias: XMVECTORI32 = XMVECTORI32 { i: [ 127, 127, 127, 127 ] };
pub const g_XMSubnormalExponent: XMVECTORI32 = XMVECTORI32 { i: [ -126, -126, -126, -126 ] };
pub const g_XMNumTrailing: XMVECTORI32 = XMVECTORI32 { i: [ 23, 23, 23, 23 ] };
pub const g_XMMinNormal: XMVECTORI32 = XMVECTORI32 { i: [ 0x00800000, 0x00800000, 0x00800000, 0x00800000 ] };
pub const g_XMNegInfinity: XMVECTORU32 = XMVECTORU32 { u: [ 0xFF800000, 0xFF800000, 0xFF800000, 0xFF800000 ] };
pub const g_XMNegQNaN: XMVECTORU32 = XMVECTORU32 { u: [ 0xFFC00000, 0xFFC00000, 0xFFC00000, 0xFFC00000 ] };
pub const g_XMBin128: XMVECTORI32 = XMVECTORI32 { i: [ 0x43000000, 0x43000000, 0x43000000, 0x43000000 ] };
pub const g_XMBinNeg150: XMVECTORU32 = XMVECTORU32 { u: [ 0xC3160000, 0xC3160000, 0xC3160000, 0xC3160000 ] };
pub const g_XM253: XMVECTORI32 = XMVECTORI32 { i: [ 253, 253, 253, 253 ] };
pub const g_XMExpEst1: XMVECTORF32 = XMVECTORF32 { f: [ -6.93147182e-1, -6.93147182e-1, -6.93147182e-1, -6.93147182e-1 ] };
pub const g_XMExpEst2: XMVECTORF32 = XMVECTORF32 { f: [  2.40226462e-1,  2.40226462e-1,  2.40226462e-1,  2.40226462e-1 ] };
pub const g_XMExpEst3: XMVECTORF32 = XMVECTORF32 { f: [ -5.55036440e-2, -5.55036440e-2, -5.55036440e-2, -5.55036440e-2 ] };
pub const g_XMExpEst4: XMVECTORF32 = XMVECTORF32 { f: [  9.61597636e-3,  9.61597636e-3,  9.61597636e-3,  9.61597636e-3 ] };
pub const g_XMExpEst5: XMVECTORF32 = XMVECTORF32 { f: [ -1.32823968e-3, -1.32823968e-3, -1.32823968e-3, -1.32823968e-3 ] };
pub const g_XMExpEst6: XMVECTORF32 = XMVECTORF32 { f: [  1.47491097e-4,  1.47491097e-4,  1.47491097e-4,  1.47491097e-4 ] };
pub const g_XMExpEst7: XMVECTORF32 = XMVECTORF32 { f: [ -1.08635004e-5, -1.08635004e-5, -1.08635004e-5, -1.08635004e-5 ] };
pub const g_XMLogEst0: XMVECTORF32 = XMVECTORF32 { f: [  1.442693,  1.442693,  1.442693,  1.442693 ] };
pub const g_XMLogEst1: XMVECTORF32 = XMVECTORF32 { f: [ -0.721242, -0.721242, -0.721242, -0.721242 ] };
pub const g_XMLogEst2: XMVECTORF32 = XMVECTORF32 { f: [  0.479384,  0.479384,  0.479384,  0.479384 ] };
pub const g_XMLogEst3: XMVECTORF32 = XMVECTORF32 { f: [ -0.350295, -0.350295, -0.350295, -0.350295 ] };
pub const g_XMLogEst4: XMVECTORF32 = XMVECTORF32 { f: [  0.248590,  0.248590,  0.248590,  0.248590 ] };
pub const g_XMLogEst5: XMVECTORF32 = XMVECTORF32 { f: [ -0.145700, -0.145700, -0.145700, -0.145700 ] };
pub const g_XMLogEst6: XMVECTORF32 = XMVECTORF32 { f: [  0.057148,  0.057148,  0.057148,  0.057148 ] };
pub const g_XMLogEst7: XMVECTORF32 = XMVECTORF32 { f: [ -0.010578, -0.010578, -0.010578, -0.010578 ] };
pub const g_XMLgE: XMVECTORF32 = XMVECTORF32 { f: [  1.442695,  1.442695,  1.442695,  1.442695 ] };
pub const g_XMInvLgE: XMVECTORF32 = XMVECTORF32 { f: [  6.93147182e-1,  6.93147182e-1,  6.93147182e-1,  6.93147182e-1 ] };
pub const g_UByteMax: XMVECTORF32 = XMVECTORF32 { f: [ 255.0, 255.0, 255.0, 255.0 ] };
pub const g_ByteMin: XMVECTORF32 = XMVECTORF32 { f: [ -127.0, -127.0, -127.0, -127.0 ] };
pub const g_ByteMax: XMVECTORF32 = XMVECTORF32 { f: [ 127.0, 127.0, 127.0, 127.0 ] };
pub const g_ShortMin: XMVECTORF32 = XMVECTORF32 { f: [ -32767.0, -32767.0, -32767.0, -32767.0 ] };
pub const g_ShortMax: XMVECTORF32 = XMVECTORF32 { f: [ 32767.0, 32767.0, 32767.0, 32767.0 ] };
pub const g_UShortMax: XMVECTORF32 = XMVECTORF32 { f: [ 65535.0, 65535.0, 65535.0, 65535.0 ] };