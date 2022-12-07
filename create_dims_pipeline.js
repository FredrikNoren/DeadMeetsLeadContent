const manMade = require('./props_man_made.json');
const generic = require('./props_generic.json');
const plants = require('./plants.json');

function createModelPipelines(source, tags) {
    return source.map(item => {
        return {
            pipeline: {
                type: "Models",
                importer: {
                    type: "Regular"
                },
                output_objects: true,
                output_animations: true,
                collider: {
                    type: "FromModel"
                },
                material_overrides: [
                    {
                        filter: { type: "All" },
                        material: {
                            base_color: item.diffuse,
                            specular: item.specular,
                            specular_exponent: item.specular_exponent * 0.1,
                            alpha_cutoff: item.alpha_threshold ?? 0.95,
                            metalic: 1,
                            roughness: 1,
                        }
                    }
                ],
                transforms: [
                    { type: 'Scale', scale: 0.1 }
                ],
            },
            sources: [item.model.replace(".x", ".glb")],
            tags
        };
    });
}

const groundTextures = [
    { diffuse: 'Field1.png' },
    { diffuse: 'Grass1.png', specular: 'GrassSpecular1.png' },
    { diffuse: 'Grass2.png', specular: 'GrassSpecular1.png' },
    { diffuse: 'Mayatile1.png' },
    { diffuse: 'Moss1.png', specular: 'MossSpecular1.png' },
    { diffuse: 'Mud1.png', specular: 'MudSpecular1.png' },
    { diffuse: 'Pebbles1.png', specular: 'PebblesSpecular1.png' },
    { diffuse: 'Rock1.png', specular: 'RockSpecular1.png' },
    { diffuse: 'Sand1.png', specular: 'SandSpecular1.png' },
    { diffuse: 'Vines1.png' },
].map(mat => ({
    pipeline: {
        type: 'Materials',
        importer: {
            type: 'Single',
            name: mat.diffuse,
            base_color: `Data/Models/GroundTextures/${mat.diffuse}`,
            specular: mat.specular ? `Data/Models/GroundTextures/${mat.specular}` : undefined
        }
    },
}));

let pipelines = [
    ...createModelPipelines(manMade, ['Man made']),
    ...createModelPipelines(generic, ["Generic"]),
    ...createModelPipelines(plants, ["Plants"]),
    ...groundTextures
];
require('fs').writeFileSync('dims_pipeline.json', JSON.stringify(pipelines, null, 2));
