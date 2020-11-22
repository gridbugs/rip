use crate::world::{data::Tile, World};
use entity_table::Entity;
use grid_2d::Coord;
use line_2d::LineSegment;

impl World {
    pub fn is_solid_feature_at_coord(&self, coord: Coord) -> bool {
        let cell = self.spatial.get_cell_checked(coord);
        if let Some(feature) = cell.feature {
            self.components.solid.contains(feature)
        } else {
            false
        }
    }

    pub fn is_solid_feature_in_line_segment(&self, line_segment: LineSegment) -> bool {
        for coord in line_segment.iter() {
            if self.is_solid_feature_at_coord(coord) {
                return true;
            }
        }
        false
    }

    pub fn is_wall_at_coord(&self, coord: Coord) -> bool {
        if let Some(spatial_cell) = self.spatial.get_cell(coord) {
            if let Some(entity) = spatial_cell.feature {
                self.components.tile.get(entity) == Some(&Tile::Wall)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn can_npc_traverse_feature_at_coord(&self, coord: Coord) -> bool {
        if let Some(spatial_cell) = self.spatial.get_cell(coord) {
            if let Some(feature) = spatial_cell.feature {
                self.components.door_state.contains(feature) || !self.components.solid.contains(feature)
            } else {
                true
            }
        } else {
            true
        }
    }

    pub fn is_npc_at_coord(&self, coord: Coord) -> bool {
        if let Some(spatial_cell) = self.spatial.get_cell(coord) {
            if let Some(entity) = spatial_cell.character {
                self.components.npc.contains(entity)
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn is_character_at_coord(&self, coord: Coord) -> bool {
        if let Some(spatial_cell) = self.spatial.get_cell(coord) {
            spatial_cell.character.is_some()
        } else {
            false
        }
    }

    pub fn get_opacity_at_coord(&self, coord: Coord) -> u8 {
        self.spatial
            .get_cell(coord)
            .and_then(|c| c.feature)
            .and_then(|e| self.components.opacity.get(e).cloned())
            .unwrap_or(0)
    }

    pub fn get_character_at_coord(&self, coord: Coord) -> Option<Entity> {
        self.spatial.get_cell(coord).and_then(|cell| cell.character)
    }

    pub fn get_stairs_at_coord(&self, coord: Coord) -> Option<Entity> {
        self.spatial
            .get_cell(coord)
            .and_then(|cell| cell.feature)
            .and_then(|feature| {
                if self.components.stairs.contains(feature) {
                    Some(feature)
                } else {
                    None
                }
            })
    }
}
