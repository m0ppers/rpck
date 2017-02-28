use byteorder::{WriteBytesExt, BigEndian};
use std::io::{Result, Read};

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
        num: u8,
    },
}

fn write_chunk<T: WriteBytesExt>(chunk: Chunk, mut target: T) -> Result<()> {
    match chunk {
        Chunk::Unique { data } => {
            // argh...negating without the compiler being a dingens
            try!(target.write_i8((data.len() as i32 * -1) as i8));
            for c in data.iter() {
                try!(target.write_u8(*c));
            }
        },
        Chunk::Repeated { byte, num } => {
            try!(target.write_u8(num - 1));
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

pub fn archive<T: Read>(rfile: T, size: u64, wfile: File) -> Result<usize> {
    try!(write_header(size, &wfile));

    let mut current: Option<Chunk> = None;

    for byte in rfile.bytes() {
        // this is all looking super fuzzy :S n00bish code is n00bish :D
        let b = byte.unwrap();
        current = match current {
            None => Some(Chunk::Unique { data: [b].to_vec()}),
            Some(current) => {
                match current {
                    // pyramid, pyramid!
                    Chunk::Unique { mut data } => {
                        if b == *data.last().unwrap() {
                            // if it is unique (data[0] != data[1]) it must be at least 2 bytes long :S
                            // (that's what I found out from example files)
                            match data.len() {
                                1 => {
                                    Some(Chunk::Repeated {
                                        byte: b,
                                        num: 2,
                                    })
                                },
                                2 => {
                                    data.push(b);
                                    Some(Chunk::Unique {
                                        data: data,
                                    })
                                },
                                127 => {
                                    try!(write_chunk(Chunk::Unique {
                                        data: data,
                                    }, &wfile));
                                    Some(Chunk::Unique { data: [b].to_vec()})
                                },
                                _ => {
                                    let mut num = 1;
                                    while data.len() != 2 && *data.last().unwrap() == b {
                                        num += 1;
                                        data.pop();
                                    }
                                    if data.len() > 0 {
                                        try!(write_chunk(Chunk::Unique {
                                            data: data,
                                        }, &wfile));
                                    }
                                    Some(Chunk::Repeated {
                                        byte: b,
                                        num: num,
                                    })
                                }
                            }
                        } else {
                            data.push(b);
                            Some(Chunk::Unique {
                                data: data,
                            })
                        }
                    },
                    Chunk::Repeated { byte, mut num } => {
                        if b == byte {
                            num += 1;
                            let result = Chunk::Repeated {
                                byte: b,
                                num: num,
                            };
                            if num < 127 {
                                Some(result)
                            } else {
                                try!(write_chunk(result, &wfile));
                                None
                            }
                        } else {
                            try!(write_chunk(current, &wfile));
                            Some(Chunk::Unique { data: [b].to_vec()})
                        }
                    }
                }
            }
        }
    }
    if current.is_some() {
        try!(write_chunk(current.unwrap(), wfile));
    }
    Ok(1)
}