use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

const ASPECT_RATIO: f32 = 16. / 9.;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_fit_inside_current_level)
            .insert_resource(LevelSelection::index(0));
    }
}

// pub const PLAYER_RENDER_LAYER: RenderLayers = RenderLayers::layer(2);

fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("Dungeon.ldtk").into(),
        ..Default::default()
    });
}


pub fn camera_fit_inside_current_level(
    mut camera_query: Query<
        (
            &mut bevy::render::camera::OrthographicProjection,
            &mut Transform,
        ),
        // Without<Player>,
    >,
    level_query: Query<(&Transform, &LevelIid), Without<OrthographicProjection>>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    level_selection: Res<LevelSelection>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    {
        let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

        for (level_transform, level_iid) in &level_query {
            // memory unsafe version
            // let ldtk_project = ldtk_project_assets
            //     .get(ldtk_projects.single())
            //     .expect("Project should be loaded if level has spawned");

            if let Some(ldtk_project_handle) = ldtk_projects.iter().next() {
                if let Some(ldtk_project) = ldtk_project_assets.get(ldtk_project_handle) {
                    let level = ldtk_project
                    .get_raw_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                if level_selection.is_match(&LevelIndices::default(), level) {
                    let level_ratio = level.px_wid as f32 / level.px_hei as f32;
                    orthographic_projection.viewport_origin = Vec2::ZERO;
                    if level_ratio > ASPECT_RATIO {
                        // level is wider than the screen
                        let height = (level.px_hei as f32 / 9.).round() * 9.;
                        let width = height * ASPECT_RATIO;
                        orthographic_projection.scaling_mode =
                            bevy::render::camera::ScalingMode::Fixed { width, height };
                        camera_transform.translation.y = 0.;
                    } else {
                        // level is taller than the screen
                        let width = (level.px_wid as f32 / 16.).round() * 16.;
                        let height = width / ASPECT_RATIO;
                        orthographic_projection.scaling_mode =
                            bevy::render::camera::ScalingMode::Fixed { width, height };
                        camera_transform.translation.x = 0.;
                    }

                    camera_transform.translation.x += level_transform.translation.x;
                    camera_transform.translation.y += level_transform.translation.y;
                }
                } else {
                    warn!("LdtkProject not yet loaded");
                    return;
                }
            } else {
                warn!("No LdtkProjectHandle found in the world!");
                return;
            }



        }
    }
}
