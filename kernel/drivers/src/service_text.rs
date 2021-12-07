use kernel::hil::led::Led;
use kernel::syscall::{SyscallDriver, CommandReturn};
use kernel::{debug, ErrorCode, ProcessId};
use kernel::hil::time::{Alarm, AlarmClient, ConvertTicks};
use kernel::grant::Grant;
use kernel::processbuffer::ReadOnlyProcessBuffer;

pub const DRIVER_NUM:usize = 0xa0002;

#[derive(Default)]
pub struct AppData {
    buffer: ReadOnlyProcessBuffer
}

const DIGITS: [u32; 10] = [
    // 0
    0b11111_10011_10101_11001_11111,
    // 1
    0b00100_01100_00100_00100_01110,
    // 2
    0b11110_00001_01110_10000_11111,
    // 3
    0b11110_00001_11110_00001_11110,
    // 4
    0b10000_10000_10100_11111_00100,
    // 5
    0b11111_10000_11110_00001_11110,
    // 6
    0b11111_10000_11111_10001_11111,
    // 7
    0b11111_00001_00010_00100_00100,
    // 8
    0b11111_10001_11111_10001_11111,
    // 9
    0b11111_10001_11111_00001_11111,
];
pub struct DotsText<'a, L: Led, A: Alarm<'a>> {
    leds: &'a [& 'a L; 25],
    alarm: &'a A,
    app_data: Grant<AppData, 1>, // tip de date, nr de subscribe-uri
    // grantu e doar accesatorul si alocatorul pt memorie
    buffer: TakeCell<'static, [u8]>,
    client: OptionalCell<&'a dyn TextScreenClient>

}

impl <'a, L: Led, A: Alarm<'a>> DotsText<'a, L, A> {
    pub fn new(
        leds: &'a [& 'a L; 25],
        alarm: &'a A,
        app_data: Grant<AppData, 1>) -> DotsText<'a, L, A> {
        // if leds.len() != 25 {
        //     panic!("DotsText needs 25 LEDs, youy supplied {}",leds.len())
        // }
        DotsText{
            leds,
            alarm,
            app_data,
            client: OptionalCell::empty()
        }
    }

    pub fn setup_alarm(&self) {
        debug!("fired");
        if self.supplied_buffer.is_some(){
            self.supplied_buffer.take().map();
            self.client.map(|client|
                client.write_complete(bufffer, self.print_len.get(), Ok(()))
            );
        }

        self.alarm.set_alarm(self.alarm.now(), self.alarm.ticks_from_ms(1000));
    }

    fn display(&self, digit: char) {
        let digit_index = digit as usize -'0' as usize;
        let crt_digit = DIGITS[digit_index];
        for idx in 0..25 {
            let bit = (crt_digit >> (24 -idx)) & 0x1;
            if bit == 1 {
                self.leds[idx].on();
            } else {
                self.leds[idx].off();
            }
        }
    }
}


impl<'a,L: Led, A:Alarm<'a>> SyscallDriver for DotsText<'a, L, A> {
    fn command(
        &self, /*this is not mutable boss*/
        command_num: usize,
        r2: usize,
        _r3: usize,
        _process_id: ProcessId,
    ) -> CommandReturn {
        match command_num{
            0 => CommandReturn::success(),
            // print digit
            1 => {
                // char::from_u32(r2 as u32) 
                let digit = r2 as u8 as char;
                if digit >='0' && digit <= '9' {
                    CommandReturn::success()
                } else  {
                    CommandReturn::failure(ErrorCode::INVAL)
                }
            }
            _ => CommandReturn::failure(ErrorCode::INVAL)
        }
    }

    fn allow_readonly(
        &self,
        process_id: ProcessId,
        allow_num: usize,
        mut buffer: ReadOnlyProcessBuffer,
    ) -> Result<ReadOnlyProcessBuffer, (ReadOnlyProcessBuffer, ErrorCode)> {
        match allow_num{
            0 => {
                let res = self.app_data.enter(process_id, |data, _upcalls| {
                    // apleata daca se poate intra in grant
                    core::mem::swap(&mut data.buffer,&mut buffer);
                });
                match res {
                    Ok(_) => Ok(buffer),
                    Err(error) => Err((buffer, error.into()))
                }
            }
            _ => Err((buffer, ErrorCode::NOSUPPORT))
        }
       
    }

    fn allocate_grant(
        &self,
        processid: ProcessId
    ) -> Result<(), kernel::process::Error> {
        // tre sa alocam chestii boss
        self.app_data.enter(processid, |_,_|{
            // ce mai poti sa faci aici?
            debug!("alloc grant")})
        // mesaj de eroare doar daca trace syscalls e true
    }
}

impl <'a, L: Led, A: Alarm<'a>> AlarmClient for DotsText<'a,L,A> {
    fn alarm(&self) {
        // debug!("fired alarm for dots text");
        // self.display('0');
        self.setup_alarm();
    }
}


impl TextScreen for DotsText {
    fn set_client(&self, client: Option<&'a dyn TextScreenClient>){
        // trebuie sa tinem minte clientul
        if let Some(client) = client {
            self.client.replace(client);
        } else {
            // a none means they want to delete the client
            self.client.clear();
        }
        
    }

    /// Returns a tuple (width, height) with the resolution of the
    /// screen that is being used. This function is synchronous as the
    /// resolution is known by the driver at any moment.
    ///
    /// The resolution is constant.
    fn get_size(&self) -> (usize, usize);

    /// Sends a write command to the driver, and the buffer to write from
    /// and the len are sent as arguments. When the `write` operation is
    /// finished, the driver will call the `write_complete()` callback.
    ///
    /// Return values:
    /// - `Ok(())`: The write command is valid and will be sent to the driver.
    /// - `BUSY`: The driver is busy with another command.
    fn print(
        &self,
        supplied_buffer: &'static mut [u8],
        len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])>{ 
        self.buffer.map_or(Err((ErrorCode::NOMEM),supplied_buffer));
        if self.buffer.is_some() {

        }

    }

    /// Sends to the driver a command to turn on the display of the screen.
    /// When finished, the driver will call the `command_complete()` callback.
    ///
    /// Return values:
    /// - `Ok(())`: The command is valid and will be sent to the driver.
    /// - `BUSY`: Another command is in progress.
    fn display_on(&self) -> Result<(), ErrorCode>{
        // wrong
        self.client.map(|client| {
            client.command_complete(Ok(()));
        });
        Ok(());
    }

    /// Sends to the driver a command to turn off the display of the screen.
    /// When finished, the driver will call the `command_complete()` callback.
    ///
    /// Return values:
    /// - `Ok(())`: The command is valid and will be sent to the driver.
    /// - `BUSY`: Another command is in progress.
    fn display_off(&self) -> Result<(), ErrorCode>;

    /// Sends to the driver a command to clear the display of the screen.
    /// When finished, the driver will call the `command_complete()` callback.
    ///
    /// Return values:
    /// - `Ok(())`: The command is valid and will be sent to the driver.
    /// - `BUSY`: Another command is in progress.
    fn clear(&self) -> Result<(), ErrorCode>;
}
