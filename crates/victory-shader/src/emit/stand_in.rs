//! Stand-ins for passes whose results the host cannot produce yet: the
//! title's own program, translated as it is, with every image store writing
//! one constant in every channel. The program keeps its bindings, dispatch
//! shape and the pixels it covers; only the values change.
//!
//! - Ambient light (`VICTORY_AMBIENT_STUB`): the screen-probe temporal
//!   resolve of `indirect_diffuse_lighting`, the last pass of the diffuse
//!   global illumination before the tiled lighting reads it (pass order
//!   measured on a world frame of the title). Its input comes from ray
//!   tracing, which does not run on the host yet, so everything the sun
//!   does not reach is black; a constant there is a floor of ambient light.
//! - Shadow masks (`VICTORY_SHADOWMASK_STUB`): the ray-traced distant
//!   shadow passes (`rtdirectionalshadows`), writing 1 (lit) so that a
//!   trace that finds nothing does not black out the sun.
//!
//! shortcut: the outputs' encodings are not known (the resolve writes
//! several images, history among them; the second order of spherical
//! harmonics would want only its first coefficient set): every channel of
//! every output gets the same value.

use super::HostShader;
use crate::ShaderError;

/// `indirect_diffuse_lighting_CS_ScreenProbes_TemporalResolve*` (with and
/// without reflective caustics and the constant-buffer variant).
pub const AMBIENT_HASHES: &[&str] = &[
    "2bed88a03a2f5558",
    "34fb51b8a0312e82",
    "5053352ac4370ca1",
    "2fc5d11ece80c884",
    "3b4f6eb5edb79848",
    "cf76630573822d04",
];

/// The ambient floor written by default.
pub const AMBIENT_VALUE: f32 = 0.1;

/// `rtdirectionalshadows_CS_RTDistantShadowTrace*` (trace, leak plug, and
/// their temporal upsamples).
pub const SHADOWMASK_HASHES: &[&str] = &[
    "a0544eac81df1fc8",
    "a1b20bc47e3c9597",
    "ff6a098566877642",
    "e8c962358d862e3f",
    "d4c32d1ea3b676be",
    "17829ac71aab4e8d",
    "ba263b6ab4b6d619",
    "afeea48bbb9913ad",
];

/// Lit.
pub const SHADOWMASK_VALUE: f32 = 1.0;

/// `container` translated with every image store writing `value`.
pub fn storing(container: &[u8], value: f32) -> Result<HostShader, ShaderError> {
    super::lower::STORE_VALUE.set(Some(value));
    let r = super::translate(container);
    super::lower::STORE_VALUE.set(None);
    r
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_store_value_is_this_translation_s_alone() {
        assert!(super::storing(b"not a container", 0.5).is_err());
        assert_eq!(super::super::lower::STORE_VALUE.get(), None);
    }
}
