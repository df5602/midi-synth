error_chain!{
    foreign_links {
        Usb(::libusb::Error);
        PortAudio(::portaudio::Error);
        MidiMessageRxChannel(::std::sync::mpsc::SendError<(::usb_midi::MidiMessage, ::midi_controller::MidiControllerType)>);
        MidiMessageTxChannel(::std::sync::mpsc::SendError<::usb_midi::MidiMessage>);
        SynthControlChannel(::std::sync::mpsc::SendError<::synth::dispatcher::SynthControl>);
        CtrlCError(::ctrlc::Error);
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
