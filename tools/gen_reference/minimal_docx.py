#!/usr/bin/env python3
"""
Generate minimal DOCX file for reference.

This script creates a simple "Hello World" DOCX file using python-docx.
The generated file is used to understand the XML structure and relationships.
"""

from docx import Document
from docx.shared import Pt, RGBColor, Inches
from docx.enum.text import WD_ALIGN_PARAGRAPH
from docx.enum.section import WD_SECTION
from docx.oxml import OxmlElement
from docx.oxml.ns import qn
import os

def create_minimal_docx():
    """Create a minimal DOCX with basic content."""
    doc = Document()
    
    # Add a simple paragraph
    p = doc.add_paragraph("Hello World")
    
    # Save the document
    output_path = os.path.join(os.path.dirname(__file__), 
                               "../../tests/fixtures/reference_docs/minimal.docx")
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    doc.save(output_path)
    print(f"Created: {output_path}")
    return output_path


def create_layout_docx():
    """Create a DOCX with headers/footers, page and section breaks, and block quotes."""
    doc = Document()

    def add_toc_paragraph(document):
        """Insert a TOC field for levels 1-3 (auto-updatable in Word)."""
        p = document.add_paragraph()
        r = p.add_run()

        fld_begin = OxmlElement("w:fldChar")
        fld_begin.set(qn("w:fldCharType"), "begin")
        r._r.append(fld_begin)

        instr = OxmlElement("w:instrText")
        instr.set(qn("xml:space"), "preserve")
        instr.text = ' TOC \\"1-3\\" \\h \\z \\u '
        r._r.append(instr)

        fld_sep = OxmlElement("w:fldChar")
        fld_sep.set(qn("w:fldCharType"), "separate")
        r._r.append(fld_sep)

        fld_end = OxmlElement("w:fldChar")
        fld_end.set(qn("w:fldCharType"), "end")
        r._r.append(fld_end)

    # Header and footer
    section = doc.sections[0]
    section.header.paragraphs[0].text = "Header: Reference Document"
    section.footer.paragraphs[0].text = "Footer: Page "
    footer_run = section.footer.paragraphs[0].add_run()
    footer_run.add_field("PAGE")  # requires python-docx >= 0.8.11 field support; if unavailable, text remains

    # Page break
    doc.add_paragraph("Page 1 content before page break.")
    doc.add_page_break()
    doc.add_paragraph("Page 2 content after page break.")

    # Section break (next page)
    doc.add_section(start_type=WD_SECTION.NEW_PAGE)
    doc.add_paragraph("Section 2 begins here.")

    # Block quote
    block_quote = doc.add_paragraph("This is a block quote sample.")
    block_quote.style = "Intense Quote" if "Intense Quote" in [s.name for s in doc.styles] else block_quote.style

    # Table of contents field (levels 1-3)
    doc.add_paragraph("Table of Contents (update field in Word):")
    add_toc_paragraph(doc)

    output_path = os.path.join(os.path.dirname(__file__), "../../tests/fixtures/reference_docs/layout.docx")
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    doc.save(output_path)
    print(f"Created: {output_path}")
    return output_path


def create_images_docx():
    """Create a DOCX with inline and floating images."""
    doc = Document()

    # Inline image
    doc.add_paragraph("Inline image below:")
    inline_img_path = os.path.join(os.path.dirname(__file__), "../../tests/fixtures/reference_docs/assets/inline.png")
    os.makedirs(os.path.dirname(inline_img_path), exist_ok=True)
    # Placeholder: caller should place an image at inline.png; keep reference for structure
    if os.path.exists(inline_img_path):
        doc.add_picture(inline_img_path, width=Inches(2))
    else:
        doc.add_paragraph("[inline.png missing — place test asset here]")

    # Floating image (left, wrapped)
    doc.add_paragraph("Floating image (left, tight wrap):")
    float_img_path = os.path.join(os.path.dirname(__file__), "../../tests/fixtures/reference_docs/assets/floating.png")
    if os.path.exists(float_img_path):
        pic = doc.add_picture(float_img_path, width=Inches(2.5))
        last_paragraph = doc.paragraphs[-1]
        last_paragraph.alignment = WD_ALIGN_PARAGRAPH.LEFT
        # python-docx lacks native floating support; note in text for XML inspection
        doc.add_paragraph("(Floating positioning to be adjusted in XML if needed)")
    else:
        doc.add_paragraph("[floating.png missing — place test asset here]")

    output_path = os.path.join(os.path.dirname(__file__), "../../tests/fixtures/reference_docs/images.docx")
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    doc.save(output_path)
    print(f"Created: {output_path}")
    return output_path

def create_styled_docx():
    """Create a DOCX with various text styles."""
    doc = Document()
    
    # Heading
    doc.add_heading("Document Title", level=1)
    
    # Paragraph with mixed formatting
    p = doc.add_paragraph()
    p.add_run("Bold text").bold = True
    p.add_run(" and ")
    p.add_run("italic text").italic = True
    p.add_run(" and ")
    p.add_run("underlined text").underline = True
    
    # Colored text
    p = doc.add_paragraph()
    run = p.add_run("Red colored text")
    run.font.color.rgb = RGBColor(255, 0, 0)
    
    # Different font sizes
    p = doc.add_paragraph()
    run = p.add_run("Small text (8pt)")
    run.font.size = Pt(8)
    
    p = doc.add_paragraph()
    run = p.add_run("Large text (16pt)")
    run.font.size = Pt(16)
    
    # Paragraph alignment
    p = doc.add_paragraph("Left aligned (default)")
    p.alignment = WD_ALIGN_PARAGRAPH.LEFT
    
    p = doc.add_paragraph("Center aligned")
    p.alignment = WD_ALIGN_PARAGRAPH.CENTER
    
    p = doc.add_paragraph("Right aligned")
    p.alignment = WD_ALIGN_PARAGRAPH.RIGHT
    
    # Bullet list
    doc.add_paragraph("First item", style='List Bullet')
    doc.add_paragraph("Second item", style='List Bullet')
    doc.add_paragraph("Third item", style='List Bullet')
    
    # Numbered list
    doc.add_paragraph("First", style='List Number')
    doc.add_paragraph("Second", style='List Number')
    doc.add_paragraph("Third", style='List Number')
    
    # Table
    table = doc.add_table(rows=3, cols=3)
    table.style = 'Light Grid Accent 1'
    
    # Header row
    hdr_cells = table.rows[0].cells
    hdr_cells[0].text = 'Header 1'
    hdr_cells[1].text = 'Header 2'
    hdr_cells[2].text = 'Header 3'
    
    # Data rows
    table.rows[1].cells[0].text = 'Data 1'
    table.rows[1].cells[1].text = 'Data 2'
    table.rows[1].cells[2].text = 'Data 3'
    
    table.rows[2].cells[0].text = 'Data 4'
    table.rows[2].cells[1].text = 'Data 5'
    table.rows[2].cells[2].text = 'Data 6'
    
    # Hyperlink
    p = doc.add_paragraph()
    p.add_run("Click here").font.color.rgb = RGBColor(0, 0, 255)
    
    # Save the document
    output_path = os.path.join(os.path.dirname(__file__), 
                               "../../tests/fixtures/reference_docs/styled.docx")
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    doc.save(output_path)
    print(f"Created: {output_path}")
    return output_path

def create_headings_docx():
    """Create a DOCX with all heading levels."""
    doc = Document()
    
    doc.add_heading("Heading Level 1", level=1)
    doc.add_paragraph("Body text under heading 1")
    
    doc.add_heading("Heading Level 2", level=2)
    doc.add_paragraph("Body text under heading 2")
    
    doc.add_heading("Heading Level 3", level=3)
    doc.add_paragraph("Body text under heading 3")
    
    doc.add_heading("Heading Level 4", level=4)
    doc.add_paragraph("Body text under heading 4")
    
    doc.add_heading("Heading Level 5", level=5)
    doc.add_paragraph("Body text under heading 5")
    
    doc.add_heading("Heading Level 6", level=6)
    doc.add_paragraph("Body text under heading 6")
    
    # Save the document
    output_path = os.path.join(os.path.dirname(__file__), 
                               "../../tests/fixtures/reference_docs/headings.docx")
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    doc.save(output_path)
    print(f"Created: {output_path}")
    return output_path

if __name__ == "__main__":
    print("Generating DOCX reference documents...")
    create_minimal_docx()
    create_styled_docx()
    create_headings_docx()
    create_images_docx()
    create_layout_docx()
    print("Done!")
