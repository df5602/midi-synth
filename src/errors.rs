error_chain!{
    foreign_links {
        Usb(::libusb::Error);
        MidiMessageChannel(::std::sync::mpsc::SendError<::usb_midi::MidiMessage>);
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
