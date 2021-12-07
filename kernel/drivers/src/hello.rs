use kernel::syscall::{SyscallDriver, CommandReturn};
use kernel::process::{Error, ProcessId};
use kernel::{ ErrorCode, debug};
use core::cell::Cell;

pub const DRIVER_NUM: usize = 0xbabe;

pub struct Hello {
    n : Cell<u8>,
    call_ctr : Cell<usize>
}


impl Hello {
    pub fn new () -> Hello {
        Hello{n: Cell::new(1), call_ctr: Cell::new(0)}
    }
}

impl SyscallDriver for Hello {
    fn command(
        &self, /*this is not mutable boss*/
        command_num: usize,
        r2: usize,
        _r3: usize,
        _process_id: ProcessId,
    ) -> CommandReturn {
        match command_num {
            0 => CommandReturn::success(),
            1 => {
                self.n.set(r2 as u8);
                debug!("Hello {}", self.n.get());
                CommandReturn::success()
            },
            2 => {
                self.call_ctr.set(self.call_ctr.get()+1);
                CommandReturn::success()
            },
            3 => {
                let val = self.call_ctr.get();
                if val == 0 {
                    debug!("Ouch, too many decrements");
                    // I wanna panic or exit cu 1
                    // ptr la kernel
                    //  self.kernel
                    // .process_each_capability(&self.capability, |proc| 
                        // cauti dupa processId lol
                    CommandReturn::failure(ErrorCode::INVAL)
                } else {
                    self.call_ctr.set(self.call_ctr.get()-1);
                    CommandReturn::success()
                }
                
            },
            5 => {
                CommandReturn::success_u32(self.call_ctr.get()as u32)
            }
        

            _ => CommandReturn::failure (ErrorCode::NOSUPPORT)
        }
    }



    fn allocate_grant(&self, _process_id: ProcessId) -> Result<(), Error> {
        debug!("grant");
        Ok(())
    }
}