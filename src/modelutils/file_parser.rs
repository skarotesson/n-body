use std::io::Read;
use crate::modelutils;

pub fn file2obj(filename: &str) -> modelutils::Model {
    let file_path = std::path::Path::new(filename);
    let display_name = file_path.display();

    let mut file_handle = match std::fs::File::open(&file_path) {
        Err(err_msg) => panic!("Couldn't open file \"{}\". Reason: {}", display_name, err_msg),
        Ok(file) => file
    };

    let mut file_contents = String::new();
    match file_handle.read_to_string(&mut file_contents) {
        Err(err_msg) => panic!("Couldn't read file \"{}\". Reason: {}", display_name, err_msg),
        Ok(_) => ()
    }

    // Vertices
    let mut v: Vec<f32> = vec![];
    let mut v_indices: Vec<u32> = vec![];
    
    // Texture coordinates
    let mut vt: Vec<f32> = vec![];
    let mut vt_indices: Vec<u32> = vec![];
    
    // Normals
    let mut vn: Vec<f32> = vec![];
    let mut vn_indices: Vec<u32> = vec![];

    

    for line in file_contents.lines() {
        if line.len() > 2 {
            match &line[..2] {
                "v " => {
                    parse_obj_line(&mut v, line, false);
                },
                
                "vt" => {
                    parse_obj_line(&mut vt, line, false);
                },
                "vn" => {
                    parse_obj_line(&mut vn, line, false);
                },
                "vp" => (), // TODO: Implement free form geometry
                "f " => {
                    let mut data = line.split(' ');
                    data.next(); // Get slash separated index data (e.g. 4/5/6)
                    
                    let vertex_1 = data.next().unwrap();

                    // OBJ files may define n-polygons. Given a line with n points, assume that a
                    // triangle fan is being defined; take the first vertex and then windows of
                    // two. e.g. f 1 2 3 4 5 = f 1 2 3, f 1 3 4, f 1 3 5
                    let _ = data.clone().zip(data.skip(1)) // TODO: CLONE NOOOOO
                        .for_each(|(vertex_2,vertex_3)| {
                            for vertex in [vertex_1, vertex_2, vertex_3] {
                                // Wavefront indices come in the following formats:
                                // v v v
                                // v/vt v/vt v/vt
                                // v//vn v//vn v//vn
                                // v/vt/vn v/vt/vn v/vt/vn
                                // v is always first, then vt, then vn
                                // Hence, we can use a match statement.
                                for (i, chunk) in vertex.split("/").enumerate() {
                                    match i {
                                        0 => v_indices.push(chunk.parse().unwrap()),
                                        1 => vt_indices.push(chunk.parse().unwrap()),
                                        2 => vn_indices.push(chunk.parse().unwrap()),
                                        _ => (),
                                    }
                                }
                            }
                        });

                    //parse_obj_line(&mut v_indices, line, true);
                },
                _ => (),
            }
        }
    }
    // Unlike OpenGL, v_indices in OBJ files are 1-indexed
    // Therefore offset by -1:
    
    // TODO: A bit hacky (?)
    v_indices.iter_mut().for_each(|x| *x -= 1 );
    vt_indices.iter_mut().for_each(|x| *x -= 1 );
    vn_indices.iter_mut().for_each(|x| *x -= 1 );

    

    // Zip texture coordinate with vertices into a format OpenGL will understand
    let mut vertices: Vec<f32> = vec![];
    let mut indices: Vec<u32> = vec![];

    // There is an index for each unique vertex coordinate
    let mut vt_indices_sorted: Vec<std::collections::HashSet<u32>> = vec![std::collections::HashSet::new(); v.len()/3];
    
    // Maps (v_index, vt_index) to indices corresponding to constructed vector (mut vertices) which
    // is passed directly to OpenGL
    let mut index_map: std::collections::HashMap<(u32, u32), u32> = std::collections::HashMap::new();

    // Some models don't provide texture coordinates. Provide dummy data
    if vt_indices.len() == 0 {
        vt_indices = vec![0; v_indices.len()];
        vt.push(0.0);
        vt.push(0.0);
    }

    // Assuming that each vertex only has one associated UV coordinate, we take the vertices in
    // order of appearance and assign a UV texture coordinate (vt) index to it. This allows us to
    // make a single vector with all the data in order: [x1, y1, z1, u1, v1, x2, y2, ...] so that
    // we can feed it to our shader program directly.
    v_indices.iter()
        .zip(vt_indices.iter())
        .for_each( |(v_i, vt_i)| {
                // For each vertex, find the index for the corresponding normal coordinate.
               // println!("{:?} -> {:?}", vt_indices_sorted[*v_i as usize], *vt_i);
                vt_indices_sorted[*v_i as usize].insert(*vt_i);
            }
        );

    // We take groups of three values (x,y,z) to get vertex coordinates. We then index into the 
    // UV index array using the previously found index associated with that vertex. We have an
    // output vector `vertices: Vec<f32>` which we concatenate all the vertex data onto.
    v.chunks(3)
        .enumerate()
        .zip(vt_indices_sorted.iter())
        .for_each( |((v_index, vertex), vt_set)| {
            for vt_index in vt_set {
                index_map.insert((v_index as u32, *vt_index as u32), vertices.len() as u32/5);
                vertices.extend_from_slice(&vertex);
                vertices.extend_from_slice(&vt[2* *vt_index as usize..2* *vt_index as usize+2]);

                //println!("{:?}");
            }
        }
    );

    v_indices.iter()
        .zip(vt_indices.iter())
        .for_each(|(v_i, vt_i)| {
            indices.push(*index_map.get(&(*v_i, *vt_i)).unwrap());
        }
    );


    // Create new model using vertices
    modelutils::Model::new(&vertices, &indices)
}

// Generics are used to make function valid for both u32 and f32
fn parse_obj_line<T: std::str::FromStr + std::fmt::Debug>(array: &mut Vec<T>, line: &str, slash_separated: bool)
where <T as std::str::FromStr>::Err: std::fmt::Debug {

    // From "v 1.0 1.0 1.0" create ["v", "1.0", "1.0", "1.0"]
    let mut data = line.split(' ');

    if slash_separated {

    }

    // Each line begins with an identifier.
    // Skip to the data with `data.next();`
    data.next();

    // Loop through the remainder (the numbers) and add them to v.
    for datum in data {
        array.push(datum.parse().unwrap());
    }
}
