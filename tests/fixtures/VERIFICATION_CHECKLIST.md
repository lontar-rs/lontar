# Reference Document Verification Checklist

Verify each generated reference document opens and renders correctly.

## How to Verify

### LibreOffice (local)
1. Install LibreOffice (apt install libreoffice)
2. Open each .docx with `libreoffice --writer <file>`
3. Open each .pptx with `libreoffice --impress <file>`
4. Check each item below visually

### MS Office Online (web)
1. Upload files to OneDrive or go to office.com
2. Open each file in the web editor
3. Check each item below — this catches compatibility issues LibreOffice misses

---

## DOCX Verification

| Document | LibreOffice | MS Office Online | Notes |
|---|---|---|---|
| minimal.docx | ☐ Opens, text visible | ☐ Opens, text visible | |
| headings.docx | ☐ All 6 levels distinct | ☐ All 6 levels in nav pane | |
| styled_text.docx | ☐ Bold/italic/underline correct | ☐ Color and highlight visible | |
| tables.docx | ☐ Borders, alignment correct | ☐ Merged cells render correctly | Check column widths |
| images.docx | ☐ Inline image visible | ☐ Float image wraps text | Check image resolution |
| lists.docx | ☐ Bullet/number styles correct | ☐ Nesting indentation correct | |
| headers_footers.docx | ☐ Header text on every page | ☐ Footer page numbers correct | |
| page_breaks.docx | ☐ Content starts on new pages | ☐ Section breaks respected | |
| table_of_contents.docx | ☐ TOC renders with page nums | ☐ TOC clickable links work | Right-click → update TOC |
| hyperlinks.docx | ☐ Links clickable, correct URL | ☐ Tooltip shows on hover | |
| block_quotes.docx | ☐ Indented, styled differently | ☐ Nested quotes visible | |

## PPTX Verification

| Document | LibreOffice Impress | MS PowerPoint Online | Notes |
|---|---|---|---|
| title_slide.pptx | ☐ Centered title + subtitle | ☐ Centered title + subtitle | |
| content_bullets.pptx | ☐ Bullet points visible | ☐ Nested bullets indented | |
| table_slide.pptx | ☐ Table renders with borders | ☐ Table cells aligned | |
| chart_slide.pptx | ☐ Bar/line/pie charts visible | ☐ Chart data correct | Most critical — charts are complex |
| image_slide.pptx | ☐ Image visible, sized correctly | ☐ Image not cropped/distorted | |
| two_column.pptx | ☐ Two columns side by side | ☐ Text doesn't overlap | |
| speaker_notes.pptx | ☐ Notes visible in notes view | ☐ Notes visible in presenter | |

## Common Issues to Watch For

- [ ] Missing fonts falling back to incorrect substitutes
- [ ] Table column widths collapsing to zero
- [ ] Images showing as broken placeholders
- [ ] List numbering restarting incorrectly
- [ ] Chart data series in wrong order
- [ ] RTL text (if any) rendering backwards
- [ ] File marked as "needs repair" on open — indicates malformed XML

## Sign-off

| Verifier | Date | LibreOffice Version | MS Office Version |
|---|---|---|---|
| | | | |
