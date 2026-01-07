# jj-desc demo - Backfill multiple commits at once

Output docs/demo.gif

Set Shell "zsh"
Set FontSize 15
Set Width 1000
Set Height 700
Set Theme "Catppuccin Mocha"
Set Padding 20
Set TypingSpeed 40ms

# Setup: change to demo directory
Hide
Type "cd /tmp/jj-desc-demo"
Enter
Sleep 500ms
Show

# Show the problem - multiple commits without descriptions
Type "jj log"
Enter
Sleep 3s

# One command to fix them all!
Type "jj-desc"
Enter
Sleep 12s

# Show the result - all commits now have descriptions
Type "jj log"
Enter
Sleep 4s
