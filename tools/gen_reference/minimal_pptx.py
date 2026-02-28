#!/usr/bin/env python3
"""
Generate minimal PPTX file for reference.

This script creates a simple presentation using python-pptx.
The generated file is used to understand the XML structure and relationships.
"""

from pptx import Presentation
from pptx.util import Inches, Pt
from pptx.enum.text import PP_ALIGN
from pptx.dml.color import RGBColor
import os

def create_minimal_pptx():
    """Create a minimal PPTX with a single title slide."""
    prs = Presentation()
    
    # Add a title slide
    slide = prs.slides.add_slide(prs.slide_layouts[0])  # Title slide layout
    title = slide.shapes.title
    title.text = "Hello World"
    
    # Save the presentation
    output_path = os.path.join(os.path.dirname(__file__), 
                               "../../tests/fixtures/reference_docs/minimal.pptx")
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    prs.save(output_path)
    print(f"Created: {output_path}")
    return output_path

def create_styled_pptx():
    """Create a PPTX with various text styles and layouts."""
    prs = Presentation()
    
    # Slide 1: Title slide
    slide = prs.slides.add_slide(prs.slide_layouts[0])
    title = slide.shapes.title
    title.text = "Styled Presentation"
    subtitle = slide.placeholders[1]
    subtitle.text = "With various formatting"
    
    # Slide 2: Title and content
    slide = prs.slides.add_slide(prs.slide_layouts[1])
    title = slide.shapes.title
    title.text = "Text Formatting"
    
    content = slide.placeholders[1]
    tf = content.text_frame
    
    # Add bullet points with formatting
    p = tf.paragraphs[0]
    p.text = "Bold text"
    p.level = 0
    run = p.runs[0]
    run.bold = True
    
    p = tf.add_paragraph()
    p.text = "Italic text"
    p.level = 0
    run = p.runs[0]
    run.italic = True
    
    p = tf.add_paragraph()
    p.text = "Colored text"
    p.level = 0
    run = p.runs[0]
    run.font.color.rgb = RGBColor(255, 0, 0)
    
    p = tf.add_paragraph()
    p.text = "Large text (24pt)"
    p.level = 0
    run = p.runs[0]
    run.font.size = Pt(24)
    
    # Slide 3: Bullet list
    slide = prs.slides.add_slide(prs.slide_layouts[1])
    title = slide.shapes.title
    title.text = "Bullet List"
    
    content = slide.placeholders[1]
    tf = content.text_frame
    
    p = tf.paragraphs[0]
    p.text = "First item"
    p.level = 0
    
    p = tf.add_paragraph()
    p.text = "Second item"
    p.level = 0
    
    p = tf.add_paragraph()
    p.text = "Nested item"
    p.level = 1
    
    p = tf.add_paragraph()
    p.text = "Third item"
    p.level = 0
    
    # Slide 4: Blank slide with shapes
    slide = prs.slides.add_slide(prs.slide_layouts[6])  # Blank layout
    
    # Add a text box
    left = Inches(1)
    top = Inches(1)
    width = Inches(8)
    height = Inches(1)
    
    txBox = slide.shapes.add_textbox(left, top, width, height)
    tf = txBox.text_frame
    tf.text = "This is a text box on a blank slide"
    
    # Add a shape with text
    left = Inches(2)
    top = Inches(3)
    width = Inches(4)
    height = Inches(2)
    
    shape = slide.shapes.add_shape(1, left, top, width, height)  # 1 = rectangle
    shape.text = "Rectangle Shape"
    
    # Save the presentation
    output_path = os.path.join(os.path.dirname(__file__), 
                               "../../tests/fixtures/reference_docs/styled.pptx")
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    prs.save(output_path)
    print(f"Created: {output_path}")
    return output_path

def create_layouts_pptx():
    """Create a PPTX demonstrating different slide layouts."""
    prs = Presentation()
    
    # Slide 1: Title slide
    slide = prs.slides.add_slide(prs.slide_layouts[0])
    title = slide.shapes.title
    title.text = "Different Layouts"
    subtitle = slide.placeholders[1]
    subtitle.text = "Demonstrating slide layout types"
    
    # Slide 2: Title and content
    slide = prs.slides.add_slide(prs.slide_layouts[1])
    title = slide.shapes.title
    title.text = "Title and Content"
    content = slide.placeholders[1]
    tf = content.text_frame
    tf.text = "This is a title and content layout"
    
    # Slide 3: Blank
    slide = prs.slides.add_slide(prs.slide_layouts[6])
    
    left = Inches(1)
    top = Inches(1)
    width = Inches(8)
    height = Inches(5)
    
    txBox = slide.shapes.add_textbox(left, top, width, height)
    tf = txBox.text_frame
    tf.text = "This is a blank layout with a custom text box"
    
    # Save the presentation
    output_path = os.path.join(os.path.dirname(__file__), 
                               "../../tests/fixtures/reference_docs/layouts.pptx")
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    prs.save(output_path)
    print(f"Created: {output_path}")
    return output_path

if __name__ == "__main__":
    print("Generating PPTX reference documents...")
    create_minimal_pptx()
    create_styled_pptx()
    create_layouts_pptx()
    print("Done!")
