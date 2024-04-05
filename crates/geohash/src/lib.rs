use std::collections::LinkedList;

#[derive(Copy, Clone, Debug)]
pub struct BoundingBox {
    min_lat: f64,
    min_lng: f64,
    max_lat: f64,
    max_lng: f64,
}

impl BoundingBox {
    fn intersects(&self, b_box2: &Self) -> bool {
        if self.min_lat >= b_box2.max_lat
            || self.max_lat <= b_box2.min_lat
            || self.min_lng >= b_box2.max_lng
            || self.max_lng <= b_box2.min_lng
        {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
struct GeoHashBBox {
    geo_hash: u64,
    b_box: BoundingBox,
}

pub fn geo_hashes_for_bounding_box(
    world_b_box: BoundingBox,
    b_box: BoundingBox,
    zoom_level: u32,
) -> Vec<u64> {
    let mut queue: LinkedList<GeoHashBBox> = LinkedList::new();
    queue.push_back(GeoHashBBox {
        geo_hash: 1,
        b_box: world_b_box,
    });
    for _ in 0..zoom_level {
        let mut temp_queue: LinkedList<GeoHashBBox> = LinkedList::new();
        while let Some(front) = queue.pop_front() {
            let mut next_level_geo_hash_bbox: LinkedList<GeoHashBBox> =
                front.next_level_geo_hash_bbox();
            while let Some(geo_hash_bbox) = next_level_geo_hash_bbox.pop_front() {
                if geo_hash_bbox.b_box.intersects(&b_box) {
                    temp_queue.push_back(geo_hash_bbox)
                }
            }
        }
        queue.append(&mut temp_queue);
    }
    queue.iter().map(|g| g.geo_hash).collect::<Vec<u64>>()
}

impl GeoHashBBox {
    const BOTTOM_LEFT_QUAD: u64 = 0b00;
    const BOTTOM_RIGHT_QUAD: u64 = 0b10;
    const TOP_RIGHT_QUAD: u64 = 0b11;
    const TOP_LEFT_QUAD: u64 = 0b01;

    fn next_level_geo_hash_bbox(&self) -> LinkedList<Self> {
        let mut list: LinkedList<GeoHashBBox> = LinkedList::new();
        list.push_back(GeoHashBBox {
            geo_hash: self.next_bottom_left_geo_hash(),
            b_box: self.next_bottom_left_bounding_box(),
        });
        list.push_back(GeoHashBBox {
            geo_hash: self.next_top_right_geo_hash(),
            b_box: self.next_top_right_bounding_box(),
        });
        list.push_back(GeoHashBBox {
            geo_hash: self.next_bottom_right_geo_hash(),
            b_box: self.next_bottom_right_bounding_box(),
        });
        list.push_back(GeoHashBBox {
            geo_hash: self.next_top_left_geo_hash(),
            b_box: self.next_top_left_bounding_box(),
        });

        list
    }

    fn next_bottom_left_geo_hash(&self) -> u64 {
        self.geo_hash << 2 | Self::BOTTOM_LEFT_QUAD
    }

    fn next_bottom_right_geo_hash(&self) -> u64 {
        self.geo_hash << 2 | Self::BOTTOM_RIGHT_QUAD
    }

    fn next_top_left_geo_hash(&self) -> u64 {
        self.geo_hash << 2 | Self::TOP_LEFT_QUAD
    }

    fn next_top_right_geo_hash(&self) -> u64 {
        self.geo_hash << 2 | Self::TOP_RIGHT_QUAD
    }

    fn next_bottom_left_bounding_box(&self) -> BoundingBox {
        let mid_lat = (self.b_box.min_lat + self.b_box.max_lat) / 2.0;
        let mid_lng = (self.b_box.min_lng + self.b_box.max_lng) / 2.0;
        BoundingBox {
            min_lat: self.b_box.min_lat,
            min_lng: self.b_box.min_lng,
            max_lat: mid_lat,
            max_lng: mid_lng,
        }
    }

    fn next_bottom_right_bounding_box(&self) -> BoundingBox {
        let mid_lat = (self.b_box.min_lat + self.b_box.max_lat) / 2.0;
        let mid_lng = (self.b_box.min_lng + self.b_box.max_lng) / 2.0;
        BoundingBox {
            min_lat: self.b_box.min_lat,
            min_lng: mid_lng,
            max_lat: mid_lat,
            max_lng: self.b_box.max_lng,
        }
    }

    fn next_top_left_bounding_box(&self) -> BoundingBox {
        let mid_lat = (self.b_box.min_lat + self.b_box.max_lat) / 2.0;
        let mid_lng = (self.b_box.min_lng + self.b_box.max_lng) / 2.0;
        BoundingBox {
            min_lat: mid_lat,
            min_lng: self.b_box.min_lng,
            max_lat: self.b_box.max_lat,
            max_lng: mid_lng,
        }
    }

    fn next_top_right_bounding_box(&self) -> BoundingBox {
        let mid_lat = (self.b_box.min_lat + self.b_box.max_lat) / 2.0;
        let mid_lng = (self.b_box.min_lng + self.b_box.max_lng) / 2.0;
        BoundingBox {
            min_lat: mid_lat,
            min_lng: mid_lng,
            max_lat: self.b_box.max_lat,
            max_lng: self.b_box.max_lng,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bounding_box_intersection() {
        let b_box1 = BoundingBox {
            min_lat: 0.0,
            min_lng: 0.0,
            max_lat: 2.0,
            max_lng: 2.0,
        };
        let b_box2 = BoundingBox {
            min_lat: 2.1,
            min_lng: 2.1,
            max_lat: 3.3,
            max_lng: 3.3,
        };
        assert_eq!(false, b_box1.intersects(&b_box2));
        let b_box2 = BoundingBox {
            min_lat: 0.0,
            min_lng: 2.000001,
            max_lat: 2.0,
            max_lng: 3.0,
        };
        assert_eq!(false, b_box1.intersects(&b_box2));
        let b_box2 = BoundingBox {
            min_lat: 2.000001,
            min_lng: 0.0,
            max_lat: 3.0,
            max_lng: 3.0,
        };
        assert_eq!(false, b_box1.intersects(&b_box2));
        let b_box2 = BoundingBox {
            min_lat: -0.000001,
            min_lng: 0.0,
            max_lat: -2.0,
            max_lng: 2.0,
        };
        assert_eq!(false, b_box1.intersects(&b_box2));
        let b_box2 = BoundingBox {
            min_lat: 0.0,
            min_lng: -4.0,
            max_lat: 2.0,
            max_lng: -2.000001,
        };
        assert_eq!(false, b_box1.intersects(&b_box2));

        let b_box2 = BoundingBox {
            min_lat: 1.5,
            min_lng: 1.5,
            max_lat: 1.8,
            max_lng: 1.8,
        };
        assert_eq!(true, b_box1.intersects(&b_box2));
        let b_box2 = BoundingBox {
            min_lat: 1.5,
            min_lng: 1.5,
            max_lat: 2.5,
            max_lng: 2.5,
        };
        assert_eq!(true, b_box1.intersects(&b_box2));
    }

    #[test]
    fn test_geo_hashes_for_bounding_box_zoom_level_2() {
        let world_b_box = BoundingBox {
            min_lat: -90.0,
            min_lng: -180.0,
            max_lat: 90.0,
            max_lng: 180.0,
        };
        let b_box = BoundingBox {
            min_lat: 1.0,
            min_lng: 1.0,
            max_lat: 3.0,
            max_lng: 3.0,
        };
        let geo_hashes = geo_hashes_for_bounding_box(world_b_box, b_box, 2);
        assert_eq!(1, geo_hashes.len());
        assert_eq!("11100", format!("{:b}", geo_hashes[0]));
    }

    #[test]
    fn test_geo_hashes_for_bounding_box_zoom_level_15() {
        let world_b_box = BoundingBox {
            min_lat: -90.0,
            min_lng: -180.0,
            max_lat: 90.0,
            max_lng: 180.0,
        };
        let b_box = BoundingBox {
            min_lat: 12.924052,
            min_lng: 77.669285,
            max_lat: 12.928086,
            max_lng: 77.673468,
        };
        let geo_hashes = geo_hashes_for_bounding_box(world_b_box, b_box, 15);
        assert_eq!(4, geo_hashes.len());
        assert_eq!(
            "1110010110010111000011110100011",
            format!("{:b}", geo_hashes[0])
        );
        assert_eq!(
            "1110010110010111000011110100010",
            format!("{:b}", geo_hashes[1])
        );
        assert_eq!(
            "1110010110010111000011110101000",
            format!("{:b}", geo_hashes[2])
        );
        assert_eq!(
            "1110010110010111000011110101001",
            format!("{:b}", geo_hashes[3])
        );
    }

    #[test]
    fn test_geo_hash_for_bounding_box_level_3() {
        let world_b_box = BoundingBox {
            min_lat: -90.0,
            min_lng: -180.0,
            max_lat: 90.0,
            max_lng: 180.0,
        };
        let b_box = BoundingBox {
            min_lat: 44.0,
            min_lng: 46.0,
            max_lat: 50.0,
            max_lng: 50.0,
        };
        let geo_hashes = geo_hashes_for_bounding_box(world_b_box, b_box, 3);
        assert_eq!(2, geo_hashes.len());
        assert_eq!("1110011", format!("{:b}", geo_hashes[0]));
        assert_eq!("1110110", format!("{:b}", geo_hashes[1]));
    }
}
