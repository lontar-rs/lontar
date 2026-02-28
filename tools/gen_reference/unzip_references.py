#!/usr/bin/env python3
"""
Unzip all reference DOCX/PPTX files to inspect their XML structure.

Extracts each .docx and .pptx file into a directory with the same name,
making it easy to inspect the generated XML.
"""

import os
import zipfile
import glob
import shutil

REFERENCE_DIR = os.path.join(os.path.dirname(__file__),
                             "../../tests/fixtures/reference_docs")
EXTRACTED_DIR = os.path.join(os.path.dirname(__file__),
                             "../../tests/fixtures/expected_xml")


def unzip_all():
    os.makedirs(EXTRACTED_DIR, exist_ok=True)

    patterns = ["*.docx", "*.pptx"]
    files = []
    for pat in patterns:
        files.extend(glob.glob(os.path.join(REFERENCE_DIR, pat)))

    if not files:
        print(f"No .docx or .pptx files found in {REFERENCE_DIR}")
        print("Run minimal_docx.py and minimal_pptx.py first.")
        return

    for filepath in sorted(files):
        basename = os.path.splitext(os.path.basename(filepath))[0]
        ext = os.path.splitext(filepath)[1].lstrip(".")
        extract_to = os.path.join(EXTRACTED_DIR, f"{basename}_{ext}")

        if os.path.exists(extract_to):
            shutil.rmtree(extract_to)

        with zipfile.ZipFile(filepath, "r") as z:
            z.extractall(extract_to)

        print(f"Extracted: {filepath} -> {extract_to}/")

    print(f"\nDone! Inspect XML in: {EXTRACTED_DIR}")


if __name__ == "__main__":
    unzip_all()
