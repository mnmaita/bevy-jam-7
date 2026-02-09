use bevy::{
    asset::{LoadedFolder, RecursiveDependencyLoadState},
    prelude::*,
};
use bevy_kira_audio::prelude::{AudioPlugin as KiraAudioPlugin, *};

pub const ASSET_FOLDER_MUSIC: &str = "music";
pub const ASSET_FOLDER_SFX: &str = "sfx";

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((KiraAudioPlugin, SpatialAudioPlugin));

        app.init_resource::<AudioLoadStates>();
        app.init_resource::<MusicHandles>();
        app.init_resource::<SoundEffectHandles>();

        app.add_audio_channel::<BgmChannel>();

        app.add_systems(Startup, (load_music_files, load_sound_effect_files));
        app.add_systems(
            Update,
            (
                update_music_assets_load_state,
                update_sound_effect_assets_load_state,
            )
                .chain()
                .run_if(not(AudioLoadStates::loaded)),
        );

        app.add_observer(on_play_music);
        app.add_observer(on_stop_music);
        app.add_observer(on_play_sound_effect);
        app.add_observer(on_play_audio_channel);
        app.add_observer(on_stop_audio_channel);
    }
}

#[derive(Resource)]
pub struct BgmChannel;

#[derive(Resource)]
pub struct AudioLoadStates {
    sound_effects_load_state: RecursiveDependencyLoadState,
    music_load_state: RecursiveDependencyLoadState,
}

impl Default for AudioLoadStates {
    fn default() -> Self {
        Self {
            sound_effects_load_state: RecursiveDependencyLoadState::NotLoaded,
            music_load_state: RecursiveDependencyLoadState::NotLoaded,
        }
    }
}

impl AudioLoadStates {
    pub fn loaded(load_states: Res<AudioLoadStates>) -> bool {
        load_states.music_load_state.is_loaded() && load_states.sound_effects_load_state.is_loaded()
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct MusicHandles(
    #[cfg(not(target_family = "wasm"))] Handle<LoadedFolder>,
    #[cfg(target_family = "wasm")] Vec<Handle<AudioSource>>,
);

#[derive(Resource, Default, Deref, DerefMut)]
struct SoundEffectHandles(
    #[cfg(not(target_family = "wasm"))] Handle<LoadedFolder>,
    #[cfg(target_family = "wasm")] Vec<Handle<AudioSource>>,
);

pub struct PlaybackSettings {
    pub emitter: Option<Entity>,
    pub fade_in: Option<AudioTween>,
    pub loop_from: Option<f64>,
    pub loop_until: Option<f64>,
    pub panning: f32,
    pub playback_rate: f64,
    pub reverse: bool,
    pub volume: f32,
}

impl PlaybackSettings {
    pub fn from_volume(volume: f32) -> Self {
        Self {
            volume,
            ..Default::default()
        }
    }

    pub fn looped(mut self) -> Self {
        self.loop_from = Some(0.0);
        self
    }

    pub fn with_emitter(mut self, emitter: Entity) -> Self {
        self.emitter = Some(emitter);
        self
    }

    pub fn with_playback_rate(mut self, playback_rate: f64) -> Self {
        self.playback_rate = playback_rate;
        self
    }
}

impl Default for PlaybackSettings {
    fn default() -> Self {
        Self {
            emitter: None,
            fade_in: None,
            loop_from: None,
            loop_until: None,
            panning: 0.5,
            playback_rate: 1.0,
            reverse: false,
            volume: 1.0,
        }
    }
}

#[derive(Event)]
pub struct PlayMusic {
    file_name: String,
    settings: Option<PlaybackSettings>,
}

impl PlayMusic {
    pub fn new(file_name: impl Into<String>) -> Self {
        let file_name = file_name.into();
        Self {
            file_name,
            settings: None,
        }
    }

    pub fn with_settings(mut self, settings: PlaybackSettings) -> Self {
        self.settings = Some(settings);
        self
    }
}

#[derive(Event, Default)]
pub struct StopMusic {
    fade_out: Option<AudioTween>,
}

impl StopMusic {
    pub fn with_fade_out(mut self, fade_out: AudioTween) -> Self {
        self.fade_out = Some(fade_out);
        self
    }
}

#[derive(Event)]
pub struct PlaySoundEffect {
    pub file_name: String,
    pub settings: Option<PlaybackSettings>,
}

impl PlaySoundEffect {
    pub fn new(file_name: impl Into<String>) -> Self {
        let file_name = file_name.into();
        Self {
            file_name,
            settings: None,
        }
    }

    pub fn with_settings(mut self, settings: PlaybackSettings) -> Self {
        self.settings = Some(settings);
        self
    }
}

#[derive(Event)]
pub struct PlayAudioChannel {
    channel: String,
    file_name: String,
    settings: Option<PlaybackSettings>,
    music: bool,
}

impl PlayAudioChannel {
    pub fn new(channel: impl Into<String>, file_name: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            file_name: file_name.into(),
            settings: None,
            music: false,
        }
    }

    pub fn with_settings(mut self, settings: PlaybackSettings) -> Self {
        self.settings = Some(settings);
        self
    }

    pub fn with_music(mut self) -> Self {
        self.music = true;
        self
    }
}

#[derive(Event)]
pub struct StopAudioChannel {
    channel: String,
    fade_out: Option<AudioTween>,
}

impl StopAudioChannel {
    pub fn new(channel: impl Into<String>) -> Self {
        Self {
            channel: channel.into(),
            fade_out: None,
        }
    }

    pub fn with_fade_out(mut self, fade_out: AudioTween) -> Self {
        self.fade_out = Some(fade_out);
        self
    }
}

fn on_play_music(
    play_music: On<PlayMusic>,
    mut spatial_audio_emitters: Query<&mut SpatialAudioEmitter>,
    asset_server: Res<AssetServer>,
    bgm_audio_channel: Res<AudioChannel<BgmChannel>>,
) {
    let event = play_music.event();
    let path = format_music_file_name(&event.file_name);

    if bgm_audio_channel.is_playing_sound() {
        bgm_audio_channel.stop();
    }

    let mut play_audio_command = bgm_audio_channel.play(asset_server.load(path));

    if let Some(settings) = &event.settings {
        play_audio_with_settings(
            &mut play_audio_command,
            settings,
            settings
                .emitter
                .and_then(|entity| spatial_audio_emitters.get_mut(entity).ok()),
        );
    }
}

fn on_stop_music(stop_music: On<StopMusic>, bgm_audio_channel: Res<AudioChannel<BgmChannel>>) {
    let event = stop_music.event();
    let mut tween_command = bgm_audio_channel.stop();

    if let Some(fade_out_tween) = &event.fade_out {
        tween_command.fade_out(fade_out_tween.clone());
    }
}

fn on_play_sound_effect(
    play_sfx: On<PlaySoundEffect>,
    mut spatial_audio_emitters: Query<&mut SpatialAudioEmitter>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let event = play_sfx.event();
    let path = format_sfx_file_name(&event.file_name);
    let mut play_audio_command = audio.play(asset_server.load(path));

    if let Some(settings) = &event.settings {
        play_audio_with_settings(
            &mut play_audio_command,
            settings,
            settings
                .emitter
                .and_then(|entity| spatial_audio_emitters.get_mut(entity).ok()),
        );
    }
}

fn on_play_audio_channel(
    play_channel: On<PlayAudioChannel>,
    mut audio: ResMut<DynamicAudioChannels>,
    mut spatial_audio_emitters: Query<&mut SpatialAudioEmitter>,
    asset_server: Res<AssetServer>,
) {
    let event = play_channel.event();
    let channel = match audio.get_channel(&event.channel) {
        Some(channel) => channel,
        None => audio.create_channel(&event.channel),
    };
    let path = match event.music {
        true => format_music_file_name(&event.file_name),
        false => format_sfx_file_name(&event.file_name),
    };
    let mut play_audio_command = channel.play(asset_server.get_handle(path).unwrap_or_default());

    if let Some(settings) = &event.settings {
        play_audio_with_settings(
            &mut play_audio_command,
            settings,
            settings
                .emitter
                .and_then(|entity| spatial_audio_emitters.get_mut(entity).ok()),
        );
    };
}

fn on_stop_audio_channel(stop_channel: On<StopAudioChannel>, audio: ResMut<DynamicAudioChannels>) {
    let event = stop_channel.event();

    if let Some(channel) = audio.get_channel(&event.channel) {
        let mut tween_command = channel.stop();

        if let Some(fade_out_tween) = &event.fade_out {
            tween_command.fade_out(fade_out_tween.clone());
        }
    }
}

fn play_audio_with_settings(
    play_audio_command: &mut PlayAudioCommand,
    settings: &PlaybackSettings,
    opt_spatial_audio_emitter: Option<Mut<SpatialAudioEmitter>>,
) {
    if settings.reverse {
        play_audio_command.reverse();
    }

    if let Some(fade_in) = &settings.fade_in {
        play_audio_command.fade_in(fade_in.clone());
    }

    if let Some(loop_from) = settings.loop_from {
        play_audio_command.loop_from(loop_from);
    }

    if let Some(loop_until) = settings.loop_until {
        play_audio_command.loop_until(loop_until);
    }

    if let Some(entity) = settings.emitter
        && let Some(mut spatial_audio_emitter) = opt_spatial_audio_emitter
    {
        spatial_audio_emitter.instances = vec![play_audio_command.handle()];
        play_audio_command.with_emitter(entity);
    }

    play_audio_command
        .with_panning(settings.panning)
        .with_playback_rate(settings.playback_rate)
        .with_volume(settings.volume);
}

fn load_music_files(mut commands: Commands, asset_server: Res<AssetServer>) {
    let music_handles = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server.load_folder(ASSET_FOLDER_MUSIC)
        }

        #[cfg(target_family = "wasm")]
        {
            let asset_music_list = [
                // format_music_file_name("example.ogg"),
            ];
            asset_music_list
                .iter()
                .map(|path| asset_server.load::<AudioSource>(path))
                .collect::<Vec<Handle<AudioSource>>>()
        }
    };

    commands.insert_resource(MusicHandles(music_handles));
}

fn load_sound_effect_files(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sound_effect_handles = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server.load_folder(ASSET_FOLDER_SFX)
        }

        #[cfg(target_family = "wasm")]
        {
            let asset_sfx_list = [
                // format_sfx_file_name("example.ogg"),
            ];
            asset_sfx_list
                .iter()
                .map(|path| asset_server.load::<AudioSource>(path))
                .collect::<Vec<Handle<AudioSource>>>()
        }
    };

    commands.insert_resource(SoundEffectHandles(sound_effect_handles));
}

fn update_music_assets_load_state(
    mut audio_load_states: ResMut<AudioLoadStates>,
    music_handles: Res<MusicHandles>,
    asset_server: Res<AssetServer>,
) {
    audio_load_states.music_load_state = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server.recursive_dependency_load_state(music_handles.id())
        }
        #[cfg(target_family = "wasm")]
        {
            let all_loaded = music_handles.iter().all(|handle| {
                asset_server
                    .recursive_dependency_load_state(handle.id())
                    .is_loaded()
            });
            if all_loaded {
                RecursiveDependencyLoadState::Loaded
            } else {
                RecursiveDependencyLoadState::NotLoaded
            }
        }
    };
}

fn update_sound_effect_assets_load_state(
    mut audio_load_states: ResMut<AudioLoadStates>,
    sound_effect_handles: Res<SoundEffectHandles>,
    asset_server: Res<AssetServer>,
) {
    audio_load_states.sound_effects_load_state = {
        #[cfg(not(target_family = "wasm"))]
        {
            asset_server.recursive_dependency_load_state(sound_effect_handles.id())
        }
        #[cfg(target_family = "wasm")]
        {
            let all_loaded = sound_effect_handles.iter().all(|handle| {
                asset_server
                    .recursive_dependency_load_state(handle.id())
                    .is_loaded()
            });
            if all_loaded {
                RecursiveDependencyLoadState::Loaded
            } else {
                RecursiveDependencyLoadState::NotLoaded
            }
        }
    };
}

fn format_music_file_name(file_name: &str) -> String {
    format!("{ASSET_FOLDER_MUSIC}/{file_name}")
}

fn format_sfx_file_name(file_name: &str) -> String {
    format!("{ASSET_FOLDER_SFX}/{file_name}")
}
