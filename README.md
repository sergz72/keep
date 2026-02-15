# keep
A small command line utility to keep the n latest files in the folder.

# Usage
Usage: keep folder_name number_of_latest_files_to_keep [dry_run]

# Description

This utility does the following:

1. Receives a file list for the specified folder and groups files by file name without extension.
2. Sorts file groups by modification date in descending order.
3. Skips number_of_latest_files_to_keep files in each group of files, and if this is not a dry run, removes the rest of the files in this group.
