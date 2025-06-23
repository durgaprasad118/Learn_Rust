## Better Commits

1. Which files to add

- Check suitable files (same topic) {combine changes of a specific topic}
- we can add part of file using `git add -p /path/of/file` few y or n done

2. Write perfect commit

- add good description

## Branching

- Long running branch (main)/(dev)
- branches should be added to long running (merge or reabse)
- _short lived branches_ - for a feature of bug fix
  - will be eventually deleted after rebase or merge

## Pull request

- Invites to review for feedback
- contributing to repositories that you don't have access to directly push
- **fork** - creating your own copy of repository

## Merge conflicts

- when integrating commits from different sources
- you can always undo the merge conflict issue
  - `git merge --abort`
  - `git rebase --abort`
  - puts in same stage as before doing merge
- after resolvign conflict we need to `add the file`
  - then commit and push
- fast forward merge
  - when the feture branch has same base as main so when we push the changes to that no merge commit is created (it's like breanch is tip is taken and put infront of long running main branch)
-
