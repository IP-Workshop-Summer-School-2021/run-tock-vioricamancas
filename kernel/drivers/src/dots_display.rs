// use kernel::hil::led::Led;
// use kernel::syscall::{SyscallDriver, CommandReturn};
// use kernel::{ErrorCode, ProcessId};

// pub const DRIVER_NUM:usize = 0xa0001;

// const DIGITS: [u32;10] = [
//     0b11111_10011_10101_11001_11111,
//     0b11111_10011_10101_11001_11111,
//     0b11111_10011_10101_11001_11111,
//     0b11111_10011_10101_11001_11111,
//     0b11111_10011_10101_11001_11111,
//     0b11111_10011_10101_11001_11111,
//     0b11111_10011_10101_11001_11111,
//     0b11111_10011_10101_11001_11111,
//     0b11111_10011_10101_11001_11111,
//     0b11111_10011_10101_11001_11111,
// ];

// pub struct DotsDisplay<'a, L: Led> {
//     leds: &'a [&'a L; 25]
// }

// impl <'a> DotsDisplay<'a, L: Led> {
//     pub fn new(leds: &'a [& 'a L; 25]) -> DotsDisplay<'a> {
//         // if leds.len() != 25 {
//         //     panic!("DotsDisplay needs 25 LEDs, youy supplied {}",leds.len())
//         // }
//         DotsDisplay{
//             leds
//         }
//     }

//     fn display(&self, digit: char) {
//         let digit_index = digit as usize -'0' as usize;
//         let crt_digit = DIGITS[digit_index];
//         for idx in 0..25 {
//             let bit = (crt_digit >> (24 -idx)) & 0x1;
//             if bit == 1 {
//                 self.leds[idx].on();
//             } else {
//                 self.leds[idx].off();
//             }
//         }
//     }
// }

// impl<'a> SyscallDriver for DotsDisplay<'a, L: Led> {
//     fn command(
//         &self, /*this is not mutable boss*/
//         command_num: usize,
//         r2: usize,
//         r3: usize,
//         process_id: ProcessId,
//     ) -> CommandReturn {
//         match command_num{
//             0 => CommandReturn::success(),
//             // print digit
//             1 => {
//                 // char::from_u32(r2 as u32) 
//                 let digit = r2 as u8 as char;
//                 if digit >='0' && digit <= '9' {
//                     CommandReturn::success()
//                 } else  {
//                     CommandReturn::failure(ErrorCode::INVAL)
//                 }
//             }
//             _ => CommandReturn::failure(ErrorCode::INVAL)
//         }
//     }

//     fn allocate_grant(
//         &self,
//          _: ProcessId
//     ) -> Result<(), kernel::process::Error> {
//        Ok(())
//     }
// }

