use std::{
    env, 
    thread, 
    time::{Duration, Instant},
};
use tapo::{
    requests::{Color, LightingEffect, LightingEffectType},
    ApiClient,
};
use aubio::{Pitch, Source, AudioIn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize smart lighting control
    let tapo_username = "amoghupadhyay007@gmail.com"; 
    let tapo_password = "PR0wPQ3axzaI7T";
    let ip_address = "192.168.1.2";

    let device = ApiClient::new(tapo_username, tapo_password)
        .l930(ip_address)
        .await?;

    // Initialize audio capture
    let audio_in = AudioIn::new("default", 44100, 256)?;
    let mut pitch = Pitch::new(Pitch::default_mode(), 4096, 256, 44100)?;

    // Start main loop for real-time audio analysis and lighting control
    let mut last_update = Instant::now();
    loop {
        // Capture audio input
        let mut samples = vec![0.0; 256];
        audio_in.get_samples(&mut samples)?;

        // Analyze audio pitch (frequency)
        let pitch_freq = pitch.do_(samples.as_slice());

        // Map pitch frequency to hue and saturation values
        let hue = (pitch_freq as u16) % 360;
        let saturation = 100;

        // Control smart lighting based on mapped parameters
        device.set_hue_saturation(hue, saturation).await?;

        // Limit update rate to prevent overwhelming the lighting device
        let elapsed = last_update.elapsed();
        if elapsed < Duration::from_millis(100) {
            thread::sleep(Duration::from_millis(100) - elapsed);
        }
        last_update = Instant::now();
    }
}
