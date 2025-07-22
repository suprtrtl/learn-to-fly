use crate::*;

#[derive(Debug)]
pub struct Animal {
    pub(crate) position: na::Point2<f32>,
    pub(crate) rotation: na::Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eye: Eye,
    pub(crate) brain: nn::Network,
}

impl Animal {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();

        let brain = nn::Network::random(
            rng,
            &[
                // Input Layer
                //
                // Eye returns Vec<f32>, network uses Vec<f32>
                // pass directly
                nn::LayerTopology {
                    neurons: eye.cells(),
                },
                // Hidden Layer
                // No best answer for how many neurons or how many layers
                // Start with one layer and work your way up
                nn::LayerTopology {
                    neurons: 2 * eye.cells(),
                },

                // Output Layer
                //
                // Speed + Rotation = 2 Neurons
                nn::LayerTopology { neurons: 2 }
            ],
        );

        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}
