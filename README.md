# wordol_solver

A primitive solver for WORDOL(https://app.39m.ltd/games/wordol/) inspired by Wordle.

# TODO List

The current version takes the output of WORDOL inside the source code. Change it to take it via standard input or external json file.

Rethink how to find eligible units.

The enum IncludedInUnit all contain an IdolName. Maybe a struct is better.

Improve how to deal with empty slots. Currently green empty slots must be input as SameType, not Included.

Rust is probably overkill. Maybe rewrite it in another language?

# Probably unoptimal and buggy

Any feedback is welcome. If you find a case where the answer is not included in the result or an ineligible unit is included, please open an issue with a screenshot.
