use anyhow::Result;

use crate::cmd::{GenerateGallery, Run};
use crate::gallery::GalleryGenerator;

impl Run for GenerateGallery {
    fn run(&self) -> Result<()> {
        let generator = GalleryGenerator::new(self.output.clone());
        generator.generate()?;
        Ok(())
    }
}
