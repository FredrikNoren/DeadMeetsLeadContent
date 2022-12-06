const manMade = require('./props_man_made.json');
const generic = require('./props_generic.json');
const plants = require('./plants.json');

function createPipelines(source, tags) {
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

let pipelines = [
    ...createPipelines(manMade, ['Man made']),
    ...createPipelines(generic, ["Generic"]),
    ...createPipelines(plants, ["Plants"])
];
require('fs').writeFileSync('dims_pipeline.json', JSON.stringify(pipelines, null, 2));
