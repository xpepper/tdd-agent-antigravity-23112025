# The Goose Game Kata

Goose game is a game where two or more players move pieces around a track by rolling a die. The aim of game is to reach square number sixty-three before any of the other players and avoid obstacles.

## General requirements

- Implement this as a library that can be used to build a text-based game interface
- Focus on clean, testable code with clear separation of concerns
- Use idiomatic Rust patterns and error handling

## Features

### 1. Add players

As a player, I want to add me to game so that I can play.

**Scenarios:**

1. Add Player
   - If there is no participant
   - user writes: "add player Pippo"
   - system responds: "players: Pippo"
   - user writes: "add player Pluto"
   - system responds: "players: Pippo, Pluto"

2. Duplicated Player
   - If there is already a participant "Pippo"
   - user writes: "add player Pippo"
   - system responds: "Pippo: already existing player"

### 2. Move a player

As a player, I want to move marker on the board to make the game progress

**Scenarios:**

1. Start
   - If there are two participants "Pippo" and "Pluto" on space "Start"
   - user writes: "move Pippo 4, 2"
   - system responds: "Pippo rolls 4, 2. Pippo moves from Start to 6"
   - user writes: "move Pluto 2, 2"
   - system responds: "Pluto rolls 2, 2. Pluto moves from Start to 4"
   - user writes: "move Pippo 2, 3"
   - system responds: "Pippo rolls 2, 3. Pippo moves from 6 to 11"

### 3. Win

As a player, I win game if I land on space "63"

**Scenarios:**

1. Victory
   - If there is one participant "Pippo" on space "60"
   - user writes: "move Pippo 1, 2"
   - system responds: "Pippo rolls 1, 2. Pippo moves from 60 to 63. Pippo Wins!!"

2. Winning with the exact dice shooting
   - If there is one participant "Pippo" on space "60"
   - user writes: "move Pippo 3, 2"
   - system responds: "Pippo rolls 3, 2. Pippo moves from 60 to 63. Pippo bounces! Pippo returns to 61"

### 4. The game throws the dice

As a player, I want game throws dice for me to save effort

**Scenarios:**

1. Dice roll
   - If there is one participant "Pippo" on space "4"
   - assuming that the dice get 1 and 2
   - when user writes: "move Pippo"
   - system responds: "Pippo rolls 1, 2. Pippo moves from 4 to 7"

### 5. Space "6" is "The Bridge"

As a player, when I get to the space "The Bridge", I jump to space "12"

**Scenarios:**

1. Get to "The Bridge"
   - If there is one participant "Pippo" on space "4"
   - assuming that the dice get 1 and 1
   - when user writes: "move Pippo"
   - system responds: "Pippo rolls 1, 1. Pippo moves from 4 to The Bridge. Pippo jumps to 12"

### 6. If you land on "The Goose", move again

As a player, when I get to a space with a picture of "The Goose", I move forward again by the sum of the two dice rolled before

The spaces 5, 9, 14, 18, 23, 27 have a picture of "The Goose"

**Scenarios:**

1. Single Jump
   - If there is one participant "Pippo" on space "3"
   - assuming that the dice get 1 and 1
   - when user writes: "move Pippo"
   - system responds: "Pippo rolls 1, 1. Pippo moves from 3 to 5, The Goose. Pippo moves again and goes to 7"

2. Multiple Jump
   - If there is one participant "Pippo" on space "10"
   - assuming that the dice get 2 and 2
   - when user writes: "move Pippo"
   - system responds: "Pippo rolls 2, 2. Pippo moves from 10 to 14, The Goose. Pippo moves again and goes to 18, The Goose. Pippo moves again and goes to 22"

### 7. Prank (Optional Step)

As a player, when I land on a space occupied by another player, I send him to my previous position so that game can be more entertaining.

**Scenarios:**

1. Prank
   - If there are two participants "Pippo" and "Pluto" respectively on spaces "15" and "17"
   - assuming that the dice get 1 and 1
   - when user writes: "move Pippo"
   - system responds: "Pippo rolls 1, 1. Pippo moves from 15 to 17. On 17 there is Pluto, who returns to 15"

## Implementation Notes

- The game board has spaces numbered from 0 (Start) to 63
- Players take turns moving based on dice rolls
- Special spaces: The Bridge (6), The Goose spaces (5, 9, 14, 18, 23, 27)
- Players bounce back if they overshoot space 63
- Consider using a Game struct to manage state and Player struct for individual players