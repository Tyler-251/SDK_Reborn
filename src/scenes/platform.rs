use bevy::prelude::*;

#[derive(Component)]
pub struct Platform {
    platform_type: PlatformType
}

pub enum PlatformType {
    Solid,
    Passthrough
}

impl Platform {
    pub const SOLID: Self = {
        Self {
            platform_type: PlatformType::Solid
        }
    };
    pub const PASSTHROUGH: Self = {
        Self {
            platform_type: PlatformType::Passthrough
        }
    };
    pub fn get_type (&self) -> &PlatformType {
        return &self.platform_type;
    }
}