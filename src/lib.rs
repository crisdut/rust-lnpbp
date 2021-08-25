// LNP/BP Core Library implementing LNPBP specifications & standards
// Written in 2019 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#![doc = include_str!("../README.md")]
// Coding conventions
#![recursion_limit = "256"]
#![deny(dead_code, missing_docs, warnings)]
// TODO #184: when we will be ready for the release #![deny(missing_docs)]

pub extern crate lnpbp_bech32 as bech32;
pub extern crate lnpbp_chain as chain;
#[cfg(feature = "elgamal")]
pub extern crate lnpbp_elgamal as elgamal;
