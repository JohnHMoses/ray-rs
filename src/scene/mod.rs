pub mod objects;

use cgmath::{Matrix4, Matrix3, Vector3, SquareMatrix, Matrix, Zero};

use std::rc::Weak;

use self::objects::*;

pub struct Scene {
	transform_root: TransformNode,
	objects: Vec<Box<Geometry>>,
	lights: Vec<Light>,
	camera: Camera,
	// TODO: KdTree
	// TODO: texture map
	scene_bounds: Option<BoundingBox>,

}

impl Scene {
	pub fn new() -> Scene {
		Scene {
			transform_root: TransformNode::new(None, Matrix4::from_value(1.0)),
			objects: Vec::new(),
			lights: Vec::new(),
			camera: Camera::new(),
			scene_bounds: None,
		}
	}
}

struct TransformNode {
	parent: Option<Weak<TransformNode>>,
	children: Vec<TransformNode>,
	// TODO: better names once we know what these are
	xform: Matrix4<f64>,
	inverse: Matrix4<f64>,
	normi: Matrix3<f64>,
}

impl TransformNode {
	pub fn new(parent: Option<Weak<TransformNode>>, xform: Matrix4<f64>) -> TransformNode {
		let xform = if let Some(ref weak_parent) = parent {
			// use the parents xform if it exists
			// TODO: make sure this is the correct order of operations
			weak_parent.upgrade().unwrap().xform * xform
		} else {
			xform
		};

		// TODO: don't unwrap, do error checking? Probably should always be able to though
		let inverse = xform.invert().unwrap();
		let normi = mat3_from_mat4(xform).invert().unwrap().transpose();

		TransformNode {
			parent,
			children: Vec::new(),
			xform,
			inverse,
			normi,
		}
	}
}

trait Geometry {
	fn intersect(&self, ray: &Ray, isect: &mut Intersect) -> bool;

}

pub struct Camera {
	// TODO: better names once we know what these are
	m: Matrix3<f64>,
	eye: Vector3<f64>,
	look: Vector3<f64>,
	u: Vector3<f64>,
	v: Vector3<f64>,
	normalized_height: f64,
	aspect_ratio: f64,
}

impl Camera {
	pub fn new() -> Camera {
		Camera {
			m: Matrix3::identity(),
			eye: Vector3::zero(), // TODO: double check out glm does default vector init
			look: Vector3::zero(),
			u: Vector3::zero(),
			v: Vector3::zero(),
			normalized_height: 0.0,
			aspect_ratio: 0.0,
		}
	}
}

pub struct MaterialSceneObject {
	material: Material,
}

pub struct Material {
	// TODO: really not sure how this should be written, maybe options?

}

pub struct MaterialParameter {
	// TODO: really not sure how this should be written, maybe enums?
	value: Vector3<f64>,
	texture_map: Option<TextureMap>,
}

pub struct TextureMap {
	// TODO: probably use generic image buffer
	//		 look at cpp code to understand usage before implementing
}

// TODO: move this to a math helpers file
pub fn mat3_from_mat4(mat4: Matrix4<f64>) -> Matrix3<f64> {
	Matrix3::new(
		mat4.x.x, mat4.x.y, mat4.x.z,
		mat4.y.x, mat4.y.y, mat4.y.z,
		mat4.z.y, mat4.z.y, mat4.z.z
	)
}