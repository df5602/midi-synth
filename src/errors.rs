error_chain!{
    foreign_links {
        Usb(::libusb::Error);
        PortAudio(::portaudio::Error);
        MidiMessageChannel(::std::sync::mpsc::SendError<::usb_midi::MidiMessage>);
        F32Channel(::std::sync::mpsc::SendError<f32>);
    }

    errors {
        MidiControllerNotConnected {
            description("MIDI controller not connected"),
            display("MIDI controller not connected")
        }

        MidiOperationNotSupported {
            description("MIDI operation not supported"),
            display("MIDI operation not supported")
        }
    }
}
