use std::any::TypeId;

use crate::layers::Layer;
use arch::SmallElementArch;
use random::Random;
use scrypto::prelude::ToPrimitive;
use small_circle::SmallElementCircle;
use small_element_square::SmallElementSquare;
use split_circle::SmallElementSplitCircle;

pub mod arch;
pub mod small_circle;
pub mod small_element_square;
pub mod split_circle;

pub fn random_small_element(random: &mut Random, exclusions: &[TypeId]) -> Option<Box<dyn Layer>> {
    let available_layers: Vec<Box<dyn Layer>> = vec![
        Box::new(SmallElementCircle),
        Box::new(SmallElementSquare),
        Box::new(SmallElementArch),
        Box::new(SmallElementSplitCircle),
    ];

    // Filter out the excluded layers
    let allowed_layers: Vec<Box<dyn Layer>> = available_layers
        .into_iter()
        .filter(|layer| !exclusions.contains(&layer.layer_type()))
        .collect();

    if !allowed_layers.is_empty() {
        // Pick a random layer
        let variant = random
            .roll::<u8>(allowed_layers.len().to_u8().unwrap())
            .to_usize()
            .unwrap();

        Some(allowed_layers.into_iter().nth(variant).unwrap())
    } else {
        None
    }
}
