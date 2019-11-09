# WebeID
Unique ID generator based on Snowflake, but for us wizards


## A 64bit unique ID
### Definition
5 bytes - time in milliseconds since custom epoch.  (total of 34.84 years)
1 byte - node id (total of 256 nodes)
2 bytes - incrementing sequence number

## Assumptions made for this ID
 - Technology will quickly advance to the point where 128bit IDs (like GUID) will be acceptably performant in extremely large database tables.
   - This means that we don't need large time range and we likely won't ever make use of the "future use" bit that Snowflake and Sonyflake have.
 - The developer using this is a hobbyist or small production
   - Very likely won't scale up ID creation to 1000's of distributed machines
   - Use of these IDs is unlikely to outlive the max time frame

## Advantages
 - Supports fast id creation, up to 65536 id's per millisecond.
   - I wanted this number to be really big, since my slightly overclocked 3.8GHZ i5-7600 on Pop-OS 19.10 (Ubuntu derivative) can create a new ID in about 500nanoseconds.
     - That's about 2000 IDs per millisecond. So plenty of room to grow as CPUs get faster *without needing more nodes*.
 - Data sections are divisible on common 8bit byte boundaries,
   - Parsing sections does not require bit operations. Convert to big-endian byte array with ".to_be_bytes()" and then use the slice indexes 0-4, 5, 6-7
   - Since we're focusing on a high rate of id generation PER NODE, the first 6 bytes become highly repeated and therefore compressable.

## Notes
 - I could have chosen intervals of 10ms like Sony's Sonyflake, which would put the total timeframe up to 348.4 years
   - Higher sequences per 10ms means that IDs would be even more compressable.
   - However; My bet is that processors in the future, even in just a few years, will be able to blow past the max sequence range in 10ms (causing duplicates).