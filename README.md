<div>
<div align="end">
    <h1>Nostalgia</h1>
</div>
Customizable markdown, generate your site in an instant.
</div>

### Concept

Nostalgia was born from the lack of customizable site generation tools. 

API docs, landing pages and overall sites are usually a pain to maintain standardized, usually those are separated on different projects, that need to be maintained separately.

Nostalgia is here to join everything together in a single place, while keeping full customization and control to the user.

With Nostalgia, you write markdown, and it magically generates you a full working site, you can save your layouts to standardize your documentations and overall look of your sites.

### Design

```mermaid
flowchart TD
    page["page.md"] --> parse[parse]
    parse --> ast["AST<br/>(spans, directives attached)"]
    comp["component .md"] --> expand
    ast --> expand["expand [calls]<br/>with arguments"]
    expand --> site["site-wide structure<br/>(pages, URLs, nav)"]
    site --> html[emit HTML]
    html --> tw["run tailwind<br/>(shell out)"]
```