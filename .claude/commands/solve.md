We are going to solve today's AOC 2025 problem. We are looking to develop a clear , readable, yet highly performant solution to the problems presented in rust. We will also create documentation for each day to educate concisely the problem and solution.

## Workflow

First we will run the new day initialisation script. Then you will wait for me to add the part 1 puzzle and input text. When I have added them, you will create a concise commit of the generated files.

Once these steps are complete, you will think hard about how to abstract and solve the problem in with the most effecient algorithm(s) you can find. This may involve breaking the problem down to multiple steps and combining algorithms. Write appropriate tests for the given examples. If there are multiple possible implementations that could be fastest, implement each and create benchmarking tests. Use the fastest as the default implementation for the day. Using the all-impls flag should run all implementations.

Once part 1 is complete, ask me to verify the answer and provide part 2. At this stage you will commit the part 1 solution and the part 2 puzzle (in different concise commits).

Once confirmed, complete part 2 in the same detailed way as part 1. It may require adjusting, adding to, or re-writing part 1, all of which are valid approaches. Again we are looking for fast implementations (ideally <1ms).

Once part 2 is completed, and I have verified the answer, commit the changes.

Then think about how we could refactor or optimise the solution, aiming for clear and fast implementation that is as performant as possible. Do not use paralellisation unless expressly directed too, we are mainly looking for algorithmic and language based optimisations. If I approve any ideas for optimisation, implement them, check answers and tests, and commit the changes.

Then update the documentation for the day. 

## Final Notes
Begin by using your todolist tool to keep track of the workflow you are completing. Use it as often as possible if you need to break down work further.

Always run `just ci` and fix any issues before commiting changes.

Ask clarifying questions as needed.

