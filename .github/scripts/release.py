#!/usr/bin/env python3
import json
import re
import subprocess
import sys
import os

VALID_VERSION_NUMBER = re.compile(r"^v\d+\.\d+\.\d+$")
DEFAULT='last'

def get_last_version() -> str:
    """Return the version number of the last release."""
    json_string = (
        subprocess.run(
            ["gh", "release", "view", "--repo", "sdf-labs/sql-functions", "--json", "tagName"],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        .stdout.decode("utf8")
        .strip()
    )

    return json.loads(json_string)["tagName"]


def bump_patch_number(version_number: str) -> str:
    """Return a copy of `version_number` with the patch number incremented."""
    major, minor, patch = version_number.split(".")
    return f"{major}.{minor}.{int(patch) + 1}"


def create_new_patch_release(new_version_number = None):
    """Create a new patch release on GitHub."""
    if new_version_number is None:
        try:
            last_version_number = get_last_version()
        except subprocess.CalledProcessError as err:
            raise RuntimeError("Could not get last version number")
        new_version_number = bump_patch_number(last_version_number)
        check_valid = re.match(VALID_VERSION_NUMBER, new_version_number)
        if not check_valid:
            raise ValueError(f"Version number is not valid: {new_version_number}")
    subprocess.run(
        ["gh", "release", "create", "--repo", "sdf-labs/sql-functions", "--generate-notes", new_version_number],
        check=True,
    )
    return new_version_number


if __name__ == "__main__":
    # Add Parsing for passed arguments
    new_version_number = sys.argv[1]
    # Validate Regex of Version Number
    if new_version_number != DEFAULT:
        new_version_number = 'v' + new_version_number.strip()
        check_valid = re.match(VALID_VERSION_NUMBER, new_version_number)
        if not check_valid:
            raise ValueError(f"Version number is not valid: {new_version_number}")
        new_version = create_new_patch_release(new_version_number)
    else:
        new_version = create_new_patch_release()
    with open(os.environ["GITHUB_OUTPUT"], "a") as f:
        f.write(f"version={new_version}\n")
