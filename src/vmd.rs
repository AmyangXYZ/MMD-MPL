use encoding_rs::SHIFT_JIS;

use crate::{
    mpl::MPLKeyFrame,
    utils::{Quaternion, Vector3},
    with_bone_db,
};
use std::io::{Cursor, Read, Write};

const FRAME_RATE: f32 = 30.0;

fn create_ease_in_out_interpolation() -> [u8; 64] {
    let mut interpolation = [0u8; 64];

    // Ease-in-out control points - using (20, 107) for pronounced curve
    let x1 = 64u8;
    let y1 = 0u8;
    let x2 = 63u8;
    let y2 = 127u8;

    // Block 1: X_x1,Y_x1,phy1,phy2, X_y1,Y_y1,Z_y1,R_y1, X_x2,Y_x2,Z_x2,R_x2, X_y2,Y_y2,Z_y2,R_y2
    interpolation[0] = x1; // X_x1
    interpolation[1] = x1; // Y_x1
    interpolation[2] = 0; // phy1 (physics off)
    interpolation[3] = 0; // phy2 (physics off)
    interpolation[4] = y1; // X_y1
    interpolation[5] = y1; // Y_y1
    interpolation[6] = y1; // Z_y1
    interpolation[7] = y1; // R_y1
    interpolation[8] = x2; // X_x2
    interpolation[9] = x2; // Y_x2
    interpolation[10] = x2; // Z_x2
    interpolation[11] = x2; // R_x2
    interpolation[12] = y2; // X_y2
    interpolation[13] = y2; // Y_y2
    interpolation[14] = y2; // Z_y2
    interpolation[15] = y2; // R_y2

    // Block 2: Y_x1,Z_x1,R_x1,X_y1, Y_y1,Z_y1,R_y1,X_x2, Y_x2,Z_x2,R_x2,X_y2, Y_y2,Z_y2,R_y2,00
    interpolation[16] = x1; // Y_x1
    interpolation[17] = x1; // Z_x1
    interpolation[18] = x1; // R_x1
    interpolation[19] = y1; // X_y1
    interpolation[20] = y1; // Y_y1
    interpolation[21] = y1; // Z_y1
    interpolation[22] = y1; // R_y1
    interpolation[23] = x2; // X_x2
    interpolation[24] = x2; // Y_x2
    interpolation[25] = x2; // Z_x2
    interpolation[26] = x2; // R_x2
    interpolation[27] = y2; // X_y2
    interpolation[28] = y2; // Y_y2
    interpolation[29] = y2; // Z_y2
    interpolation[30] = y2; // R_y2
    interpolation[31] = 0; // padding

    // Block 3: Z_x1,R_x1,X_y1,Y_y1, Z_y1,R_y1,X_x2,Y_x2, Z_x2,R_x2,X_y2,Y_y2, Z_y2,R_y2,00,00
    interpolation[32] = x1; // Z_x1
    interpolation[33] = x1; // R_x1
    interpolation[34] = y1; // X_y1
    interpolation[35] = y1; // Y_y1
    interpolation[36] = y1; // Z_y1
    interpolation[37] = y1; // R_y1
    interpolation[38] = x2; // X_x2
    interpolation[39] = x2; // Y_x2
    interpolation[40] = x2; // Z_x2
    interpolation[41] = x2; // R_x2
    interpolation[42] = y2; // X_y2
    interpolation[43] = y2; // Y_y2
    interpolation[44] = y2; // Z_y2
    interpolation[45] = y2; // R_y2
    interpolation[46] = 0; // padding
    interpolation[47] = 0; // padding

    // Block 4: R_x1,X_y1,Y_y1,Z_y1, R_y1,X_x2,Y_x2,Z_x2, R_x2,X_y2,Y_y2,Z_y2, R_y2,00,00,00
    interpolation[48] = x1; // R_x1
    interpolation[49] = y1; // X_y1
    interpolation[50] = y1; // Y_y1
    interpolation[51] = y1; // Z_y1
    interpolation[52] = y1; // R_y1
    interpolation[53] = x2; // X_x2
    interpolation[54] = x2; // Y_x2
    interpolation[55] = x2; // Z_x2
    interpolation[56] = x2; // R_x2
    interpolation[57] = y2; // X_y2
    interpolation[58] = y2; // Y_y2
    interpolation[59] = y2; // Z_y2
    interpolation[60] = y2; // R_y2
    interpolation[61] = 0; // padding
    interpolation[62] = 0; // padding
    interpolation[63] = 0; // padding

    interpolation
}

#[derive(Debug, Clone)]
pub struct VMDWriter {
    pub key_frames: Vec<MPLKeyFrame>,
    pub ik_disabled_bones: Vec<String>, // Bones to disable IK for
}

impl VMDWriter {
    pub fn new(key_frames: Vec<MPLKeyFrame>) -> Self {
        Self {
            key_frames,
            ik_disabled_bones: vec![
                "右足IK親".to_string(),
                "左足IK親".to_string(),
                "右足ＩＫ".to_string(),
                "左足ＩＫ".to_string(),
                "右つま先ＩＫ".to_string(),
                "左つま先ＩＫ".to_string(),
            ],
        }
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

        // Write interpolation parameters (64 bytes, ease-in-out curve)
        let interpolation = create_ease_in_out_interpolation();
        // let interpolation = [20u8; 64];
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

    /// Write a property key frame with IK flags to the buffer
    fn write_property_key_frame(
        cursor: &mut Cursor<Vec<u8>>,
        frame: u32,
        ik_states: &[(String, bool)], // (bone_name, ik_enabled)
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Write frame number (4 bytes, little endian)
        cursor.write_all(&frame.to_le_bytes())?;

        // Write visibility (1 byte, always visible for now)
        cursor.write_all(&[1u8])?;

        // Write IK state count (4 bytes, little endian)
        cursor.write_all(&(ik_states.len() as u32).to_le_bytes())?;

        // Write each IK state
        for (bone_name, ik_enabled) in ik_states {
            // Write bone name (20 bytes)
            let (name_bytes, _, _) = SHIFT_JIS.encode(bone_name);
            let mut name_buffer = [0u8; 20];
            for (i, &byte) in name_bytes.iter().enumerate() {
                if i < 20 {
                    name_buffer[i] = byte;
                }
            }
            cursor.write_all(&name_buffer)?;

            // Write IK enabled flag (1 byte)
            cursor.write_all(&[if *ik_enabled { 1u8 } else { 0u8 }])?;
        }

        Ok(())
    }

    /// Write VMD file data from recorded frames to bytes
    pub fn write(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
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

        // Calculate property key frame size (if we have IK disabled bones)
        let property_key_frame_count = if !self.ik_disabled_bones.is_empty() {
            1
        } else {
            0
        };
        let property_key_frame_size = 4 + 1 + 4 + (self.ik_disabled_bones.len() * (20 + 1)); // frame + visibility + ik_count + ik_states

        // Calculate sizes
        let header_size = 30 + 20; // Header + model name
        let bone_frame_size = 15 + 4 + 12 + 16 + 64; // 111 bytes per bone frame
        let morph_frame_size = 15 + 4 + 4; // 23 bytes per morph frame
        let total_size = header_size + 4 + // bone frame count
                    (bone_frame_size * total_bone_frames) as usize +
                    4 + // morph frame count  
                    (morph_frame_size * total_morph_frames) as usize +
                    4 + // camera keyframe count
                    4 + // light keyframe count
                    4 + // self shadow keyframe count
                    4 + // property keyframe count
                    (property_key_frame_size * property_key_frame_count) as usize;

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
            let frame_number = (frame.time * FRAME_RATE) as u32;
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
            let frame_number = (frame.time * FRAME_RATE) as u32;
            for morph_frame in &frame.morph_frames {
                Self::write_morph_frame(
                    &mut cursor,
                    &morph_frame.name_jp,
                    frame_number,
                    morph_frame.weight,
                )?;
            }
        }

        // Write counts for other frame types
        cursor.write_all(&0u32.to_le_bytes())?; // Camera keyframe count
        cursor.write_all(&0u32.to_le_bytes())?; // Light keyframe count
        cursor.write_all(&0u32.to_le_bytes())?; // Self shadow keyframe count

        // Write property keyframe count
        cursor.write_all(&property_key_frame_count.to_le_bytes())?;

        // Write property keyframes (for IK flags)
        if !self.ik_disabled_bones.is_empty() {
            let frame_number = 0; // Write at frame 0
            let ik_states: Vec<(String, bool)> = self
                .ik_disabled_bones
                .iter()
                .map(|bone_name| (bone_name.clone(), false))
                .collect();
            Self::write_property_key_frame(&mut cursor, frame_number, &ik_states)?;
        }

        Ok(cursor.into_inner())
    }
}

#[derive(Debug, Clone)]
pub struct VMDReader;

impl VMDReader {
    pub fn new() -> Self {
        Self
    }

    /// Read a bone frame from the buffer
    fn read_bone_frame(
        cursor: &mut Cursor<Vec<u8>>,
    ) -> Result<(String, u32, Vector3, Quaternion), Box<dyn std::error::Error>> {
        // Read bone name (15 bytes)
        let mut name_buffer = [0u8; 15];
        cursor.read_exact(&mut name_buffer)?;

        // Decode Shift-JIS bone name
        let name = SHIFT_JIS.decode(&name_buffer).0;
        let bone_name = name.trim_matches('\0').to_string();

        // Read frame number (4 bytes, little endian)
        let mut frame_buffer = [0u8; 4];
        cursor.read_exact(&mut frame_buffer)?;
        let frame = u32::from_le_bytes(frame_buffer);

        // Read position (12 bytes: 3 x f32, little endian)
        let mut pos_x_buffer = [0u8; 4];
        let mut pos_y_buffer = [0u8; 4];
        let mut pos_z_buffer = [0u8; 4];
        cursor.read_exact(&mut pos_x_buffer)?;
        cursor.read_exact(&mut pos_y_buffer)?;
        cursor.read_exact(&mut pos_z_buffer)?;

        let pos_x = f32::from_le_bytes(pos_x_buffer);
        let pos_y = f32::from_le_bytes(pos_y_buffer);
        let pos_z = f32::from_le_bytes(pos_z_buffer);
        let position = Vector3::new(pos_x, pos_y, pos_z);

        // Read rotation quaternion (16 bytes: 4 x f32, little endian)
        let mut rot_x_buffer = [0u8; 4];
        let mut rot_y_buffer = [0u8; 4];
        let mut rot_z_buffer = [0u8; 4];
        let mut rot_w_buffer = [0u8; 4];
        cursor.read_exact(&mut rot_x_buffer)?;
        cursor.read_exact(&mut rot_y_buffer)?;
        cursor.read_exact(&mut rot_z_buffer)?;
        cursor.read_exact(&mut rot_w_buffer)?;

        let rot_x = f32::from_le_bytes(rot_x_buffer);
        let rot_y = f32::from_le_bytes(rot_y_buffer);
        let rot_z = f32::from_le_bytes(rot_z_buffer);
        let rot_w = f32::from_le_bytes(rot_w_buffer);
        let rotation = Quaternion::new(rot_x, rot_y, rot_z, rot_w);

        // Skip interpolation parameters (64 bytes)
        let mut interpolation_buffer = [0u8; 64];
        cursor.read_exact(&mut interpolation_buffer)?;

        Ok((bone_name, frame, position, rotation))
    }

    /// Read VMD file and extract bone keyframes
    pub fn read(&self, vmd_data: &[u8]) -> Result<Vec<MPLKeyFrame>, Box<dyn std::error::Error>> {
        let mut cursor = Cursor::new(vmd_data.to_vec());

        // Read header (30 bytes)
        let mut header_buffer = [0u8; 30];
        cursor.read_exact(&mut header_buffer)?;
        let header = String::from_utf8_lossy(&header_buffer);
        if !header.starts_with("Vocaloid Motion Data") {
            return Err("Invalid VMD file header".into());
        }

        // Skip model name (20 bytes)
        let mut model_name_buffer = [0u8; 20];
        cursor.read_exact(&mut model_name_buffer)?;

        // Read bone frame count
        let mut bone_count_buffer = [0u8; 4];
        cursor.read_exact(&mut bone_count_buffer)?;
        let bone_frame_count = u32::from_le_bytes(bone_count_buffer);

        // Read all bone frames
        let mut all_bone_frames: Vec<(f32, crate::mpl::MPLBoneFrame)> = Vec::new();

        for _ in 0..bone_frame_count {
            let (bone_name_jp, frame_number, position, rotation) =
                Self::read_bone_frame(&mut cursor)?;

            // Convert frame number to time
            let time = frame_number as f32 / FRAME_RATE;

            // Convert Japanese bone name to English
            let bone_name_en = self.convert_bone_name_jp_to_en(&bone_name_jp);

            let bone_frame =
                crate::mpl::MPLBoneFrame::new(bone_name_en, bone_name_jp, position, rotation);

            all_bone_frames.push((time, bone_frame));
        }

        // Group by time and convert to MPLKeyFrame format
        let mut key_frames = Vec::new();
        all_bone_frames.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut current_time = -1.0;
        let mut current_bone_frames = Vec::new();

        for (time, bone_frame) in all_bone_frames {
            if (time - current_time).abs() > 0.001 {
                // New time frame
                if !current_bone_frames.is_empty() {
                    key_frames.push(MPLKeyFrame::new(current_time, current_bone_frames, vec![]));
                }
                current_time = time;
                current_bone_frames = vec![bone_frame];
            } else {
                // Same time frame
                current_bone_frames.push(bone_frame);
            }
        }

        // Add the last frame
        if !current_bone_frames.is_empty() {
            key_frames.push(MPLKeyFrame::new(current_time, current_bone_frames, vec![]));
        }

        Ok(key_frames)
    }

    /// Convert Japanese bone name to English using the bone database
    fn convert_bone_name_jp_to_en(&self, jp_name: &str) -> String {
        let clean_name = jp_name.trim_matches('\0');

        // Try to find the bone in the database
        if let Some(english_name) = with_bone_db(|db| {
            // Search through all bones to find one with matching Japanese name
            for bone in db.bones() {
                if let Some(jp_bone_name) = db.japanese_name(bone) {
                    if jp_bone_name == clean_name {
                        return Some(bone.to_string());
                    }
                }
            }
            None
        }) {
            return english_name;
        }

        println!("Bone not found in database: {}", clean_name);
        // Fallback to original name if not found in database
        clean_name.to_string()
    }
}
