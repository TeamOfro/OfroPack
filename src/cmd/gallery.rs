use anyhow::Result;

use crate::cmd::{GenerateGallery, Run};
use crate::services::GalleryGenerator;

impl Run for GenerateGallery {
    fn run(&self) -> Result<()> {
        let generator = GalleryGenerator::new(self.output.clone());
        generator.generate()?;
        Ok(())
    }
}
