#[cfg(feature = "prototty_graphical")]
use prototty_graphical::*;
#[cfg(feature = "prototty_graphical_gfx")]
use prototty_graphical_gfx::*;
use prototty_native_audio::NativeAudioPlayer;
use rip_native::{simon::Arg, NativeCommon};
use rip_prototty::{app, Frontend};

struct NoAudio;

impl prototty_audio::AudioPlayer for NoAudio {
    type Sound = ();
    fn play(&self, _sound: &Self::Sound, _properties: prototty_audio::AudioProperties) {}
    fn load_sound(&self, _bytes: &'static [u8]) -> Self::Sound {
        ()
    }
}

fn main() {
    env_logger::init();
    let NativeCommon {
        rng_seed,
        file_storage,
        controls,
        save_file,
    } = NativeCommon::arg().with_help_default().parse_env_or_exit();
    let audio_player = NativeAudioPlayer::new_default_device();
    let context = Context::new(ContextDescriptor {
        font_bytes: FontBytes {
            normal: include_bytes!("./fonts/PxPlus_IBM_CGAthin.ttf").to_vec(),
            bold: include_bytes!("./fonts/PxPlus_IBM_CGA.ttf").to_vec(),
        },
        title: "Template Roguelike".to_string(),
        window_dimensions: WindowDimensions::Windowed(Dimensions {
            width: 640.,
            height: 480.,
        }),
        cell_dimensions: Dimensions {
            width: 12.,
            height: 12.,
        },
        font_dimensions: Dimensions {
            width: 12.,
            height: 12.,
        },
        underline_width: 0.1,
        underline_top_offset: 0.8,
    })
    .unwrap();
    let app = app(
        Frontend::Native,
        controls,
        file_storage,
        save_file,
        audio_player,
        rng_seed,
    );
    context.run_app(app);
}
