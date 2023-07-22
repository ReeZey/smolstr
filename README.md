# smolstr - 5 bit encoding

smolstr is a concept i had once about space saving by storing text in 5 bits instead of the normal 8 bits which unicode is encoded in.  
this is nothing more than a test, feel free to use it anywhere, its open source and should be pretty straight forward  

from my own testing this saves on average a ratio of 3/2 in direct size comparison  

if you want to test this encoder, i've left a tool in the [releases](https://github.com/ReeZey/smolstr/releases/latest) tab which anyone who has touched an command prompt should be able to figure out, also to some extent the user could just drag and drop txt files ontop of exe

# file format
The file format is built with three blocks  
#### "THE MAGIC NUMBER" -> [4 bytes, always "smol" in unicode]  
#### "ALPHABET" -> [32 bytes, encoded as raw unicode]  
#### "DATA" -> [? bytes, always padded to u128 (16 bytes, unsigned 128 integer)]  

# 

Reading the file format is pretty simple, one could skip first 36 bytes, but its recommended to read what alphabet the file is using otherwise it will text will be jibberish  
this will require some bitshifting, so atleast basic understanding on how bitshifting works.  

### writing  

everytime the current u128 runs out of bits (every 25 characters, 5 * 16 = 125), the current u128 is pushed to the output file and another u128 starts, rinse and repeat until there is no data left and the last u128 just gets pushed, so the last u128 always gets padded.

### reading  

reading is the same as writing but in reverse, so instead of writing you read u128, go through every 5 bits until there are no bits to go through then drop current u128 and read the next u128. when the end of the file is reached you are done!


# what is the 1 and 2 in standard alphabet?
- pushing a 1 will toggle "number mode" all character read during this time will get the index in the alphabet to result in a number a = 1, b = 2, etc... example: "1aabb1" will result the number "1122"
- pushing a 2 will either make the following letter uppercase or if the following character is a space it will put a newline character (\n)*  

*this might be changed in the future

# rules
- charset can be changed **BUT** charset must be EXACTLY 32 characters. *special* characters (space, 1, 2) **CANNOT** be changed   
they should stay the same cause they have different meaning), default is " abcdefghijklmnopqrstuvwxyz.!?12"
