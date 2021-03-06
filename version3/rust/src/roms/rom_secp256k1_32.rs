/*
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

/* Fixed Data in ROM - Field and Curve parameters */

use secp256k1::big::NLEN;
use super::super::arch::Chunk;
use types::{ModType, CurveType, CurvePairingType, SexticTwist, SignOfX};

// Base Bits= 28
// secp256k1 modulus
pub const MODULUS: [Chunk; NLEN] = [
    0xFFFFC2F, 0xFFFFFEF, 0xFFFFFFF, 0xFFFFFFF, 0xFFFFFFF, 0xFFFFFFF, 0xFFFFFFF, 0xFFFFFFF,
    0xFFFFFFF, 0xF,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x0, 0xA100000, 0x2000E90, 0x7A, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const MCONST: Chunk = 0x2253531;

// secp256k1 curve
pub const CURVE_A: isize = 0;
pub const CURVE_COF_I: isize = 1;
pub const CURVE_COF: [Chunk; NLEN] = [0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_B_I: isize = 7;
pub const CURVE_B: [Chunk; NLEN] = [0x7, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x364141, 0xD25E8CD, 0x8A03BBF, 0xDCE6AF4, 0xFFEBAAE, 0xFFFFFFF, 0xFFFFFFF, 0xFFFFFFF,
    0xFFFFFFF, 0xF,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0x6F81798, 0xF2815B1, 0xE28D959, 0xFCDB2DC, 0xB07029B, 0x95CE870, 0xC55A062, 0xF9DCBBA,
    0x9BE667E, 0x7,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0xB10D4B8, 0x47D08FF, 0x554199C, 0xB448A68, 0x8A8FD17, 0xFC0E110, 0x55DA4FB, 0x26A3C46,
    0x83ADA77, 0x4,
];

pub const MODBYTES: usize = 32;
pub const BASEBITS: usize = 28;

pub const MODBITS: usize = 256;
pub const MOD8: usize = 7;
pub const MODTYPE: ModType = ModType::NOT_SPECIAL;
pub const SH: usize = 14;

pub const CURVETYPE: CurveType = CurveType::WEIERSTRASS;
pub const CURVE_PAIRING_TYPE: CurvePairingType = CurvePairingType::NOT;
pub const SEXTIC_TWIST: SexticTwist = SexticTwist::NOT;
pub const SIGN_OF_X: SignOfX = SignOfX::NOT;
pub const HASH_TYPE: usize = 32;
pub const AESKEY: usize = 16;
