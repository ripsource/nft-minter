use crate::hsl::*;
use crate::{layers::Layer, utils::*};
use random::Random;
use svg::node::element::{Circle, Element};

pub struct SmallElementCircle;

impl Layer for SmallElementCircle {
    fn generate(&self, random: &mut Random, base_color: &Option<HSL>) -> Vec<Element> {
        let random_radius = random.in_range::<u16>(50, 100) * 2; // Always an even number

        let mut circle = Circle::new()
            .set("cx", 500)
            .set("cy", 500)
            .set("r", random_radius);

        // Set the fill, which can be either solid or gradient, with a higher chance of solid than gradient
        if random.roll::<u8>(100) < 85 {
            let color = if base_color.is_some() {
                // Use the base color and derive something similar
                base_color.unwrap().derive_similar_color(random).as_string()
            } else {
                // Pick a random color
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                HSL::new_random(random, color_mode, 100).as_string()
            };

            circle = circle.set("fill", color);

            vec![circle.into()]
        } else {
            // Get a gradient definition
            let (gradient, gradient_name) = if base_color.is_some() {
                // We have a base color, so we derive something similar
                let color1 = base_color.unwrap().derive_similar_color(random);
                let color2 = base_color.unwrap().derive_similar_color(random);

                gradient_definition(random, Some(45), color1, color2)
            } else {
                // Randomize the color mode, but prefer vibrant
                let roll = random.roll::<u8>(100);
                let color_mode = if roll < 30 {
                    ColorMode::Light
                } else {
                    ColorMode::Vibrant
                };

                random_gradient_definition(random, Some(45), color_mode, 100)
            };

            circle = circle.set("fill", format!("url(#{gradient_name})",));

            vec![gradient.into(), circle.into()]
        }
    }
}
