# Tasks: Homebrew Distribution Support

## Prerequisites

- Access to GitHub account
- Repository admin permissions

## Task List

### 1. Create homebrew-tap Repository

- [ ] Create `tumf/homebrew-tap` repository on GitHub
  - Create as public repository
  - Include README.md (optional)
  - Create `Formula/` directory

**Verification**: Confirm repository exists via `gh repo view tumf/homebrew-tap`

### 2. Create Personal Access Token (PAT)

- [ ] Create new token at GitHub Settings > Developer settings > Personal access tokens
  - Scope: `repo` (Full control of private repositories) or
  - For fine-grained token: Contents Read and write permission for `tumf/homebrew-tap`
- [ ] Store token securely

**Verification**: Confirm token is valid

### 3. Configure GitHub Secret

- [ ] Add `HOMEBREW_TAP_TOKEN` at `tumf/jj-desc` repository Settings > Secrets and variables > Actions
  - Value: PAT created in step 2

**Verification**: Confirm secret is configured (value cannot be verified)

### 4. Update README.md

- [ ] Add Homebrew instructions to installation section

```markdown
### Homebrew (macOS/Linux)

```bash
brew tap tumf/tap
brew install jj-desc
```
```

**Verification**: Confirm README.md contains Homebrew installation instructions

### 5. Verify via Release

- [ ] Push new version tag to trigger release
- [ ] Confirm `publish-homebrew-formula` job succeeds
- [ ] Confirm Formula file is committed to `tumf/homebrew-tap`
- [ ] Confirm `brew tap tumf/tap && brew install jj-desc` succeeds

**Verification**: 
- GitHub Actions release workflow succeeds
- Installation via `brew install tumf/tap/jj-desc` works

## Dependencies

- Tasks 2 and 3 cannot run in parallel (secret configuration requires PAT)
- Task 4 can run in parallel with other tasks
- Task 5 requires completion of tasks 1, 2, and 3

## Notes

- No manual Formula creation needed due to cargo-dist automation
- Release workflow already configured (`publish-homebrew-formula` job)
