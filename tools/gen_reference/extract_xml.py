#!/usr/bin/env python3
"""
Extract XML from DOCX/PPTX files for analysis.

This script unzips DOCX and PPTX files and organizes the extracted XML
into the tests/fixtures/expected_xml directory for reference.
"""

import zipfile
import os
import shutil
from pathlib import Path
from xml.dom import minidom

def prettify_xml(xml_string):
    """Pretty-print XML string."""
    try:
        dom = minidom.parseString(xml_string)
        return dom.toprettyxml(indent="  ")
    except Exception:
        return xml_string

def extract_docx(docx_path, output_dir):
    """Extract XML from a DOCX file."""
    print(f"Extracting DOCX: {docx_path}")

    # Create output directory
    output_path = os.path.join(output_dir, "docx", Path(docx_path).stem)
    os.makedirs(output_path, exist_ok=True)

    # Extract all files
    with zipfile.ZipFile(docx_path, 'r') as zip_ref:
        zip_ref.extractall(output_path)

    # Pretty-print XML files
    for root, dirs, files in os.walk(output_path):
        for file in files:
            if file.endswith('.xml'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        content = f.read()

                    # Pretty-print
                    pretty_content = prettify_xml(content)

                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(pretty_content)

                    print(f"  Formatted: {file}")
                except Exception as e:
                    print(f"  Error formatting {file}: {e}")

    print(f"  Extracted to: {output_path}")
    return output_path

def extract_pptx(pptx_path, output_dir):
    """Extract XML from a PPTX file."""
    print(f"Extracting PPTX: {pptx_path}")

    # Create output directory
    output_path = os.path.join(output_dir, "pptx", Path(pptx_path).stem)
    os.makedirs(output_path, exist_ok=True)

    # Extract all files
    with zipfile.ZipFile(pptx_path, 'r') as zip_ref:
        zip_ref.extractall(output_path)

    # Pretty-print XML files
    for root, dirs, files in os.walk(output_path):
        for file in files:
            if file.endswith('.xml'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        content = f.read()

                    # Pretty-print
                    pretty_content = prettify_xml(content)

                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(pretty_content)

                    print(f"  Formatted: {file}")
                except Exception as e:
                    print(f"  Error formatting {file}: {e}")

    print(f"  Extracted to: {output_path}")
    return output_path

def extract_all():
    """Extract all reference documents."""
    script_dir = os.path.dirname(__file__)
    ref_docs_dir = os.path.join(script_dir, "../../tests/fixtures/reference_docs")
    output_dir = os.path.join(script_dir, "../../tests/fixtures/expected_xml")

    # Create output directory
    os.makedirs(output_dir, exist_ok=True)

    # Find and extract all DOCX files
    docx_files = list(Path(ref_docs_dir).glob("*.docx"))
    for docx_file in docx_files:
        try:
            extract_docx(str(docx_file), output_dir)
        except Exception as e:
            print(f"Error extracting {docx_file}: {e}")

    # Find and extract all PPTX files
    pptx_files = list(Path(ref_docs_dir).glob("*.pptx"))
    for pptx_file in pptx_files:
        try:
            extract_pptx(str(pptx_file), output_dir)
        except Exception as e:
            print(f"Error extracting {pptx_file}: {e}")

    print(f"\nAll files extracted to: {output_dir}")

if __name__ == "__main__":
    print("Extracting XML from reference documents...")
    extract_all()
    print("Done!")
