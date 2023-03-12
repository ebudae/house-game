use std::fs::File;
use std::io::{Result, Write};

//struct Vertex {
//    position: [f32; 3],
//}

fn write_obj_file(vertices: &[Vertex], indices: &[u32], filename: &str) -> Result<()> {
    let mut file = File::create(filename)?;
    for v in vertices {
        writeln!(file, "v {} {} {}", v.x, v.y, v.z )?;
    }
    for i in (0..indices.len()).step_by(3) {
        writeln!(file, "f {} {} {}", indices[i] + 1, indices[i + 1] + 1, indices[i + 2] + 1)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let ( k,j ) = create_mesh();
    write_obj_file(&k, &j, "mesh0.obj")?;

    Ok(())
}

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

fn create_mesh() -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Create a 2D grid of vertices with a spacing of 1 unit between each vertex
    for i in 0..200 {
        for j in 0..200 {
            let x = (i as f32 - 100.0) * 1.0;
            let z = (j as f32 - 100.0) * 1.0;
            let _y = 0.0; // set the y-coordinate to 0, as the mesh is flat
            let y = (x.powi(2) + z.powi(2)).sqrt().sin() / (x.powi(2) + z.powi(2)).sqrt(); // calculate the y-coordinate using the sinc function

            vertices.push(Vertex { x, y, z });
        }
    }

    // Create indices that define triangles from adjacent vertices
    for i in 0..199 {
        for j in 0..199 {
            let v0 = i * 200 + j;
            let v1 = i * 200 + j + 1;
            let v2 = (i + 1) * 200 + j;
            let v3 = (i + 1) * 200 + j + 1;

            // First triangle
            indices.push(v0);
            indices.push(v1);
            indices.push(v2);

            // Second triangle
            indices.push(v2);
            indices.push(v1);
            indices.push(v3);
        }
    }

    (vertices, indices)
}