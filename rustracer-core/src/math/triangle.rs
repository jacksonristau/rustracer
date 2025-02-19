use wavefront::{Obj, Vertex};

use super::vector::Vector;

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub position : (Vector, Vector, Vector),
    pub normals : (Vector, Vector, Vector),
    pub uvs : ([f32; 3], [f32; 3], [f32; 3]),
}

impl Triangle {
    pub fn new(p1: Vector, p2: Vector, p3: Vector, n1: Vector, n2: Vector, n3: Vector, uv1 : [f32; 3], uv2 : [f32; 3], uv3 : [f32; 3]) -> Self {
        Triangle {
            position : (p1, p2, p3),
            normals : (n1, n2, n3),
            uvs : (uv1, uv2, uv3),
        }
    }

    fn from_model(triangle : [Vertex; 3]) -> Self{
        Triangle {
            position : (Vector::new(triangle[0].position()[0], triangle[0].position()[1], triangle[0].position()[2], 1.0),
                        Vector::new(triangle[1].position()[0], triangle[1].position()[1], triangle[1].position()[2], 1.0),
                        Vector::new(triangle[2].position()[0], triangle[2].position()[1], triangle[2].position()[2], 1.0)),
            normals : (Vector::new(triangle[0].normal().unwrap()[0], triangle[0].normal().unwrap()[1], triangle[0].normal().unwrap()[2], 0.0),
                        Vector::new(triangle[1].normal().unwrap()[0], triangle[1].normal().unwrap()[1], triangle[1].normal().unwrap()[2], 0.0),
                        Vector::new(triangle[2].normal().unwrap()[0], triangle[2].normal().unwrap()[1], triangle[2].normal().unwrap()[2], 0.0)),
        }
        // Triangle {
        //     position : (Vector::new(triangle[0].position()[0], triangle[0].position()[1], triangle[0].position[2], 1.0),
        //                 Vector::new(triangle[1].position()[0], triangle[1].position()[1], triangle[1].position[2], 1.0),
        //                 Vector::new(triangle[2].position()[0], triangle[2].position()[1], triangle[2].position[2], 1.0)),
        //     normals : (Vector::new(triangle[0].normal().unwrap()[0], triangle[0].normal()[1], triangle[0].normal[2], 0.0),
        //                 Vector::new(triangle[1].normal()[0], triangle[1].normal()[1], triangle[1].normal[2], 0.0),
        //                 Vector::new(triangle[2].normal()[0], triangle[2].normal()[1], triangle[2].normal[2], 0.0)),
        //     uvs : (
        //         triangle[0].uv.unwrap_or([0.0, 0.0, 0.0]),
        //         triangle[1].uv.unwrap_or([0.0, 0.0, 0.0]),
        //         triangle[2].uv.unwrap_or([0.0, 0.0, 0.0]),
        //     ),
        // }
    }

    pub fn from_obj(filename : &str) -> Vec<Triangle>{
        let model = Obj::from_file(filename).unwrap();
        model.triangles().map(Self::from_model).collect()
    }
}