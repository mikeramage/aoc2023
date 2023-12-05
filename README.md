# aoc2023

## Day 1
Here we are again. Another year, another incomplete repo full of dubious code. Day 1 was well hard this year, wasn't it. Expected Frank Spencer, got Frankie Fraser. All them 1s and 2s and sixes and sevens and whatnot. Nasty. Tears yer fingernails out. 

The new question this year is to Github copilot or not to Github copilot? What I'm going to do is code the solution on my lonesome first, then give it to copilot and compare outputs. That should help me write more idiomatic Rust. Let's see how that goes ... 

... I just did that and copilot is an absolute beast - getting very close to the correct code by generating its solution based on a simple 2-line comment. 

## Day 2
This was also a bit trickier than your average day 2, or maybe I'm just getting old. Games and grabs and cubes and bags. Very colourful. I need a lie down.   

You know what? I think my code is actually OK for this. I'm getting into the habit of iter->filter->map-reduce rather than using foreach loops and it's really powerful. No error handling and the commenting is rubbish, but I'm just bashing the solution out ASAP. I should probably write some UT and error handling but life's too short. I'll save that for the day job. Anyway, part 1 took a while, but I had a decent abstration so part 2 was quite quick - just had to do a one-liner map/reducing over the games and grabs. Copilot came up with something very similar, but wrong. Would have saved me 45 minutes though (yes, I'm quite slow). 