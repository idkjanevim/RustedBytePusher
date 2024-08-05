const RAM_SIZE: usize = 2usize.pow(20) * 16;
pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 256;
pub const NUM_KEYS: usize = 16;
pub const SAMPLE_RATE: usize = 0x100 * 60;

//Address	Bytes	Description
//0	         2	    Keyboard state. Key X = bit X.
//2	         3	    The program counter is fetched from this address at the beginning of each frame.
//5	         1	    A value of ZZ means: pixel(XX, YY) is at address ZZYYXX.
//6	         2	    A value of XXYY means: audio sample ZZ is at address XXYYZZ.

enum MemoryMap {
    KeyboardState = 0x0,
    ProgramCounter = 0x2,
    PixelData = 0x5,
    _SoundData = 0x6,
}
pub struct Emulator {
    ram: Vec<u8>,
    screen: Vec<Vec<u8>>,
    keys: [bool; NUM_KEYS],
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            ram: vec![0; RAM_SIZE],
            screen: vec![vec![0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            keys: [false; NUM_KEYS],
        }
    }

    pub fn reset(&mut self) {
        self.ram = vec![0; RAM_SIZE];
        self.screen = vec![vec![0; SCREEN_WIDTH]; SCREEN_HEIGHT];
        self.keys = [false; NUM_KEYS];
    }

    fn addr_at(&self, offset: usize) -> usize {
        let addr: usize = ((self.ram[offset] as usize) << 16
            | (self.ram[offset + 1] as usize) << 8
            | self.ram[offset + 2] as usize) as usize;
        addr
    }

    pub fn load(&mut self, data: &[u8]) {
        for i in 0..data.len() {
            self.ram[i] = data[i];
        }
    }

    pub fn get_screen(&self) -> &Vec<Vec<u8>> {
        &self.screen
    }

    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    pub fn cycle(&mut self) {
        // Wait for the next timer tick (60 ticks are generated per second).
        // Done in frontend

        // Poll the keys and store their states as a 2-byte value at address 0.
        self.ram[MemoryMap::KeyboardState as usize] = ((self.keys[15] as u8) << 7)
            | ((self.keys[14] as u8) << 6)
            | ((self.keys[13] as u8) << 5)
            | ((self.keys[12] as u8) << 4)
            | ((self.keys[11] as u8) << 3)
            | ((self.keys[10] as u8) << 2)
            | ((self.keys[9] as u8) << 1)
            | self.keys[8] as u8;
        self.ram[(MemoryMap::KeyboardState as usize) + 1] = ((self.keys[7] as u8) << 7)
            | ((self.keys[6] as u8) << 6)
            | ((self.keys[5] as u8) << 5)
            | ((self.keys[4] as u8) << 4)
            | ((self.keys[3] as u8) << 3)
            | ((self.keys[2] as u8) << 2)
            | ((self.keys[1] as u8) << 1)
            | self.keys[0] as u8;

        // Fetch the 3-byte program counter from address 2, and execute exactly 65536 instructions.
        let mut pc = self.addr_at(MemoryMap::ProgramCounter as usize) as usize;

        for _ in 0..65536 {
            // B = A
            let source_addr = self.addr_at(pc);
            let dest_addr = self.addr_at(pc + 3);
            let jump_addr = self.addr_at(pc + 6);

            if source_addr == dest_addr && jump_addr == pc {
                break;
            }

            let value = self.ram[source_addr];
            self.ram[dest_addr] = value;
            // JMP C
            pc = jump_addr;

            //println!("{}", pc);
        }
        // Send the 64-KiB pixeldata block designated by the byte value at address 5 to the display device.
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let pixel_addr = ((self.ram[MemoryMap::PixelData as usize] as usize) << 16
                    | (y as usize) << 8
                    | x as usize) as usize;
                let pixel_data = self.ram[pixel_addr];
                self.screen[y][x] = pixel_data;
            }
        }

        //Send the 256-byte sampledata block designated by the 2-byte value at address 6 to the audio device.
        // I tried implementing sound with no success :) I guess i just dont understand how sound works ? idk

        // Go back to step 1
    }
}
