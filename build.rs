use ambient_pipeline_types::{
    materials::{MaterialsImporter, PipelinePbrMaterial},
    models::{MaterialFilter, MaterialOverride, ModelTransform},
    Collider, MaterialsPipeline, ModelImporter, ModelsPipeline, Pipeline, PipelineProcessor,
    PipelinesFile,
};
use serde::{Deserialize, Serialize};
use std::vec;

#[derive(Debug, Serialize, Deserialize)]
struct Model {
    name: String,
    inherits: String,
    model: String,
    diffuse: String,
    specular: Option<String>,
    alpha_threshold: Option<f32>,
    alpha: Option<bool>,
    specular_exponent: Option<f32>,
    use_specular: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GroundTexture {
    diffuse: String,
    specular: Option<String>,
}

fn create_model_pipelines(source: Vec<Model>, tags: Vec<String>) -> Vec<Pipeline> {
    source
        .into_iter()
        .map(|item| Pipeline {
            processor: PipelineProcessor::Models(ModelsPipeline {
                importer: ModelImporter::Regular,
                output_prefabs: true,
                output_animations: true,
                collider: Collider::FromModel {
                    flip_normals: false,
                    reverse_indices: false,
                },
                material_overrides: vec![MaterialOverride {
                    filter: MaterialFilter::All,
                    material: PipelinePbrMaterial {
                        base_color: item.diffuse.parse().ok(),
                        specular: item.specular.and_then(|x| x.parse().ok()),
                        specular_exponent: item.specular_exponent.map(|x| x * 0.1),
                        alpha_cutoff: Some(item.alpha_threshold.unwrap_or(0.95)),
                        metallic_factor: Some(1.),
                        roughness_factor: Some(1.),
                        transparent: item.alpha,
                        ..Default::default()
                    },
                }],
                transforms: vec![
                    ModelTransform::Scale { scale: 0.1 },
                    ModelTransform::RotateZ { deg: -90. },
                ],
                ..Default::default()
            }),
            sources: vec![item.model],
            tags: tags.clone(),
            categories: vec![],
        })
        .collect()
}

fn main() {
    let man_made: Vec<Model> =
        serde_json::from_str(include_str!("assets/props_man_made.json")).unwrap();
    let generic: Vec<Model> =
        serde_json::from_str(include_str!("assets/props_generic.json")).unwrap();
    let plants: Vec<Model> = serde_json::from_str(include_str!("assets/plants.json")).unwrap();
    let units: Vec<Model> = serde_json::from_str(include_str!("assets/units.json")).unwrap();

    let ground_textures: Vec<GroundTexture> =
        serde_json::from_str(include_str!("assets/ground_textures.json")).unwrap();

    let ground_textures = ground_textures
        .into_iter()
        .map(|mat| Pipeline {
            processor: PipelineProcessor::Materials(MaterialsPipeline {
                importer: Box::new(MaterialsImporter::Single(PipelinePbrMaterial {
                    name: Some(mat.diffuse.clone()),
                    base_color: Some(
                        format!("Data/Models/GroundTextures/{}", mat.diffuse)
                            .parse()
                            .unwrap(),
                    ),
                    specular: mat
                        .specular
                        .map(|x| format!("Data/Models/GroundTextures/{}", x).parse().unwrap()),
                    ..Default::default()
                })),
                output_decals: false,
            }),
            sources: vec![],
            tags: vec![],
            categories: vec![],
        })
        .collect::<Vec<_>>();

    let pipelines = [
        create_model_pipelines(man_made, vec!["Man made".to_string()]),
        create_model_pipelines(generic, vec!["Generic".to_string()]),
        create_model_pipelines(plants, vec!["Plants".to_string()]),
        create_model_pipelines(units, vec!["Units".to_string()]),
        ground_textures,
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    std::fs::write(
        "assets/pipeline.toml",
        toml::to_string_pretty(&PipelinesFile { pipelines }).unwrap(),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=assets/*.json");
    println!("cargo:rerun-if-changed=assets/Data");
    println!("cargo:rerun-if-changed=assets/Maps");
}
