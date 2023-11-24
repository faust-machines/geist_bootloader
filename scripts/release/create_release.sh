#!/bin/bash

# Ensure the script is run from the 'main' branch
current_branch=$(git rev-parse --abbrev-ref HEAD)
if [ "$current_branch" != "main" ]; then
  echo "This script must be run from the 'main' branch because we use squash commits for PRs."
  echo "Commits from PRs end up in a detached HEAD state and are not included when running this script from other branches."
  exit 1
fi

# Check if tag_name argument is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <new_tag_name>"
  exit 1
fi

# Assign new tag name from the first argument
new_tag_name=$1

# Check if there are any existing tags
if git describe --tags --abbrev=0 > /dev/null 2>&1; then

  # Get the latest release tag
  latest_tag=$(git describe --tags --abbrev=0)

  # Get the commit hash associated with the latest release tag
  latest_commit_hash=$(git rev-list -n 1 $latest_tag)

  # Get the commit date in ISO 8601 format
  latest_commit_date=$(git show -s --format=%ci $latest_commit_hash)

  # Check OS for date command compatibility and convert the date
  if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS (BSD) - using 'cut' to remove the timezone part and then convert
    latest_commit_date=$(echo $latest_commit_date | cut -d ' ' -f 1-2)
    latest_commit_date=$(date -j -f "%Y-%m-%d %H:%M:%S" "$latest_commit_date" "+%Y-%m-%dT%H:%M:%SZ")
  else
    # Linux (GNU) - direct conversion
    latest_commit_date=$(date -u -d "$latest_commit_date" "+%Y-%m-%dT%H:%M:%SZ")
  fi

else
  echo "No existing tags found. Proceeding without previous tag information."
  latest_tag="None"
  latest_commit_hash="None"
  # Use a very old date to include all PRs in the release notes
  latest_commit_date="1970-01-01T00:00:00Z"
fi

# Generate the release notes
release_notes=$(gh pr list --base main --search "is:closed merged:>=${latest_commit_date}" --json number,title,author | jq -r '.[] | "- [#\(.number)]: \(.title)"')

# lets get the latest commit hash set git rev-parse HEAD
target_commit_hash=$(git rev-parse HEAD)

# Show the release notes preview
echo "Preview of the release notes:"
echo "$release_notes"

# Display the current version and new tag name, then ask for confirmation
echo ""
echo "Previous version: $latest_tag"
echo "New version: $new_tag_name"
echo "Previous commit hash: $latest_commit_hash"
echo "New commit hash: $target_commit_hash"
read -p "You are about to create and release a new version with the tag: $new_tag_name. Continue? [y/N]: " confirm

# Check if the user confirmed
if [[ $confirm == [yY] || $confirm == [yY][eE][sS] ]]; then
  # Create a new annotated Git tag with a message
  git tag -a $new_tag_name $target_commit_hash -m "Release $new_tag_name"

  # Save the release notes to a file
  echo "$release_notes" > release_notes.md

  # Push the new Git tag to the remote repository
  git push origin $new_tag_name

  # Create a new GitHub release with the provided new tag name
  gh release create $new_tag_name --notes-file release_notes.md
else
  echo "Release creation cancelled."
fi

# Remove the release notes file
rm release_notes.md