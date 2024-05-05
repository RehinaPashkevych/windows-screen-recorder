use std::{
    io::{self, Write},
    time::{Instant},
};

use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use chrono::Local; 

struct GuiState {
    selected_encoder: VideoEncoderType,
    selected_quality: VideoEncoderQuality,
    selected_path: String,
    recording_active: bool,
    recording_start: Option<Instant>,
}


static STATE: Lazy<Arc<Mutex<GuiState>>> = Lazy::new(|| {
    Arc::new(Mutex::new(GuiState::default()))
});

impl Default for GuiState {
    fn default() -> Self {
        Self {
            selected_encoder: VideoEncoderType::Mp4,
            selected_quality: VideoEncoderQuality::HD1080p,
            selected_path: String::new(),
            recording_active: false,
            recording_start: None,
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


        // Include current date and time in the filename
        let now = Local::now();
        let datetime_format = now.format("%Y%m%d_%H%M%S").to_string(); // Format date and time
        let filename = format!("{}rec_{}.{}", state.selected_path, datetime_format, file_extension);

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

        if let Some(encoder) = self.encoder.as_mut() {
            encoder.send_frame(frame)?;
        }
    
        // Check the recording active state
        let state = STATE.lock().unwrap();
        if !state.recording_active {
            self.encoder.take().unwrap().finish()?;
            capture_control.stop();
            println!("\nCapture Session Closed");
            return Ok(());
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

    let state = Arc::clone(&STATE);  // Clone the Arc to capture a thread-safe reference to the state

    rt.block_on(async {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([420.0, 200.0]),
            ..Default::default()
        };

        eframe::run_simple_native("Screen recorder", options, move |ctx, _frame| {


            // Custom visuals at the context level
            let mut visuals = egui::Visuals::dark();
            visuals.override_text_color = Some(egui::Color32::from_rgb(255, 255, 255));  // Make all text white
            visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(25, 20, 20);
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(40, 35, 35);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(60, 55, 55);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(80, 75, 75);
            visuals.widgets.inactive.bg_stroke.width = 0.0;
            visuals.widgets.active.bg_stroke.width = 1.0;
            visuals.widgets.active.bg_stroke.color = egui::Color32::from_rgb(140, 140, 255);
            ctx.set_visuals(visuals);

                // Adjust font sizes
            let font_id = egui::FontId::new(16.0, egui::FontFamily::Proportional); // Create a font identifier for size 20
            let mut style = (*ctx.style()).clone();
            style.text_styles.insert(egui::TextStyle::Body, font_id.clone()); // Use clone for Body text style
            style.text_styles.insert(egui::TextStyle::Button, font_id.clone()); // Use clone for Button text style
            style.text_styles.insert(egui::TextStyle::Heading, egui::FontId::new(22.0, egui::FontFamily::Proportional)); // Larger font for headings
            ctx.set_style(style);



            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Settings");

                ui.horizontal(|ui| {
                    let path_label = ui.label("Path + name: ");
                    let mut state_guard = STATE.lock().unwrap();
                    let state = &mut *state_guard;
                    ui.text_edit_singleline(&mut state.selected_path).labelled_by(path_label.id);
                });

                // Accessing the state safely
                let mut state = state.lock().unwrap();

                // Horizontal layout for encoder type label and combobox
                ui.horizontal(|ui| {
                    ui.label("Encoder type: ");
                    egui::ComboBox::from_id_source("encoder_type")
                        .selected_text(format!("{:?}", state.selected_encoder))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut state.selected_encoder, VideoEncoderType::Avi, "Avi");
                            ui.selectable_value(&mut state.selected_encoder, VideoEncoderType::Hevc, "Hevc");
                            ui.selectable_value(&mut state.selected_encoder, VideoEncoderType::Mp4, "Mp4");
                            ui.selectable_value(&mut state.selected_encoder, VideoEncoderType::Wmv, "Wmv");
                        });
                });

                // Horizontal layout for quality label and combobox
                ui.horizontal(|ui| {
                    ui.label("Quality: ");
                    egui::ComboBox::from_id_source("quality")
                        .selected_text(format!("{:?}", state.selected_quality))
                        .show_ui(ui, |ui| {
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
                });

                ui.add_space(20.0); // Add 20 pixels of space before the button row

                ui.horizontal(|ui| {
                    // Temporarily disable UI interaction if recording is active
                    let response_start = ui.add_enabled(!state.recording_active, egui::Button::new("Start").fill(egui::Color32::from_rgb(90, 130, 190))
                      .rounding(egui::Rounding::same(10.0)));

                    if response_start.clicked() {
                        state.recording_active = true;
                        state.recording_start = Some(Instant::now());
                        tokio::spawn(async {
                            match run_recorder().await {
                                Ok(_) => println!("Recording started successfully."),
                                Err(e) => eprintln!("Failed to start recording: {:?}", e),
                            }
                        });
                    }

                    let response_stop = ui.add_enabled(state.recording_active, egui::Button::new("Stop").fill(egui::Color32::from_rgb(190, 90, 90))
                        .rounding(egui::Rounding::same(10.0)));

                    if response_stop.clicked(){
                        state.recording_active = false;
                        println!("Recording stopped.");
                    } 
                    

                    if state.recording_active {
                        if let Some(start) = state.recording_start {
                            let elapsed = start.elapsed();
                            let hours = elapsed.as_secs() / 3600;
                            let minutes = (elapsed.as_secs() % 3600) / 60;
                            let seconds = elapsed.as_secs() % 60;
                            ui.label(format!("{:02}:{:02}:{:02}", hours, minutes, seconds));
                            ctx.request_repaint();  // Request a repaint at the beginning of each frame
                        }
                    }
                });
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
