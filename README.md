# RPck file archiver

WIP implementation of an RPck file archiver.

## RPck

RPck is a very old and very exotic file compression format used on the Amiga
(and even there it is super exotic). Un-RPcking tools are available. However
compression tools are (to my knowledge) non existing.

This is a reimplementation of a file archiver derived from the ASM decompression code here:

http://wt.exotica.org.uk/files/others/xfd_RPck.lzx

RPck is a VERY simple format and won't really bring down the size like modern tools would.

If you genuinely need this project and we meet in real life somewhen I owe you a beer.

## Implementation

This is done in crappy rust. I like rust but I am bad at it. Very bad.

After running you can take `test.txt.rpck` and run `xfddecrunch test.txt.rpck` in the
shell of your Amiga. Voila.

## Tests

none 8]
