# Chutes and Ladders game analysis

My daughter enjoys playing the old game _Chutes and Ladders_. It's a simple, entirely-luck-based game where the players move towards the goal (space 100), but can be forced on detours via predetermined chutes and ladders on the board. For example, you can be very close to space 100, but then land on space 98 which will send you back down to space 78.

After sitting through some mind-bogglingly boring games with my daughter, I got to thinking about how many turns the average Chutes and Ladders game is, and all sorts of similar questions. Since Chutes and Ladders is purely based on a fixed set of chutes and ladders, plus the roll of the die, it should be fairly straightforward to simulate a game automatically. By simulating enough games, I could perform some statisical analysis on the results and determine things like:

* How many turns is the average game?
* What percentage of games take X turns or longer?

## Simulation

This repo contains a Rust-based Chutes and Ladders simulator. It can simulate a single player working its way across the board, taking chutes/ladders, and reaching the end goal. It keeps track of the turns that a game takes, the number of spaces traversed, and the number of chutes/ladders taken.

It's set up to simulate 1 million games, then dump the results of all the games to a CSV file.

Execute `cargo run` to run the simulator and save the results in `runs.csv`.

## Analysis

I have a basic Jupyter notebook (`Exploration.ipynb`) which uses pandas to analyze the results of the simulation and graph the results. I'm new to pandas and my stats knowledge is rusty, so my analysis work is pretty scrappy.
