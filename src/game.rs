

pub fn run(mut gba: agb::Gba) -> !{
    //Get/init hardware controls
    let vblank = agb::interrupt::VBlank::get();

    loop{
        

        //Wait for vblank
        vblank.wait_for_vblank();
    }
}