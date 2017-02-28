extern crate byteorder;

use byteorder::{WriteBytesExt, BigEndian};
use std::io::{Result, Read, Write};
use std::fs::{File};

struct Header {
    magic: [u8; 4],
    orig_length: u32, // for some reasons the decrucnher asm checks if the first byte is a 0
                    // maybe it is a real u24?
    unknown: u32, // probably a checksum... again...checking explicitly for 0 in the asm :S
}

enum Chunk {
    Unique {
        data: Vec<u8>,
    },
    Repeated {
        byte: u8,
        num: usize,
    },
}

fn write_chunk<T: WriteBytesExt>(chunk: Chunk, target: &mut T) -> Result<()> {
    match chunk {
        Chunk::Unique { data } => {
            // argh...negating without the compiler being a dingens
            try!(target.write_i8((data.len() as i32 * -1) as i8));
            for c in data.iter() {
                try!(target.write_u8(*c));
            }
        },
        Chunk::Repeated { byte, num } => {
            try!(target.write_u8(num as u8 - 1));
            try!(target.write_u8(byte));
        }
    }
    Ok(())
}

fn write_header<T: WriteBytesExt>(length: u64, mut target: T) -> Result<usize> {
    let header = Header { magic: *b"RPck", orig_length: length as u32, unknown: 0};
    for c in header.magic.iter() {
        try!(target.write_u8(*c));
    }
    
    try!(target.write_u32::<BigEndian>(header.orig_length));
    try!(target.write_u32::<BigEndian>(header.unknown));
    Ok(1)
}

pub fn archive<T: Read>(rfile: T, size: u64, mut wfile: File) -> Result<()> {
    try!(write_header(size, &wfile));
    write_data(rfile, &mut wfile)
}

fn write_data<TR: Read, TW: Write>(readable: TR, mut writeable: &mut TW) -> Result<()> {
    let mut current: Option<Chunk> = None;
    for byte in readable.bytes() {
        // this is all looking super fuzzy :S n00bish code is n00bish :D
        let b = byte.unwrap();
        current = match current {
            None => Some(Chunk::Unique { data: [b].to_vec()}),
            Some(current) => {
                let (chunk, len) = match current {
                    Chunk::Unique { mut data } => {
                        let len = data.len();
                        if len > 1 && data.ends_with(&[b, b]) {
                            if len > 2 {
                                data.split_off(len - 2);
                                try!(write_chunk(Chunk::Unique {
                                    data: data,
                                }, &mut writeable));
                            }
                            (Chunk::Repeated {
                                byte: b,
                                num: 3,
                            }, 3)
                        } else {
                            data.push(b);
                            (Chunk::Unique {
                                data: data,
                            }, len + 1)
                        }
                    },
                    Chunk::Repeated { byte, mut num } => {
                        if b == byte {
                            num += 1;
                            (Chunk::Repeated {
                                byte: b,
                                num: num,
                            }, num)
                        } else {
                            try!(write_chunk(current, &mut writeable));
                            (Chunk::Unique { data: [b].to_vec()}, 1)
                        }
                    }
                };
                
                if len == 128 {
                    try!(write_chunk(chunk, &mut writeable));
                    None
                } else {
                    Some(chunk)
                }
            }
        };
    }
    if current.is_some() {
        try!(write_chunk(current.unwrap(), &mut writeable));
    }
    Ok(())
}

#[cfg(test)]
mod test;