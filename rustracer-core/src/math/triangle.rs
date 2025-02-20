use wavefront::{Obj, Vertex};

use super::vector::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub position : (Vector, Vector, Vector),
    pub normals : Option<(Vector, Vector, Vector)>,
    pub uvs : Option<([f32; 3], [f32; 3], [f32; 3])>,
}

impl Triangle {
    pub fn new(p1: Vector, p2: Vector, p3: Vector, n1: Vector, n2: Vector, n3: Vector, uv1 : [f32; 3], uv2 : [f32; 3], uv3 : [f32; 3]) -> Self {
        Triangle {
            position : (p1, p2, p3),
            normals : Some((n1, n2, n3)),
            uvs : Some((uv1, uv2, uv3)),
        }
    }

    fn from_model(triangle : [Vertex; 3]) -> Self{
        let pos = (
            Vector::from_vec(triangle[0].position(), 1.0),
            Vector::from_vec(triangle[1].position(), 1.0),
            Vector::from_vec(triangle[2].position(), 1.0)
        );
        let normals = None;
        let uvs = None;
        if triangle[0].normal().is_some() && triangle[1].normal().is_some() && triangle[2].normal().is_some() {
            let normals = (
                Vector::from_vec(triangle[0].normal().unwrap(), 0.0),
                Vector::from_vec(triangle[1].normal().unwrap(), 0.0),
                Vector::from_vec(triangle[2].normal().unwrap(), 0.0)
            );
        }
        if triangle[0].uv().is_some() && triangle[1].uv().is_some() && triangle[2].uv().is_some() {
            let uvs = (
                triangle[0].uv().unwrap(),
                triangle[1].uv().unwrap(),
                triangle[2].uv().unwrap()
            );
        }
        Triangle {
            position : pos,
            normals : normals,
            uvs : uvs,
        }
    }

    pub fn from_obj(filename : &str) -> Vec<Triangle>{
        let model = Obj::from_file(filename).unwrap();
        model.triangles().map(Self::from_model).collect()
    }
}