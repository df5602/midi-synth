error_chain!{
    foreign_links {
        Usb(::libusb::Error);
    }
}
