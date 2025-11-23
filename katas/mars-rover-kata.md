# Mars Rover Kata – Rules & Iterations

## Context
You are building the control software for a Mars Rover operating on a rectangular plateau. The plateau is represented as a grid with (x, y) coordinates. The rover has a heading (N, E, S, W) and can receive a sequence of commands to move and rotate. Multiple iterative extensions will evolve the design.

## Iteration 1: Basic Navigation
Input: Upper-right plateau coordinate and the rover's initial position and heading. Then a string of commands.

Commands:
- L: rotate 90° left (counter-clockwise)
- R: rotate 90° right (clockwise)
- M: move forward one grid point in current heading

Example Input:
```
5 5
1 2 N
LMLMLMLMM
3 3 E
MMRMMRMRRM
```
Expected Output:
```
1 3 N
5 1 E
```

Requirements:
- Plateau lower-left is implicitly (0, 0)
- Prevent moving outside plateau; decide: ignore move or error (pick and document one strategy)
- Provide a function to execute a command sequence and return final (x, y, heading)

## Iteration 2: Multiple Rovers & Collision Avoidance
- Support deploying multiple rovers; each sequence processed sequentially
- Track occupied cells; if a move would collide, skip that move (document strategy) or raise an error
- Add tests for collision and boundary behaviors

## Iteration 3: Parsing & Validation
- Accept a structured input format (string or data structure)
- Validate plateau size, starting positions inside bounds, headings valid, command sequence only in {L,R,M}
- Provide clear error types (e.g., ParseError, ValidationError)

## Iteration 4: Extended Commands
Add new commands incrementally:
- B: move backward one grid point
- U/D: tilt camera up/down (no position change) – track a simple camera angle state
- ?: query status – could append a status snapshot to an output log

Each new command should have its own tests; keep prior behavior unaffected.

## Iteration 5: Obstacles
- Inject obstacles as a set of (x, y) coordinates
- A move into an obstacle is blocked; log the attempt
- Provide an optional path report listing all successful positions visited

## Iteration 6: Wrapping or Toroidal Option (Optional Variant)
- Optionally allow plateau to wrap (moving off east re-enters west). Make this a strategy selectable at construction time.

## Iteration 7: Functional Core / Imperative Shell
- Separate pure logic (state transition given state + command) from I/O parsing and printing
- Ensure core functions are easy to unit test

## Suggested Design
- Data structures: Plateau {max_x, max_y, obstacles?, wrapping?}; Rover {x, y, heading, camera_angle, log}
- Enum Heading {N,E,S,W}; Enum Command {L,R,M,B,U,D,QUERY}
- State transition: apply_command(state, command) -> new_state (+ side-effect-free log entries)

## Edge Cases to Consider
- Move at plateau edge
- Collision with another rover
- Invalid command character
- Empty command sequence
- 0x0 plateau or tiny plateau
- Backward move at boundary
- Obstacle immediately in front

## Testing Guidelines
- TDD: start with smallest scenario (single rover, one move)
- Use parameterized tests for rotations
- Test each new command separately
- Keep tests deterministic; avoid hidden randomness

## Stretch Ideas (After Core Complete)
- Add energy consumption per command
- Support diagonal headings (NE, SE, SW, NW)
- Serialize mission plan to JSON
- Replay log functionality
- Visualization (ASCII grid or simple GUI)

## Constraints & Practices
- Keep functions small and pure where possible
- Prefer returning Result<T, E> for fallible operations
- Document chosen strategies (boundary handling, collision policy)
- Refactor mercilessly after green tests

## Getting Started (Minimal Steps)
1. Define Heading enum and left/right rotation logic
2. Implement forward movement with bounds check
3. Parse a single rover + commands
4. Add multiple rovers sequencing
5. Layer on collisions/obstacles

## Rules
- Follow the TDD (Test-Driven Development) rules strictly
- Start with the simplest test case and build incrementally
- Refactor as you go to keep the code clean
- Be careful about edge cases and exceptions. We can not afford to lose a mars rover, just because the developers overlooked a null pointer.
