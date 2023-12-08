# aoc2023

## Day 1
Here we are again. Another year, another incomplete repo full of dubious code. Day 1 was well hard this year, wasn't it. Expected Frank Spencer, got Frankie Fraser. All them 1s and 2s and sixes and sevens and whatnot. Nasty. Tears yer fingernails out. 

The new question this year is to Github copilot or not to Github copilot? What I'm going to do is code the solution on my lonesome first, then give it to copilot and compare outputs. That should help me write more idiomatic Rust. Let's see how that goes ... 

... I just did that and copilot is an absolute beast - getting very close to the correct code by generating its solution based on a simple 2-line comment. 

## Day 2
This was also a bit trickier than your average day 2, or maybe I'm just getting old. Games and grabs and cubes and bags. Very colourful. I need a lie down.   

You know what? I think my code is actually OK for this. I'm getting into the habit of iter->filter->map-reduce rather than using foreach loops and it's really powerful. No error handling and the commenting is rubbish, but I'm just bashing the solution out ASAP. I should probably write some UT and error handling but life's too short. I'll save that for the day job. Anyway, part 1 took a while, but I had a decent abstration so part 2 was quite quick - just had to do a one-liner map/reducing over the games and grabs. Copilot came up with something very similar, but wrong. Would have saved me 45 minutes though (yes, I'm quite slow). 

## Day 3
Like a busy beaver I piled up logs and branches of code, wedging them between rocks of hard logic and tree roots of analysis, filling in the gaps with a paste of silty syntax and mulchy algorithm, damming up the turbulent torrent of gears and part numbers and reducing it to the calm, laminar trickle of clear solution. 

But my wood was rotten, my muddy cement washed away, and the dam burst, my friends, leaving me wet, homeless, and with nothing to my name but a contrived, unnecessary, and really quite bizarre beaver metaphor.

Then I wrote the blasted thing again from scratch. Second time lucky.

## Day 4
A walk in the park, if a slow and incompetent one. I inexplicably spent an inordinate amount of time wandering around in some kind of bewildered fugue state in which I hallucinated that a union was an intersection and an intersection was a union, much to the detriment of my initial attempts at a solution. A blast of fresh air and a strong coffee sorted me out and I soldiered on to eventual success.

## Day 5
What manner of hell is this? What's with all the "Ha ha I'll put the word 'mapping' in your head to make you think you need to unpack the ranges and put all the mappings into 7 HashMaps and chain the calls together but you won't have read the _real_ input just the _test_ input and it'll work on the _test_ input and you'll think it's _easy_ and that it'll also work on the _real_ input but the _real_ input has _really_ big numbers so it won't and you'll have to implement it all over again so it doesn't take the lifetime of the universe to run and then just when you've smugly solved part 1 after a rewrite I'll make the numbers _even bigger_ in part 2 so ha ha ho ho even with your nice solution to part 1 it'll still take the lifetime of the universe to run part 2 and you'll be sad and cry into your cold mug of coffee at your lonely desk you sad sack loser boo hoo boo hoo world's smallest violin"?

Well, I've got news for you, AOC. Sure there might be an elegant and speedy solution to be discovered, but I'm a brute and I wrote a brute force one. But you know what? I wrote it in _Rust_, and I _passed the -r flag_ to `cargo run`. You know what that means, right? Don't you? Huh? Yeah, course you do. It means _compiled_. It means _release_. It means _optimized_. 3 minutes and 33 seconds to chew through billions of meaningless calculations. Chew on that! Oh yeah!
