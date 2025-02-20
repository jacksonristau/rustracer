export let scene = {
    "spheres":[],
    "materials":[],
    "lights":[],
    "obj_file": "",
    "eye_pos":{
        "x":0.0,"y":0.0,"z":0.0,"w":1.0
    },
    "view_dir":{
        "x":0.0,"y":0.0,"z":-1.0,"w":0.0
    },
    "up_dir":{
        "x":0.0,"y":1.0,"z":0.0,"w":0.0
    },
    "hfov":60.0,
    "resolution":[1000,800],
    "bkg_color":{
        "r":0.2,"g":0.2,"b":0.2
    },
    "frustum_width":2.0,
    "parallel":false,
    "dc": {"r":0.2,"g":0.2,"b":0.2},
    "alpha": [0.0, 1.0],
    "dist": [1, 30]
};

/*
sphere:
{
    center: {x:0.0, y:0.0, z:-8.0, w:1.0},
    radius: 2.0,
    material_index: 0
}

material
{
    "diffuse":{"r":1.0,"g":0.0,"b":0.0},
    "specular":{"r":0.0,"g":0.0,"b":0.0},
    "k_a":0.0,"k_d":0.0,"k_s":0.0,
    "alpha":0.0,
    "index_of_refraction":0.0,
    "n_val":0,
    "texture":null
}
*/