use crate::connection::Connection;
use mp_protocol::constants::BUF_CAP;
use mp_protocol::Opcode;
use std::io;
use std::mem;

struct Protocol;
impl Protocol {
    fn check_buf_cap(xs: &Vec<&str>, buf: &[u8]) -> bool {
        let str_bytes: usize = xs.iter().map(|s| s.len()).sum();
        1 + xs.len() * mem::size_of::<usize>() + str_bytes < buf.len()
    }

    pub(crate) fn add_file(paths: Vec<&str>, bytes: &mut [u8]) -> usize {
        println!("files: {:?}", paths);
        let op = Opcode::AddFile as u8;

        bytes[0] = op;
        let mut i = 1;

        if !Self::check_buf_cap(&paths, &bytes) {
            panic!("buffer overflow")
        }

        for path in paths {
            let strlen = path.len();
            let s = mem::size_of::<usize>();
            bytes[i..i + s].copy_from_slice(&strlen.to_be_bytes());
            i += s;
            bytes[i..i + strlen].copy_from_slice(path.as_bytes());
            i += strlen;
        }

        return i;
    }
}

impl Connection {
    pub(crate) fn add_files(&mut self, files: Vec<&str>) -> io::Result<()> {
        let mut buf = [0u8; BUF_CAP];
        let count = Protocol::add_file(files, &mut buf);
        println!("{:x?}", &buf[..count]);
        self.send(&buf[..count])?;

        let count = self.recv(&mut buf)?;
        println!("recv: {:x?}", &buf[..count]);
        Ok(())
    }
}
