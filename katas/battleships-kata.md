# [Battleships kata](https://katalyst.codurance.com/battleships)

## Introduction

This version of the classic game has three ships:

* **Carrier**: 4 cells - represented on a board with 'c'
* **Destroyer**: 3 cells - represented on a board with 'd'
* **Gun Ship**: 1 cell - represented on a board with 'g'

Create a program that allows the user to specify commands for playing battleship. The commands available are:

* `addPlayer`: Creates a player for the game.
    - the user should specify the location of their ships
* `start`: Starts a new game with a fleet of ships placed at user's defined (x,y) coordinates.
* `endTurn`: Ends the player's turn.
* `print`: This command will print out the game board (Exhibit A):

```
    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
   0|   |   |   |   |   |   |   |   |   |   |
   1|   |   |   |   |   |   |   |   |   |   |
   2|   |   |   |   |   |   |   | g |   |   |
   3|   |   | d | d | d |   |   |   |   |   |
   4|   |   |   |   |   |   | g |   | c |   |
   5|   |   |   |   |   |   |   |   | c |   |
   6|   |   |   |   |   |   |   |   | c |   |
   7|   | g |   |   |   | d |   |   | c |   |
   8|   |   |   |   |   | d |   |   |   |   |
   9|   |   |   |   |   | d |   |   |   | g |
```
* `fire`: Launches a torpedo at the given (x,y) coordinates.
  - If the (x,y) coordinate is sea then the position will be marked with 'o'.
  - If the (x,y) coordinate is a ship then the position will be marked with 'x'.
  - If a ship has all cells hit then a message should print notifying the player the ship has sunk.- when all ships have been sunk the game ends

## Rules

* When all ships have been sunk the game ends
* When the game is finished, the game should display a battle report with the number of shots fired by each player, including hit/miss ship sunk.
* Ships sunk should show the lowest possible coordinate for the given ship, for example:    - a horizontal destroyer on grid reference (2,3), (3,3) and (4,3), but when reporting the sinking of the ship, you only need to reference the first coordinate.
  - A horizontal destroyer on grid reference (2,3), (3,3) and (4,3), but when reporting the sinking of the ship, you only need to reference the first coordinate.
  - A vertical destroyer on ref (5,5), (5,6) and (5,7) but you'll only need to reference (5,5) when reporting.

Using Exhibit A above, here is a battle report based on the ship positions:

```
[ Player1
 Total shots: 23
 Misses: 15
 Hits: 8
 Ships Sunk: [
	Gunship: (1,7),
	Gunship: (9,9),
	Gunship: (7, 2),
	Destroyer (2,3) ]
```

```
    | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
   0|   |   | o |   |   |   |   |   |   |   |
   1|   | o |   | o |   |   |   |   |   |   |
   2|   | o |   |   |   |   |   | X |   |   |
   3|   |   | X | X | X |   |   |   |   |   |
   4|   | o |   |   |   |   | g |   | c |   |
   5|   | o | o |   |   | o |   |   | c |   |
   6|   |   |   | o |   |   | o |   | c |   |
   7|   | X |   |   |   | d |   |   | x |   |
   8|   |   | o | o |   | d |   |   | o |   |
   9|   |   |   |   | o | x | o |   |   | X |
```

Sunk ships have all their coordinates marked with an uppercase X and hit cells have a lower case x where they were not sunk.

### Restrictions

* Complete using outside-in TDD approach
* Each player has maximum:
  * 1 Carrier
  * 2 Destroyers
  * 4 Gunships
* Grid is 10 x 10 to keep it simple (but the design should be open for enhancements)

### Hardcore enhancement (optional)
- Implement an AI player
