use std::f32::consts::PI;

#[derive(Clone)]
pub struct CelestialBody {
    pub name: String,
    pub radius: f32,           // Relative size
    pub orbit_radius: f32,     // Distance from sun (0 for sun)
    pub orbit_speed: f32,      // Radians per frame
    pub current_angle: f32,    // Current position in orbit
    pub color: [f32; 3],
    pub is_sun: bool,
}

impl CelestialBody {
    pub fn new(name: &str, radius: f32, orbit_radius: f32, orbit_speed: f32, color: [f32; 3], is_sun: bool) -> Self {
        Self {
            name: name.to_string(),
            radius,
            orbit_radius,
            orbit_speed,
            current_angle: 0.0,
            color,
            is_sun,
        }
    }
    
    pub fn update(&mut self, delta_time: f32, time_scale: f32) {
        if !self.is_sun {
            self.current_angle += self.orbit_speed * delta_time * time_scale;
            if self.current_angle > 2.0 * PI {
                self.current_angle -= 2.0 * PI;
            }
        }
    }
    
    pub fn get_position(&self) -> [f32; 3] {
        if self.is_sun {
            [0.0, 0.0, 0.0]
        } else {
            let x = self.orbit_radius * self.current_angle.cos();
            let z = self.orbit_radius * self.current_angle.sin();
            [x, 0.0, z]
        }
    }
}

pub struct SolarSystem {
    pub bodies: Vec<CelestialBody>,
    pub time_scale: f32,
}

impl SolarSystem {
    pub fn new() -> Self {
        // Scale factors for better visualization
        let size_scale = 1.0;  // Planet sizes (not to real scale, for visibility)
        let distance_scale = 1.0;  // Orbital distances
        
        let bodies = vec![
            // Sun
            CelestialBody::new("Sun", 0.15, 0.0, 0.0, [1.0, 0.9, 0.0], true),
            
            // Inner planets
            CelestialBody::new("Mercury", 0.03 * size_scale, 0.5 * distance_scale, 0.04, [0.7, 0.7, 0.7], false),
            CelestialBody::new("Venus", 0.06 * size_scale, 0.8 * distance_scale, 0.03, [0.9, 0.8, 0.5], false),
            CelestialBody::new("Earth", 0.06 * size_scale, 1.2 * distance_scale, 0.02, [0.2, 0.5, 0.8], false),
            CelestialBody::new("Mars", 0.04 * size_scale, 1.6 * distance_scale, 0.015, [0.8, 0.4, 0.2], false),
            
            // Outer planets
            CelestialBody::new("Jupiter", 0.12 * size_scale, 2.5 * distance_scale, 0.008, [0.8, 0.7, 0.6], false),
            CelestialBody::new("Saturn", 0.10 * size_scale, 3.5 * distance_scale, 0.006, [0.9, 0.8, 0.6], false),
            CelestialBody::new("Uranus", 0.08 * size_scale, 4.5 * distance_scale, 0.004, [0.5, 0.8, 0.9], false),
            CelestialBody::new("Neptune", 0.08 * size_scale, 5.5 * distance_scale, 0.003, [0.3, 0.5, 0.9], false),
        ];
        
        Self {
            bodies,
            time_scale: 1.0,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        for body in &mut self.bodies {
            body.update(delta_time, self.time_scale);
        }
    }
    
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale;
    }
    
    pub fn get_body(&self, index: usize) -> Option<&CelestialBody> {
        self.bodies.get(index)
    }
    
    pub fn get_body_mut(&mut self, index: usize) -> Option<&mut CelestialBody> {
        self.bodies.get_mut(index)
    }
}