mod editor;
mod file_watcher;
mod params;
mod runtime;

use nih_plug::prelude::*;
use std::sync::{Arc, Mutex};

pub struct VstJs {
    // VST プラグイン内で保持するデータ
    params: Arc<params::VstJsParams>,

    // JavaScript のランタイム
    runtime: Arc<Mutex<dyn runtime::runtime::ScriptRuntime + Sync + Send>>,

    sample_rate: f32,
    time: u64,
}

impl Default for VstJs {
    fn default() -> Self {
        let runtime: Arc<Mutex<dyn runtime::runtime::ScriptRuntime + Sync + Send>> =
            Arc::new(Mutex::new(
                runtime::js_sync::JsRuntimeBuilder::new()
                    .on_log(std::sync::Arc::new(|log| {
                        println!("{}", log);
                    }))
                    .build(),
            ));
        Self {
            params: Arc::new(params::VstJsParams::default()),
            runtime,
            sample_rate: 1.0,
            time: 0,
        }
    }
}

impl Plugin for VstJs {
    const NAME: &'static str = "vst_js";
    const VENDOR: &'static str = "vst_js";
    const URL: &'static str = env!("CARGO_PKG_REPOSITORY");
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];
    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::editor(self.params.clone(), self.runtime.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // デフォルトのスクリプトをコンパイル
        {
            let mut runtime = self.runtime.lock().unwrap();
            if let Err(err) = (&mut runtime).compile(&*self.params.code.lock().unwrap().clone()) {
                println!("{}", err);
            }
        }
        self.sample_rate = buffer_config.sample_rate;
        true
    }

    fn reset(&mut self) {
        self.time = 0;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // TODO: コピー回数をもっと減らして効率化できそう

        // 2 次元配列を 1 次元配列に変換
        // [[L, L, L, L], [R, R, R, R]] -> [L, L, L, L, R, R, R, R]
        let slice = buffer.as_slice();
        let len = slice.iter().map(|s| s.len()).sum();
        let mut audio = slice
            .iter()
            .fold(Vec::with_capacity(len), |mut input, channel| {
                input.extend_from_slice(channel);
                input
            });

        // イベントを取得
        let mut midi_input = Vec::<[u8; 7]>::new();
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn {
                    timing,
                    channel,
                    note,
                    velocity,
                    ..
                } => {
                    let mut midi = [0u8; 7];
                    midi[0..4].copy_from_slice(&timing.to_be_bytes());
                    midi[4] = 0x90 | channel;
                    midi[5] = note;
                    midi[6] = (velocity * 127.0).round().clamp(1.0, 127.0) as u8;
                    midi_input.push(midi);
                }
                NoteEvent::NoteOff {
                    timing,
                    channel,
                    note,
                    velocity,
                    ..
                } => {
                    let mut midi = [0u8; 7];
                    midi[0..4].copy_from_slice(&timing.to_be_bytes());
                    midi[4] = 0x80 | channel;
                    midi[5] = note;
                    midi[6] = (velocity * 127.0).round().clamp(1.0, 127.0) as u8;
                    midi_input.push(midi);
                }
                // TODO: 他のイベントも処理する
                _ => {}
            };
        }
        let mut midi_output = Vec::<[u8; 7]>::new();

        // スクリプトを実行
        {
            let mut runtime = self.runtime.lock().unwrap();
            if let Err(e) = (&mut runtime).audio(&mut audio) {
                println!("process error: {}", e);
            }
        }

        // 1 次元配列を 2 次元配列に変換
        // [L, L, L, L, R, R, R, R] -> [[L, L, L, L], [R, R, R, R]]
        slice.iter_mut().fold(0, |offset, channel| {
            let len = channel.len();
            channel.copy_from_slice(&audio[offset..offset + len]);
            offset + len
        });

        // イベントを送信
        for midi in midi_output {
            match midi[4] & 0xf0 {
                0x90 => {
                    context.send_event(NoteEvent::NoteOn {
                        timing: u32::from_be_bytes(midi[0..4].try_into().unwrap()),
                        voice_id: None,
                        channel: midi[4] & 0x0f,
                        note: midi[5],
                        velocity: midi[6] as f32 / 127.0,
                    });
                }
                0x80 => {
                    context.send_event(NoteEvent::NoteOff {
                        timing: u32::from_be_bytes(midi[0..4].try_into().unwrap()),
                        voice_id: None,
                        channel: midi[4] & 0x0f,
                        note: midi[5],
                        velocity: midi[6] as f32 / 127.0,
                    });
                }
                // TODO: 他のイベントも処理する
                _ => {}
            }

            //context.send_event(event);
        }

        ProcessStatus::Normal
    }
}

impl Vst3Plugin for VstJs {
    const VST3_CLASS_ID: [u8; 16] = *b"VstJs___________";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_vst3!(VstJs);
