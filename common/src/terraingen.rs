use crate::world::Material;

pub struct VoronoiInfo {
    pub location: na::Vector2<f64>,
    pub material: Material,
}
impl VoronoiInfo {
    pub fn new(mat: Material, /*elev: f64,*/ rain: f64, temp: f64) -> VoronoiInfo {
        VoronoiInfo {
            location: na::Vector2::new(rain, temp),
            material: mat,
        }
    }

    pub fn terraingen_voronoi(elev: f64, y: na::Vector2<f64>) -> Material {
        const NUM_CHOICES_HIGH: usize = 17;
        let voronoi_choices_high: [VoronoiInfo; NUM_CHOICES_HIGH] = [
            VoronoiInfo::new(Material::Grass, 6.1, 1.3),
            VoronoiInfo::new(Material::Ice, 5.5, -6.1),
            VoronoiInfo::new(Material::Grass, 5.1, 2.5),
            VoronoiInfo::new(Material::Stone, 4.9, -2.5),
            VoronoiInfo::new(Material::Dirt, 4.7, 0.3),
            VoronoiInfo::new(Material::Grass, 3.5, 4.1),
            VoronoiInfo::new(Material::Dirt, 3.3, -2.1),
            VoronoiInfo::new(Material::Ice, 2.3, -7.5),
            VoronoiInfo::new(Material::Dirt, 2.1, 1.7),
            VoronoiInfo::new(Material::Stone, 1.7, -4.7),
            VoronoiInfo::new(Material::Dirt, 0.5, 0.7),
            VoronoiInfo::new(Material::Dirt, -0.9, 3.1),
            VoronoiInfo::new(Material::Stone, -1.1, -5.3),
            VoronoiInfo::new(Material::Dirt, -1.1, -3.3),
            VoronoiInfo::new(Material::Dirt, -1.7, -0.7),
            VoronoiInfo::new(Material::Stone, -3.1, 2.5),
            VoronoiInfo::new(Material::Stone, -3.3, -2.1),
        ];

        const NUM_CHOICES_MED: usize = 62;
        let voronoi_choices_med: [VoronoiInfo; NUM_CHOICES_MED] = [
            VoronoiInfo::new(Material::Grass, 7.5, 1.3),
            VoronoiInfo::new(Material::Dirt, 6.9, -1.5),
            VoronoiInfo::new(Material::Bigflowergrass, 6.9, 4.3),
            VoronoiInfo::new(Material::Flowergrass, 6.9, 7.3),
            VoronoiInfo::new(Material::Dirt, 6.5, -2.9),
            VoronoiInfo::new(Material::Snow, 5.7, -4.5),
            VoronoiInfo::new(Material::Bigflowergrass, 5.7, 2.7),
            VoronoiInfo::new(Material::Dirt, 5.5, -2.7),
            VoronoiInfo::new(Material::Snow, 5.3, -6.9),
            VoronoiInfo::new(Material::Dirt, 4.9, -1.9),
            VoronoiInfo::new(Material::Flowergrass, 4.9, 4.3),
            VoronoiInfo::new(Material::Bigflowergrass, 4.7, 5.9),
            VoronoiInfo::new(Material::Snow, 4.1, -5.5),
            VoronoiInfo::new(Material::Grass, 4.1, -0.5),
            VoronoiInfo::new(Material::Snow, 3.9, -3.9),
            VoronoiInfo::new(Material::Grass, 3.1, 2.5),
            VoronoiInfo::new(Material::Flowergrass, 3.1, 7.5),
            VoronoiInfo::new(Material::Snow, 2.9, -7.7),
            VoronoiInfo::new(Material::Grass, 2.9, -2.1),
            VoronoiInfo::new(Material::Mud, 2.1, -3.5),
            VoronoiInfo::new(Material::Grass, 2.1, 0.7),
            VoronoiInfo::new(Material::Gravelstone, 1.9, -6.1),
            VoronoiInfo::new(Material::Flowergrass, 1.9, 5.1),
            VoronoiInfo::new(Material::Stone, 1.5, -7.3),
            VoronoiInfo::new(Material::Mud, 1.1, -4.7),
            VoronoiInfo::new(Material::Sand, 0.7, 7.3),
            VoronoiInfo::new(Material::Stone, 0.5, -6.1),
            VoronoiInfo::new(Material::Dirt, 0.5, -0.9),
            VoronoiInfo::new(Material::Dirt, 0.3, -2.3),
            VoronoiInfo::new(Material::Grass, 0.1, 2.1),
            VoronoiInfo::new(Material::Stone, -0.1, -7.9),
            VoronoiInfo::new(Material::Mud, -0.1, -3.5),
            VoronoiInfo::new(Material::Grass, -0.3, 3.9),
            VoronoiInfo::new(Material::Dirt, -0.5, -1.5),
            VoronoiInfo::new(Material::Graveldirt, -0.9, -5.7),
            VoronoiInfo::new(Material::Sand, -0.9, 6.3),
            VoronoiInfo::new(Material::Gravelstone, -1.1, -7.5),
            VoronoiInfo::new(Material::Dirt, -1.1, -2.5),
            VoronoiInfo::new(Material::Grass, -1.3, 1.7),
            VoronoiInfo::new(Material::Grass, -1.7, 0.1),
            VoronoiInfo::new(Material::Sand, -1.9, 4.5),
            VoronoiInfo::new(Material::Graveldirt, -2.1, -4.3),
            VoronoiInfo::new(Material::Dirt, -2.3, -1.5),
            VoronoiInfo::new(Material::Grass, -2.3, 3.1),
            VoronoiInfo::new(Material::Graveldirt, -2.5, -3.3),
            VoronoiInfo::new(Material::Grass, -3.3, 0.7),
            VoronoiInfo::new(Material::Greystone, -3.5, -7.9),
            VoronoiInfo::new(Material::Gravelstone, -3.5, -6.1),
            VoronoiInfo::new(Material::Sand, -3.9, 3.5),
            VoronoiInfo::new(Material::Graveldirt, -4.3, -3.7),
            VoronoiInfo::new(Material::Dirt, -4.3, -1.9),
            VoronoiInfo::new(Material::Redsand, -5.1, 5.3),
            VoronoiInfo::new(Material::Greystone, -5.5, -7.1),
            VoronoiInfo::new(Material::Gravelstone, -5.7, -5.3),
            VoronoiInfo::new(Material::Redsand, -5.9, 7.3),
            VoronoiInfo::new(Material::Redsand, -6.3, 2.1),
            VoronoiInfo::new(Material::Redsand, -6.5, 5.1),
            VoronoiInfo::new(Material::Redstone, -6.9, 3.9),
            VoronoiInfo::new(Material::Redstone, -7.1, 6.3),
            VoronoiInfo::new(Material::Redstone, -7.5, 2.9),
            VoronoiInfo::new(Material::Redstone, -7.7, 4.9),
            VoronoiInfo::new(Material::Redstone, -7.9, 7.1),
        ];

        const NUM_CHOICES_LOW: usize = 24;
        let voronoi_choices_low: [VoronoiInfo; NUM_CHOICES_LOW] = [
            VoronoiInfo::new(Material::Gravelstone, 5.9, -2.9),
            VoronoiInfo::new(Material::GreySand, 5.9, 1.5),
            VoronoiInfo::new(Material::GreySand, 5.5, 5.9),
            VoronoiInfo::new(Material::Gravelstone, 4.5, -6.9),
            VoronoiInfo::new(Material::Gravelstone, 4.1, -0.3),
            VoronoiInfo::new(Material::Gravelstone, 3.3, 4.1),
            VoronoiInfo::new(Material::Gravelstone, 2.3, -5.1),
            VoronoiInfo::new(Material::GreySand, 2.1, 6.7),
            VoronoiInfo::new(Material::Stone, 1.5, 1.5),
            VoronoiInfo::new(Material::Stone, 1.1, -3.3),
            VoronoiInfo::new(Material::Stone, 1.1, 4.9),
            VoronoiInfo::new(Material::GreySand, -0.1, 6.7),
            VoronoiInfo::new(Material::Stone, -0.3, -6.9),
            VoronoiInfo::new(Material::GreySand, -0.3, 3.9),
            VoronoiInfo::new(Material::Greystone, -1.7, -0.9),
            VoronoiInfo::new(Material::Redstone, -2.1, 5.5),
            VoronoiInfo::new(Material::Blackstone, -2.3, -5.1),
            VoronoiInfo::new(Material::Greystone, -2.3, 2.5),
            VoronoiInfo::new(Material::Greystone, -3.9, -2.9),
            VoronoiInfo::new(Material::Redstone, -4.5, 3.7),
            VoronoiInfo::new(Material::Redstone, -4.7, 5.9),
            VoronoiInfo::new(Material::Blackstone, -5.1, -6.9),
            VoronoiInfo::new(Material::Blackstone, -5.7, -4.7),
            VoronoiInfo::new(Material::Greystone, -6.3, -0.5),
        ];

        const NUM_CHOICES_DEEP: usize = 6;
        let voronoi_choices_deep: [VoronoiInfo; NUM_CHOICES_DEEP] = [
            VoronoiInfo::new(Material::Blackstone, 6.9, -1.3),
            VoronoiInfo::new(Material::Water, 6.7, -6.5),
            VoronoiInfo::new(Material::Blackstone, 1.7, 6.3),
            VoronoiInfo::new(Material::Blackstone, 1.1, -6.9),
            VoronoiInfo::new(Material::Blackstone, -4.3, -1.7),
            VoronoiInfo::new(Material::Lava, -4.3, 4.1),
        ];

        let mut voxel_mat: Material;

        let num_choices;
        if elev < -40.0 {
            let voronoi_choices = &voronoi_choices_deep;
            num_choices = &NUM_CHOICES_DEEP;

            let mut dist = na::norm(&(&voronoi_choices[0].location - &y));
            voxel_mat = voronoi_choices[0].material;
            for i in 1..*num_choices {
                let d = na::norm(&(&voronoi_choices[i].location - &y));
                if d <= dist {
                    dist = d;
                    voxel_mat = voronoi_choices[i].material;
                };
            }
        } else if elev < -15.0 {
            let voronoi_choices = &voronoi_choices_low;
            num_choices = &NUM_CHOICES_LOW;

            let mut dist = na::norm(&(&voronoi_choices[0].location - &y));
            voxel_mat = voronoi_choices[0].material;
            for i in 1..*num_choices {
                let d = na::norm(&(&voronoi_choices[i].location - &y));
                if d <= dist {
                    dist = d;
                    voxel_mat = voronoi_choices[i].material;
                };
            }
        } else if elev < 5.0 {
            let voronoi_choices = &voronoi_choices_med;
            num_choices = &NUM_CHOICES_MED;

            let mut dist = na::norm(&(&voronoi_choices[0].location - &y));
            voxel_mat = voronoi_choices[0].material;
            for i in 1..*num_choices {
                let d = na::norm(&(&voronoi_choices[i].location - &y));
                if d <= dist {
                    dist = d;
                    voxel_mat = voronoi_choices[i].material;
                };
            }
        } else {
            let voronoi_choices = &voronoi_choices_high;
            num_choices = &NUM_CHOICES_HIGH;

            let mut dist = na::norm(&(&voronoi_choices[0].location - &y));
            voxel_mat = voronoi_choices[0].material;
            for i in 1..*num_choices {
                let d = na::norm(&(&voronoi_choices[i].location - &y));
                if d <= dist {
                    dist = d;
                    voxel_mat = voronoi_choices[i].material;
                };
            }
        }

        voxel_mat
    }
}
