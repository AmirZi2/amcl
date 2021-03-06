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

/* Note that the original curve has been transformed to an isomorphic curve with A=-3 */

use brainpool::big::NLEN;
use super::super::arch::Chunk;
use types::{ModType, CurveType, CurvePairingType, SexticTwist, SignOfX};

// Base Bits= 56
// brainpool Modulus
pub const MODULUS: [Chunk; NLEN] = [
    0x13481D1F6E5377,
    0xF623D526202820,
    0x909D838D726E3B,
    0xA1EEA9BC3E660A,
    0xA9FB57DB,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x9E04F49B9A3787,
    0x29317218F3CF49,
    0x54E8C3CF1DBC89,
    0xBB411A3F7559CA,
    0x9773E15F,
];
pub const MCONST: Chunk = 0xA75590CEFD89B9;

// brainpool Curve
pub const CURVE_COF_I: isize = 1;
pub const CURVE_A: isize = -3;
pub const CURVE_B_I: isize = 0;
pub const CURVE_COF: [Chunk; NLEN] = [0x1, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_B: [Chunk; NLEN] = [
    0xE58101FEE92B04,
    0xEBC4AF2F49256A,
    0x733D0B76B7BF93,
    0x30D84EA4FE66A7,
    0x662C61C4,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x1E0E82974856A7,
    0x7AA3B561A6F790,
    0x909D838D718C39,
    0xA1EEA9BC3E660A,
    0xA9FB57DB,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0xA191562E1305F4,
    0x42C47AAFBC2B79,
    0xB23A656149AFA1,
    0xC1CFE7B7732213,
    0xA3E8EB3C,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0xABE8F35B25C9BE,
    0xB6DE39D027001D,
    0xE14644417E69BC,
    0x3439C56D7F7B22,
    0x2D996C82,
];

pub const MODBYTES: usize = 32;
pub const BASEBITS: usize = 56;

pub const MODBITS: usize = 256;
pub const MOD8: usize = 7;
pub const MODTYPE: ModType = ModType::NOT_SPECIAL;
pub const SH: usize = 24;

pub const CURVETYPE: CurveType = CurveType::WEIERSTRASS;
pub const CURVE_PAIRING_TYPE: CurvePairingType = CurvePairingType::NOT;
pub const SEXTIC_TWIST: SexticTwist = SexticTwist::NOT;
pub const SIGN_OF_X: SignOfX = SignOfX::NOT;
pub const HASH_TYPE: usize = 32;
pub const AESKEY: usize = 16;
