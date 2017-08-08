use cgmath::{Vector3, Vector2};

pub struct BoundingBox {
	box_min: Vector3<f64>,
	box_max: Vector3<f64>,
}

pub struct Ray {
	// TODO: rename once we know what these are
	p: Vector3<f64>,
	d: Vector3<f64>,
	atten: Vector3<f64>,
	ctr: u32,
	ray_type: RayType,
}

pub enum RayType {
	Visibility,
	Reflection,
	Refraction,
	Shadow,
}

pub struct Intersect {
	// TODO: rename once we know what these are
	// NOTE: capital N in cpp raytracer
	n: Vector3<f64>,
	bary_coords: Vector3<f64>,
	uv_coords: Vector2<f64>,
	t: f64,

	//TODO: Object pointer, material

}

pub struct Light {
	light_type: LightType,
	color: Vector3<f64>,
}

pub enum LightType {
	DirectionalLight { orientation: Vector3<f64> },
	PointLight { pos: Vector3<f64>, a: f64, b: f64, c: f64 }, // pos, a, b, c
}