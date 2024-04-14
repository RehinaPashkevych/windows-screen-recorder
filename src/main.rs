use std::{
    io::{self, Write},
    time::Instant,
};

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

struct GuiState {
    selected_encoder: VideoEncoderType,
    selected_quality: VideoEncoderQuality,
    selected_path: String,
}


static STATE: Lazy<Arc<Mutex<GuiState>>> = Lazy::new(|| {
    Arc::new(Mutex::new(GuiState::default()))
});

impl Default for GuiState {
    fn default() -> Self {
        Self {
            selected_encoder: VideoEncoderType::Mp4,
            selected_quality: VideoEncoderQuality::HD1080p,
            selected_path: String::new()
        }
    }
}


use windows_capture::{
    capture::GraphicsCaptureApiHandler,
    encoder::{VideoEncoder, VideoEncoderQuality, VideoEncoderType},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};

// This struct will be used to handle the capture events.
struct Capture {
    // The video encoder that will be used to encode the frames.
    encoder: Option<VideoEncoder>,
    // To measure the time the capture has been running
    start: Instant,
}

impl GraphicsCaptureApiHandler for Capture {
    type Flags = String;
    type Error = Box<dyn std::error::Error + Send + Sync>;

 
fn new(flags: Self::Flags) -> Result<Self, Self::Error> {
    println!("Got The Flag: {flags}");
    let state = STATE.lock().unwrap(); // Safely access the state

    // Determine the file extension based on the selected encoder type
    let file_extension = match state.selected_encoder {
        VideoEncoderType::Avi => "avi",
        VideoEncoderType::Hevc => "hevc",
        VideoEncoderType::Mp4 => "mp4",
        VideoEncoderType::Wmv => "wmv",
    };


    // Construct the filename using the formatted date-time and the file extension
    let filename = format!("{}rec.{}", state.selected_path, file_extension);

    // Create the video encoder with the new filename
    let encoder = VideoEncoder::new(
        state.selected_encoder,
        state.selected_quality,
        1920, 1080,
        &filename,
    )?;

    Ok(Self {
        encoder: Some(encoder),
        start: Instant::now(),
    })
}


    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        print!(
            "\rRecording for: {} seconds",
            self.start.elapsed().as_secs()
        );
        io::stdout().flush()?;

        self.encoder.as_mut().unwrap().send_frame(frame)?;

        if self.start.elapsed().as_secs() >= 6 {
            self.encoder.take().unwrap().finish()?;
            capture_control.stop();
            println!();  // Ensure output is on a new line
        }

        Ok(())
    }

    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture Session Closed");
        Ok(())
    }
}

// ============MAIN=======================================================================================


fn main() -> Result<(), eframe::Error> {
    let rt = tokio::runtime::Runtime::new().unwrap();  // Create the Tokio runtime

    //let path = String::from("C:/Users/РЕГИНА/Desktop");
    let state = Arc::clone(&STATE);  // Clone the Arc to capture a thread-safe reference to the state

    rt.block_on(async {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([420.0, 500.0]),
            ..Default::default()
        };

        eframe::run_simple_native("Screen recorder", options, move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Settings");
                
              
                ui.horizontal(|ui| {
                    let path_label = ui.label("Path + name: ");
                    let mut state_guard = STATE.lock().unwrap();
                    let state = &mut *state_guard; // Get a mutable reference to the state
                
                    ui.text_edit_singleline(&mut state.selected_path).labelled_by(path_label.id);
                });

                   // Ensure state is locked and accessed safely
                let mut state = state.lock().unwrap();
                
           
                egui::ComboBox::from_label("Encoder type: ")
                    .selected_text(format!("{:?}", state.selected_encoder))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut state.selected_encoder, VideoEncoderType::Avi, "Avi");
                        ui.selectable_value(&mut state.selected_encoder, VideoEncoderType::Hevc, "Hevc");
                        ui.selectable_value(&mut state.selected_encoder, VideoEncoderType::Mp4, "Mp4");
                        ui.selectable_value(&mut state.selected_encoder, VideoEncoderType::Wmv, "Wmv");
                    });

                egui::ComboBox::from_label("Quality: ")
                    .selected_text(format!("{:?}", state.selected_quality))
                    .show_ui(ui, |ui| {
                      //  ui.selectable_value(&mut state.selected_quality, VideoEncoderQuality::Auto, "Auto");
                        ui.selectable_value(&mut state.selected_quality, VideoEncoderQuality::HD1080p, "HD1080p");
                        ui.selectable_value(&mut state.selected_quality, VideoEncoderQuality::HD720p, "HD720p");
                        ui.selectable_value(&mut state.selected_quality, VideoEncoderQuality::Wvga, "Wvga");
                        ui.selectable_value(&mut state.selected_quality, VideoEncoderQuality::Ntsc, "Ntsc");
                        ui.selectable_value(&mut state.selected_quality, VideoEncoderQuality::Pal, "Pal");
                        ui.selectable_value(&mut state.selected_quality, VideoEncoderQuality::Vga, "Vga");
                        ui.selectable_value(&mut state.selected_quality, VideoEncoderQuality::Qvga, "Qvga");
                        ui.selectable_value(&mut state.selected_quality, VideoEncoderQuality::Uhd2160p, "Uhd2160p");
                        ui.selectable_value(&mut state.selected_quality, VideoEncoderQuality::Uhd4320p, "Uhd4320p");
            
                    });

                if ui.button("Start").clicked() {
                    tokio::spawn(async {
                        match run_recorder().await {
                            Ok(_) => println!("Recording started successfully."),
                            Err(e) => eprintln!("Failed to start recording: {:?}", e),
                        }
                    });
                }
            });
        })
    })
}



async fn run_recorder() -> Result<(), Box<dyn std::error::Error>> {
    let primary_monitor = Monitor::primary().expect("There is no primary monitor");

    let settings = Settings::new(
        primary_monitor,
        CursorCaptureSettings::Default,
        DrawBorderSettings::Default,
        ColorFormat::Rgba8,
        "Yea This Works".to_string(),
    )?;

    match Capture::start(settings) {
        Ok(_) => println!("Screen Capture started successfully."),
        Err(e) => {
            eprintln!("Screen Capture Failed: {:?}", e);
            return Err(Box::new(e));
        },
    }

    Ok(())
}

