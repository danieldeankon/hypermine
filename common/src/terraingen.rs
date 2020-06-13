use crate::world::Material;

pub struct VoronoiInfo {
    pub location: na::Vector3<f64>,
    pub material: Material,
}
impl VoronoiInfo {
    pub fn new(mat: Material, elev: f64, rain: f64, temp: f64) -> VoronoiInfo {
        VoronoiInfo {
            location: na::Vector3::new(elev, rain, temp),
            material: mat,
        }
    }

    pub fn terraingen_voronoi(y: na::Vector3<f64>) -> Material {
        const NUM_CHOICES: usize = 103;
        let voronoi_choices: [VoronoiInfo; NUM_CHOICES] = [
            VoronoiInfo::new(Material::Water, -10.3, 7.3, -6.9),
            VoronoiInfo::new(Material::Mud, -1.1, 7.3, -0.9),
            VoronoiInfo::new(Material::Stone, 7.3, 6.9, 4.5),
            VoronoiInfo::new(Material::GreySand, -3.0, 6.7, -5.5),
            VoronoiInfo::new(Material::Ice, 8.1, 6.7, -0.3),
            VoronoiInfo::new(Material::Mud, -1.1, 6.7, 1.7),
            VoronoiInfo::new(Material::Gravelstone, -3.4, 6.3, -1.5),
            VoronoiInfo::new(Material::Gravelstone, -3.4, 6.3, 3.7),
            VoronoiInfo::new(Material::Grass, -0.1, 6.3, 6.5),
            VoronoiInfo::new(Material::Water, -10.3, 6.1, -3.5),
            VoronoiInfo::new(Material::Flowergrass, 0.4, 6.1, 4.9),
            VoronoiInfo::new(Material::Ice, 8.1, 5.7, -7.5),
            VoronoiInfo::new(Material::Flowergrass, 0.4, 5.7, 2.3),
            VoronoiInfo::new(Material::Ice, 8.1, 5.5, -4.3),
            VoronoiInfo::new(Material::Gravelstone, -3.4, 5.3, -6.3),
            VoronoiInfo::new(Material::Graveldirt, 0.7, 5.3, -0.9),
            VoronoiInfo::new(Material::Grass, -0.1, 4.9, 3.7),
            VoronoiInfo::new(Material::Blackstone, -9.0, 4.9, 5.7),
            VoronoiInfo::new(Material::Graveldirt, 0.7, 4.3, -1.7),
            VoronoiInfo::new(Material::Water, -10.3, 4.3, -0.1),
            VoronoiInfo::new(Material::Stone, 7.3, 4.3, 1.9),
            VoronoiInfo::new(Material::Redsand, -0.3, 4.1, 6.5),
            VoronoiInfo::new(Material::Redstone, -1.8, 3.7, 7.9),
            VoronoiInfo::new(Material::Mud, -1.1, 3.5, 1.1),
            VoronoiInfo::new(Material::Grass, 6.7, 3.5, 4.3),
            VoronoiInfo::new(Material::Gravelstone, -0.2, 3.3, -2.9),
            VoronoiInfo::new(Material::Redstone, -1.8, 3.3, 5.1),
            VoronoiInfo::new(Material::Blackstone, -9.0, 2.9, -0.7),
            VoronoiInfo::new(Material::Sand, -1.2, 2.9, 5.9),
            VoronoiInfo::new(Material::Stone, 7.3, 2.9, 7.3),
            VoronoiInfo::new(Material::Ice, 8.1, 2.7, -6.9),
            VoronoiInfo::new(Material::Ice, 8.1, 2.5, -2.9),
            VoronoiInfo::new(Material::Mud, -1.1, 2.5, 1.3),
            VoronoiInfo::new(Material::GreySand, -3.0, 2.3, -5.1),
            VoronoiInfo::new(Material::Redsand, -0.3, 2.3, 7.9),
            VoronoiInfo::new(Material::Stone, 7.3, 2.1, 4.7),
            VoronoiInfo::new(Material::Grass, 6.7, 1.9, 2.5),
            VoronoiInfo::new(Material::Water, -10.3, 1.7, -6.5),
            VoronoiInfo::new(Material::Mud, -1.1, 1.7, 0.3),
            VoronoiInfo::new(Material::Snow, 2.0, 1.3, -7.3),
            VoronoiInfo::new(Material::Grass, -0.1, 1.3, 3.3),
            VoronoiInfo::new(Material::Bigflowergrass, 0.8, 1.3, 7.3),
            VoronoiInfo::new(Material::Greystone, -0.4, 1.1, -3.5),
            VoronoiInfo::new(Material::Lava, -10.0, 1.1, 4.3),
            VoronoiInfo::new(Material::Stone, -2.9, 0.7, -0.7),
            VoronoiInfo::new(Material::Stone, -2.9, 0.3, 1.1),
            VoronoiInfo::new(Material::Flowergrass, 0.4, 0.1, 5.1),
            VoronoiInfo::new(Material::Ice, 8.1, -0.1, -7.9),
            VoronoiInfo::new(Material::Snow, 2.0, -0.1, -5.7),
            VoronoiInfo::new(Material::Dirt, 0.1, -0.1, 0.1),
            VoronoiInfo::new(Material::Greystone, -3.8, -0.3, -6.7),
            VoronoiInfo::new(Material::Greystone, -0.4, -0.3, -1.9),
            VoronoiInfo::new(Material::Stone, -2.9, -0.3, 4.3),
            VoronoiInfo::new(Material::Grass, -0.1, -0.5, 2.3),
            VoronoiInfo::new(Material::Dirt, 7.5, -0.9, -2.9),
            VoronoiInfo::new(Material::Dirt, 0.1, -0.9, -0.7),
            VoronoiInfo::new(Material::Dirt, 0.1, -1.1, 0.7),
            VoronoiInfo::new(Material::Blackstone, -9.0, -1.1, 6.1),
            VoronoiInfo::new(Material::Greystone, -3.8, -1.7, -5.5),
            VoronoiInfo::new(Material::Grass, -0.1, -1.7, 3.5),
            VoronoiInfo::new(Material::Snow, 2.0, -1.9, -7.3),
            VoronoiInfo::new(Material::Greystone, -0.4, -1.9, -4.1),
            VoronoiInfo::new(Material::Stone, -2.9, -1.9, 0.3),
            VoronoiInfo::new(Material::Sand, -1.2, -2.5, 7.5),
            VoronoiInfo::new(Material::Gravelstone, -0.2, -2.7, -2.1),
            VoronoiInfo::new(Material::Dirt, 0.1, -2.7, 5.1),
            VoronoiInfo::new(Material::Dirt, 7.5, -3.1, -6.9),
            VoronoiInfo::new(Material::Gravelstone, -0.2, -3.1, 2.1),
            VoronoiInfo::new(Material::Water, -10.3, -3.3, -5.9),
            VoronoiInfo::new(Material::Stone, 7.3, -3.3, 6.5),
            VoronoiInfo::new(Material::Gravelstone, -0.2, -3.5, -4.1),
            VoronoiInfo::new(Material::Gravelstone, -0.2, -3.7, -0.1),
            VoronoiInfo::new(Material::Dirt, 7.5, -3.9, -1.9),
            VoronoiInfo::new(Material::Lava, -10.0, -3.9, 5.5),
            VoronoiInfo::new(Material::Lava, -10.0, -4.3, 3.7),
            VoronoiInfo::new(Material::Redsand, -0.3, -4.3, 7.7),
            VoronoiInfo::new(Material::Dirt, 7.5, -4.7, -5.5),
            VoronoiInfo::new(Material::Greystone, -0.4, -4.7, 2.1),
            VoronoiInfo::new(Material::GreySand, -3.0, -4.9, -6.9),
            VoronoiInfo::new(Material::Greystone, -3.8, -4.9, -4.5),
            VoronoiInfo::new(Material::Greystone, -0.4, -5.1, -1.1),
            VoronoiInfo::new(Material::Redstone, -2.4, -5.1, 6.1),
            VoronoiInfo::new(Material::Stone, -2.9, -5.5, -1.9),
            VoronoiInfo::new(Material::Stone, -2.9, -5.5, 1.5),
            VoronoiInfo::new(Material::Greystone, -0.4, -5.5, 4.5),
            VoronoiInfo::new(Material::Redstone, -1.8, -5.5, 7.9),
            VoronoiInfo::new(Material::Greystone, -3.8, -6.1, -7.5),
            VoronoiInfo::new(Material::Redsand, -0.3, -6.1, 6.7),
            VoronoiInfo::new(Material::Lava, -10.0, -6.3, 0.1),
            VoronoiInfo::new(Material::GreySand, -3.0, -6.5, -4.5),
            VoronoiInfo::new(Material::Blackstone, -5.0, -6.5, 2.7),
            VoronoiInfo::new(Material::Stone, -2.9, -6.7, -1.7),
            VoronoiInfo::new(Material::Stone, -2.9, -6.9, -5.9),
            VoronoiInfo::new(Material::Sand, -1.2, -6.9, 3.9),
            VoronoiInfo::new(Material::Redstone, -2.4, -6.9, 7.7),
            VoronoiInfo::new(Material::Sand, -1.2, -7.1, 4.9),
            VoronoiInfo::new(Material::Blackstone, -9.0, -7.5, -3.5),
            VoronoiInfo::new(Material::Blackstone, -9.0, -7.5, 2.9),
            VoronoiInfo::new(Material::Redstone, -1.8, -7.5, 6.3),
            VoronoiInfo::new(Material::Lava, -10.0, -7.5, 7.9),
            VoronoiInfo::new(Material::Stone, -2.9, -7.7, 0.9),
            VoronoiInfo::new(Material::Blackstone, -5.0, -7.7, 4.9),
            VoronoiInfo::new(Material::Blackstone, -5.0, -8.1, -0.1),
        ];

        let mut voxel_mat: Material;

        let mut dist = na::norm(&(&voronoi_choices[0].location - &y));
        voxel_mat = voronoi_choices[0].material;
        for i in 1..NUM_CHOICES {
            let d = na::norm(&(&voronoi_choices[i].location - &y));
            if d <= dist {
                dist = d;
                voxel_mat = voronoi_choices[i].material;
            };
        }

        voxel_mat
    }
}
