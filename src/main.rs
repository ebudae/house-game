use std::fs::File;
use std::io::{Result, Write};
use rand::prelude::*;

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
    let mut wvertices = Vec::new();
    let mut rng = rand::thread_rng();

    for _i in 0..=60 {
        let x = rng.gen::<f32>() * 200.0 - 100.0;
        let z = rng.gen::<f32>() * 200.0 - 100.0;
        let y = rng.gen::<f32>() * 2.0 - 1.0;
        wvertices.push(Vertex { x, y, z });
    }

    // Create a 2D grid of vertices with a spacing of 1 unit between each vertex
    for i in 0..=200 {
        for j in 0..=200 {
            let x = (i as f32 - 100.0) * 1.0;
            let z = (j as f32 - 100.0) * 1.0;
            let _y = 0.0; // set the y-coordinate to 0, as the mesh is flat
            let _y = ((x/8.0).powi(2) + (z/8.0).powi(2)).sqrt().sin() / ((x/8.0).powi(2) + (z/8.0).powi(2)).sqrt() *5.0; // calculate the y-coordinate using the sinc function
            let y = sinc( x, z, 8.0, 8.0, 8.0 ) 
                - sinc( x-30.0, z + 40.0, 3.0, 8.0, 6.0 )
                + sinc( x+3.0, z + 20.0, 19.0, 38.0, 2.0 ) 
                + sinc( x + 30.0, z, 17.68, 14.24, 7.3 )
                + worley( &wvertices, x, z )
                + sinc( x, z, 4.0, 4.0, 2.0 ) * sinc( 5.0 - x, 7.0 - z, 8.0, 6.0, 2.0 );

            vertices.push(Vertex { x, y, z });
        }
    }

    // Create indices that define triangles from adjacent vertices
    for i in 0..=199 {
        for j in 0..=199 {
            let v0 = i * 201 + j;
            let v1 = i * 201 + j + 1;
            let v2 = (i + 1) * 201 + j;
            let v3 = (i + 1) * 201 + j + 1;

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

type T = f32;
fn sinc( i: T, j: T,xt: T, zt: T, a: T )
    -> T
{
    //if ((i/xt).powi(2) + (j/zt).powi(2)) == 0.0{
    if i == 0.0 && j == 0.0 {
        a
    }else{
        ((i/xt).powi(2) + (j/zt).powi(2)).sqrt().sin() / 
        ((i/xt).powi(2) + (j/zt).powi(2)).sqrt() * a
    }
}

fn distance(x1: T, y1: T, x2: T, y2: T) -> T {
    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt()
}

fn worley( k: &Vec<Vertex>, i: f32, j:f32 )
    -> f32
{
    let mut q = 99999.99;
    let mut which = -1i32;
    for m in 0..k.len() {
        let n = distance(i, j, k[m].x, k[m].z);
        if n < q {
            q = n; 
            which = m as i32;
        }
    }
    k[which as usize].y
}