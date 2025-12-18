Build a CLI tool that evaluates Reverse Polish Notation Math("3 10 + 2 /" = 6.5)
- You can't use .unwrap() or .except().
- If the user types "3 +", it must print "Error: Not enough values", not crash.
- If the user divides by zero, it must return a specific Enum variant. 