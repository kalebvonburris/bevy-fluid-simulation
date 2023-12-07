use bevy::{prelude::*, render::render_resource::Extent3d};

// For par_chunks_mut
use rayon::prelude::*;

// For ChunkMapDoubleBuffer and Particle
use crate::particle::*;

pub fn update_density_texture_cpu(
    mut images: ResMut<Assets<Image>>,
    window: Query<&Window>,
    sprite_query: Query<(&mut Sprite, &Handle<Image>)>,
    chunk_map_double_buffer: ResMut<ChunkMapDoubleBuffer>,
) {
    // Grab the image handle
    let (_, image_handle) = sprite_query.single();

    // Grab the texture for the density texture using the image handle
    let texture = images.get_mut(image_handle).unwrap();

    // Get the window
    let window = window.single();

    // Extract the width and height of the window
    let win_width = window.width();
    let win_height = window.height();
    let win_dimensions = &(win_width, win_height);

    // Grab the read chunk map (data stored by the previous frame)
    let chunk_map_read = &chunk_map_double_buffer.as_ref().read_chunk_map;

    // Resize the texture: this ensures that the texture always fits the window
    if texture.size().to_array() != [win_width as u32, win_height as u32] {
        texture.resize(Extent3d {
            width: window.width() as u32,
            height: window.height() as u32,
            depth_or_array_layers: 1,
        });
    }

    // Precompute the midpoints of the window
    let half_win_width = win_width as f32 / 2.0;
    let half_win_height = win_height as f32 / 2.0;

    // Using Rust's chunks() iterator function to operate on an entire pixel at a time (sections of 4 u8s)
    // Directly editing the data also means we save on allocating the memory of the image
    texture.data
        .par_chunks_mut(4)
        .enumerate()
        .for_each(|(index, pixel)| {
            // Calculate the position of the pixel in the simulation
            let x_coord = ((index) as f32 % win_width) - half_win_width;
            let y_coord = -((index / win_width as usize) as f32) + half_win_height;

            // Get the chunks near the pixel
            let nearby_chunks =
                get_nearby_chunks(&(x_coord, y_coord), chunk_map_read, win_dimensions);

            let vec3_coords = Vec3::new(x_coord, y_coord, 0.0);

            // Weight value for storing the denity at this pixel
            let mut weight: f32 = 0.0;

            // Iterate over every close particle
            for chunk_index in nearby_chunks {
                // Read the values of the chunk
                if let Some(chunk) = chunk_map_read.chunks.get(chunk_index) {
                    // Perform weight additive operation on the nearby chunk
                    let chunk_lock = chunk.read().unwrap();
                    for (_, other_pos, _) in chunk_lock.iter() {
                        let distance =
                            (vec3_coords - other_pos.translation).length();
                        if distance < SMOOTHING_RADIUS {
                            let force = calculate_force(
                                vec3_coords,
                                other_pos.translation,
                            )
                            .length();
                            weight += force;
                        }
                    }
                }
            }

            // Assign the weights to the pixel
            // By cascading the weight across multiple color channels we
            // get a gradient without much extra effort
            // Colors go Red -> Orange -> Yellow -> White
            // This means higher density regions are brighter
            pixel[0] = weight.min(255.0) as u8;
            pixel[1] = (weight / 255.0).min(255.0) as u8;
            pixel[2] = (weight / (255.0 * 255.0)).min(255.0) as u8;
            pixel[3] = 255;
        });
}
