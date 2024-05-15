import pyaudiowpatch as pyaudio
import numpy as np
import asyncio
from tapo import ApiClient, Color

# Function to control lighting based on audio
async def control_lighting(volume, tapo_username, tapo_password, ip_address):
    # Adjust lighting based on volume
    import numpy as np

    brightness = int(np.clip(volume * 20000, 1, 100))  # Scale volume to brightness percentage

    # Connect to Tapo device
    client = ApiClient(tapo_username, tapo_password)
    device = await client.l530(ip_address)

    # print("Turning device on...")
    # await device.on()

    # print("Waiting 2 seconds...")
    # await asyncio.sleep(2)

    print(f"Setting the brightness to {brightness}%...")
    await device.set_brightness(brightness)

    # Example: You can also change color based on other audio properties like frequency spectrum
    # For simplicity, this example only adjusts brightness based on volume

# Function to capture audio
def capture_audio():
    CHUNK = 1024
    FORMAT = pyaudio.paInt16
    CHANNELS = 1
    RATE = 44100

    p = pyaudio.PyAudio()
    try:
            # Get default WASAPI info
            wasapi_info = p.get_host_api_info_by_type(pyaudio.paWASAPI)
    except OSError:
        print("Looks like WASAPI is not available on the system. Exiting...")
        exit()

    # Get default WASAPI speakers
    default_speakers = p.get_device_info_by_index(wasapi_info["defaultOutputDevice"])
    

    if not default_speakers["isLoopbackDevice"]:
            for loopback in p.get_loopback_device_info_generator():
                """
                Try to find loopback device with same name(and [Loopback suffix]).
                Unfortunately, this is the most adequate way at the moment.
                """
                if default_speakers["name"] in loopback["name"]:
                    default_speakers = loopback
                    break
    else:
        print("Default loopback output device not found.\n\nRun `python -m pyaudiowpatch` to check available devices.\nExiting...\n")
        exit()

    stream = p.open(format=pyaudio.paInt16,
                channels=default_speakers["maxInputChannels"],
                rate=int(default_speakers["defaultSampleRate"]),
                frames_per_buffer=10,
                input=True,
                input_device_index=default_speakers["index"]
        )
    # print(p.get_default_output_device_info())
    # stream = p.open(format=FORMAT,
    #                 channels=CHANNELS,
    #                 rate=RATE,
    #                 input=True,
    #                 frames_per_buffer=CHUNK)

    print("* listening for audio")

    while True:
        data = np.frombuffer(stream.read(CHUNK), dtype=np.int16)
        # Calculate volume (root mean square) of the audio data
        volume = np.sqrt(np.mean(data**2)) / 32768  # Normalize to [0, 1]
        volume = np.nan_to_num(volume)

        # Call function to control lighting based on volume
        asyncio.run(control_lighting(volume, "amoghupadhyay007@gmail.com", "PR0wPQ3axzaI7T", "192.168.1.2"))

    print("* done listening")

    stream.stop_stream()
    stream.close()
    p.terminate()

# Start capturing audio
capture_audio()
