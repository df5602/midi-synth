error_chain!{
    foreign_links {
        UsbError(::libusb::Error);
        PortAudioError(::portaudio::Error);
        MidiMessageRxChannelError(::std::sync::mpsc::SendError<(::usb_midi::MidiMessage, ::midi_controller::MidiControllerType)>);
        MidiMessageTxChannelError(::std::sync::mpsc::SendError<::usb_midi::MidiMessage>);
        SynthControlChannelError(::std::sync::mpsc::SendError<::synth::dispatcher::SynthControl>);
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
