use encoding_rs::SHIFT_JIS;

use crate::{
    mpl::MPLKeyFrame,
    utils::{Quaternion, Vector3},
};
use std::io::{Cursor, Write};

#[derive(Debug, Clone)]
pub struct VMDWriter {
    pub key_frames: Vec<MPLKeyFrame>,
}

impl VMDWriter {
    pub fn new(key_frames: Vec<MPLKeyFrame>) -> Self {
        Self { key_frames }
    }

    /// Write a bone frame to the buffer
    fn write_bone_frame(
        cursor: &mut Cursor<Vec<u8>>,
        name: &str,
        frame: u32,
        position: Vector3,
        rotation: Quaternion,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Write bone name (15 bytes)
        let (name_bytes, _, _) = SHIFT_JIS.encode(name);
        let mut name_buffer = [0u8; 15];
        for (i, &byte) in name_bytes.iter().enumerate() {
            if i < 15 {
                name_buffer[i] = byte;
            }
        }
        cursor.write_all(&name_buffer)?;

        // Write frame number (4 bytes, little endian)
        cursor.write_all(&frame.to_le_bytes())?;

        // Write position (12 bytes: 3 x f32, little endian)
        cursor.write_all(&position.x.to_le_bytes())?;
        cursor.write_all(&position.y.to_le_bytes())?;
        cursor.write_all(&position.z.to_le_bytes())?;

        // Write rotation quaternion (16 bytes: 4 x f32, little endian)
        cursor.write_all(&rotation.x.to_le_bytes())?;
        cursor.write_all(&rotation.y.to_le_bytes())?;
        cursor.write_all(&rotation.z.to_le_bytes())?;
        cursor.write_all(&rotation.w.to_le_bytes())?;

        // Write interpolation parameters (64 bytes, all set to 20)
        let interpolation = [20u8; 64];
        cursor.write_all(&interpolation)?;

        Ok(())
    }

    /// Write a morph frame to the buffer  
    fn write_morph_frame(
        cursor: &mut Cursor<Vec<u8>>,
        name: &str,
        frame: u32,
        weight: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Write morph name (15 bytes)
        let (name_bytes, _, _) = SHIFT_JIS.encode(name);
        let mut name_buffer = [0u8; 15];
        for (i, &byte) in name_bytes.iter().enumerate() {
            if i < 15 {
                name_buffer[i] = byte;
            }
        }
        cursor.write_all(&name_buffer)?;

        // Write frame number (4 bytes, little endian)
        cursor.write_all(&frame.to_le_bytes())?;

        // Write weight (4 bytes, little endian)
        cursor.write_all(&weight.to_le_bytes())?;

        Ok(())
    }

    /// Create VMD file data from recorded frames
    pub fn create_vmd(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if self.key_frames.is_empty() {
            return Ok(Vec::new());
        }

        // Count total bone frame entries across all keyframes
        let total_bone_frames: u32 = self
            .key_frames
            .iter()
            .map(|kf| kf.bone_frames.len() as u32)
            .sum();

        // Count total morph frame entries across all keyframes
        let total_morph_frames: u32 = self
            .key_frames
            .iter()
            .map(|kf| kf.morph_frames.len() as u32)
            .sum();

        // Calculate sizes
        let header_size = 30 + 20; // Header + model name
        let bone_frame_size = 15 + 4 + 12 + 16 + 64; // 111 bytes per bone frame
        let morph_frame_size = 15 + 4 + 4; // 23 bytes per morph frame
        let total_size = header_size + 4 + // bone frame count
                    (bone_frame_size * total_bone_frames) as usize +
                    4 + // morph frame count  
                    (morph_frame_size * total_morph_frames) as usize +
                    4 + 4 + 4; // camera, light, self shadow counts

        let buffer = Vec::with_capacity(total_size);
        let mut cursor = Cursor::new(buffer);

        // Write header (30 bytes)
        let header = "Vocaloid Motion Data 0002";
        let mut header_buffer = [0u8; 30];
        for (i, byte) in header.bytes().enumerate() {
            if i < 30 {
                header_buffer[i] = byte;
            }
        }
        cursor.write_all(&header_buffer)?;

        // Write model name (20 bytes, empty)
        let model_name_buffer = [0u8; 20];
        cursor.write_all(&model_name_buffer)?;

        // Write bone frame count
        cursor.write_all(&total_bone_frames.to_le_bytes())?;

        // Write bone frames
        for frame in &self.key_frames {
            let frame_number = (frame.time * 60.0) as u32; // Convert seconds to frame at 60 fps
            for bone_frame in &frame.bone_frames {
                Self::write_bone_frame(
                    &mut cursor,
                    &bone_frame.name_jp(),
                    frame_number,
                    bone_frame.position(),
                    bone_frame.rotation(),
                )?;
            }
        }

        // Write morph frame count
        cursor.write_all(&total_morph_frames.to_le_bytes())?;

        // Write morph frames
        for frame in &self.key_frames {
            let frame_number = (frame.time * 60.0) as u32; // Convert seconds to frame at 60 fps
            for morph_frame in &frame.morph_frames {
                Self::write_morph_frame(
                    &mut cursor,
                    &morph_frame.name_jp,
                    frame_number,
                    morph_frame.weight,
                )?;
            }
        }

        // Write counts for other frame types (all 0)
        cursor.write_all(&0u32.to_le_bytes())?; // Camera keyframe count
        cursor.write_all(&0u32.to_le_bytes())?; // Light keyframe count
        cursor.write_all(&0u32.to_le_bytes())?; // Self shadow keyframe count

        Ok(cursor.into_inner())
    }
}
