# Deimos

`The extensible Markdown format.`

## Motivation

Note that this is originally based on the original [saturn](https://github.com/hvlck/saturn) format. Saturn and Deimos have two different goals, so I decided to stop development on Saturn and begin working on Deimos.

### Goals

1. Be easy to author and write in
2. Be easily extensible to create and build with

#### Minor Goals

+ be easy to be converted into digital form from print using OCR

## Syntax

### Footnotes

`^footnoteNumber`

+ used to reference a footnote
  + ex: `^1`, `^100`

`^footnoteNumber{footnoteText}`

+ used to create a footnote
  + ex: `^1{[github](https://github.com)}`
+ valid markdown can go within the parenthesis
+ can also sdeimos multiple lines

#### Footnote Output

### Endnotes

### Escaping

+ `\` backslashes used to escape formatting

### Images

`![alternative text](url)`

+ standard CommonMark image syntax is used
+ captions?

### Inline HTML

+ inline HTML is not allowed

### Math

```deimos
``\frac{1}{2}``
```

+ math (KaTeX syntax) created using two backticks (````) or using fenced code blocks with a math language/dsl

### Metadata

+ metadata starts with a tilde (`~`)
  + todo: change?
+ metadata key/values specified using Pluto conventions
+ must occur at start (top) of document
  + no other text can come before it
+ metadata can be omitted from a document entirely
+ any key that has a `<meta>` `name` [standard names](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/meta/name) will be converted into `<meta>` tags
+ any key that doesn't have a `<meta>` `name` standard name will be converted into a [<data> element](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/data)
+ adding a `!` after the tilde (`~!`) will prevent the metadata from being converted into `<data>` and `<meta>` elements

```deimos
~key value
```

```deimos
~title This is an example document.
```

### Metaprogramming and Extensions

+ maybe a small language (like a more general-purpose lanner) for metaprogramming and general use
  + similar to [observablehq/stdlib](https://github.com/observablehq/stdlib)

### Data and Graphs

`@[bar|candle|][title|x-axis-title|y-axis-title][x-labels,x-labels|y-labels,y-labels][data]`

`@[bar][Number of Humans Per Continent|Continent|Population (in Millions)][Asia,N. America,S. America|0,50,100,150]`

+ types of graphs and charts:
  + bar
  + candle
  + pie
  + more
+ format of data section varies for each type

#### Output

### Code Blocks

#### Fenced Code Blocks

Fenced code blocks are isolated code blocks. They are made using three backticks (`\``) on a separate line, followed by the language, and closed with the same pattern on a separate line.

```deimos
\`\`\`javascript

const name = "Generic Name"

\`\`\`
```

The previous code block will produce the following HTML output:

```html
<pre><code>const name = "Generic Name"</code></pre>
```

#### Inline Code Blocks

Inline code blocks are placed in a paragraph; they start and close with a backtick (`).

```markdown
This is the start of a paragraph. The following expression `1+1` produces the result `2`.
```

The previous example will produce the following HTML:

```html
<p>This is the start of a paragraph. <code>1+1</code> produces the result <code>2</code>.</p>
```

### Headings

+ cannot have duplicate headings at the same level
  + e.g. there cannot be two `## Heading Example`
  + when generating ids, the previous heading names (those that are directly higher in the hierarchy than the duplicate) will be combined using `--` signs (see output)
+ can only be one top-level (`#`) heading
+ max heading level is six (`######`)
+ only characters and numbers are allowed in headings (`a-zA-Z0-9`)
  + only single spaces are allowed (no tabs, etc.)

#### Heading Output

```deimos
# Top

## Second to Top

### Top

## Top
```

```html
<h1 id="#top">Top</h1>
<h2 id="#second-to-top">Second to Top</h2>
<h3 id="#top--second-to-top--top">Top</h3>
<h2 id="#top--top">Top</h2>
```

### Links

+ links follow CommonMark markdown format, with some modifications
+ `[[linkText]]` used for special links backed by a provided index (e.g. provided by the application runnnig the markdown)

### Lists

Lists begin with either a `+` (plus sign; unordered list) or a number and a period (e.g. `1.`; ordered list).

```markdown
+ This is an unordered list item

1. This is the first
2. part of an ordered list
```

The previous example will produce the following HTML code:

```html
<ul>
    <li>This is an unordered list item</li>
</ul>
<ol>
    <li>This is the first</li>
    <li>part of an ordered list</li>
</ol>
```

### Paragraphs

Paragraphs are separated by line breaks and can contain [Rich Text](#rich-text).

```markdown
This is an example paragraph. This sentence has no rich formatting. *This sentence is in italics*. **This one is bold**^and there's superscript too!^. ~~Strikethrough~~ can be used to add a line through text, while __subscript__ can be used for various notations. 
```

The previous example will produce the following HTML:

```html
<p>This is an example paragraph. This sentence has no rich fromatting. <i>This sentence is in italics</i>. <b>This sentence is in bold</b><sup>and there's superscript too!</sup>. <del>Strikethrough</del> can be used to add a line through text, while <sub>subscript</sub> can be used for various notations.</p>
```

### Rich Text

Note that the characters specified with the examples are the *only* valid way to produce that rich text option.

+ strikethrough (`~~{text}~~`)
+ bold (`**{text}**`)
+ italics (`*{text}*`)
+ subscript (`__{text}__`)
+ superscript (`^{}^`)

Rich text can be present in lists, paragraphs, and tables.

### Summary and Details

+ looks similar to fenced code blocks, uses tildes (`~`) instead of backticks (`\``)
  + summary text goes in place of code language
+ problem: how to link (url hashes) to hidden headings?
  + should links be added at all?

```deimos
~~~This is the summary text
This is the hidden details content.

## You can also nest headings and other formatting
~~~
```

```html
<details>
  <summary>This is the summary text</summary>
  <p>This is the hidden details content.</p>
  <h2>You can also nest headings and other formatting</h2>
</details>
```

### Tables

+ placed in code block with language table
  + todo: remove?
+ headers are comma-separated and placed on first line
+ cells are comma-separated and take up subsequent lines
+ no alignment options

```table
\`\`\`table
header,header,header
cell,cell,cell
cell,cell,cell
\`\`\`
```

### Warnings and Alerts

### Spaced Repetition

## Quirks and Idioms

## Example Documents

```deimos
~title Document
~tags  meta,guides
# This is a document
```

## Prior Art

Influences on Deimos:

+ [Observable - Make sense of the world with data, together / Observable](https://observablehq.com/)

![Screenshot of the Haiku MarkDown variant](https://user-images.githubusercontent.com/1895289/116659117-ed0fb800-a945-11eb-9e97-c28eeaf29ab0.png)

> from the [Haiku language reference](https://github.com/osmoscraft/osmosnote/blob/master/docs/haiku-language-reference.md)

## See Also

+ [Chart.js | Chart.js](https://www.chartjs.org/docs/latest/) - planning for chart/graphic syntax


