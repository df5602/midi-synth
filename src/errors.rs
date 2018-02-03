error_chain!{
    foreign_links {
        Usb(::libusb::Error);
    }

    errors {
        MidiControllerNotConnected {
            description("MIDI controller not connected"),
            display("MIDI controller not connected")
        }
    }
}
